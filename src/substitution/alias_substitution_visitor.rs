//! Provides [`AliasSubstitutionVisitor`] - the top level component responsible for alias
//! substitution in the provided code block.

use crate::ast::Value;
use crate::error::Error;
use crate::substitution::substitute_idents;
use crate::util::log::debug;
use proc_macro2::TokenStream;
use quote::ToTokens;
use std::collections::HashMap;
use std::rc::Rc;
use syn::parse::Parse;
use syn::visit_mut::VisitMut;
use syn::{
    parse_quote, Block, Fields, ImplItem, ImplItemConst, ImplItemFn, ImplItemMacro, ImplItemType,
    Item, ItemEnum, ItemFn, ItemForeignMod, ItemImpl, ItemMod, ItemStruct, ItemTrait, ItemUnion,
    Stmt, TraitItem, TraitItemConst, TraitItemFn, TraitItemMacro, TraitItemType,
};

struct OuterAttributes(Vec<syn::Attribute>);

impl From<OuterAttributes> for Vec<syn::Attribute> {
    fn from(value: OuterAttributes) -> Self {
        value.0
    }
}

impl Parse for OuterAttributes {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let attrs = input.call(syn::Attribute::parse_outer)?;
        Ok(OuterAttributes(attrs))
    }
}

impl ToTokens for OuterAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for attr in &self.0 {
            attr.to_tokens(tokens);
        }
    }
}

macro_rules! check_error {
    ($self:expr) => {
        if $self.error.is_some() {
            return;
        }
    };
}

/// Visitor that replaces aliases in the provided code block with their definitions.
///
/// Recursively and incrementally operates on AST-level.
pub struct AliasSubstitutionVisitor {
    substitutions: HashMap<String, Rc<Value>>,
    error: Option<Error>,
}

impl AliasSubstitutionVisitor {
    pub fn new(substitutions: HashMap<String, Rc<Value>>) -> Self {
        Self {
            substitutions,
            error: None,
        }
    }

    /// An error occurred during the substitution process.
    pub fn error(&self) -> Option<&Error> {
        self.error.as_ref()
    }
    /// Generic AST-node visitor-method that can process any node non-recursively.
    fn visit_mut<N: ToTokens + Parse>(&mut self, node: &mut N) {
        debug!("Visiting generic AST node: {:?}", node.to_token_stream());
        check_error!(self);
        *node = match substitute_idents(&*node, &self.substitutions) {
            Ok(n) => n,
            Err(err) => {
                self.error = Some(err);
                return;
            }
        };
    }
    /// Similar to [`visit_mut`], but processes boxed AST-nodes.
    fn visit_boxed_mut<N: ToTokens + Parse>(&mut self, node: &mut Box<N>) {
        debug!(
            "Visiting boxed generic AST node: {:?}",
            node.to_token_stream()
        );
        check_error!(self);
        match substitute_idents::<N>(&*node, &self.substitutions) {
            Ok(n) => {
                *node = Box::new(n);
            }
            Err(err) => {
                self.error = Some(err);
            }
        }
    }
    /// Non-recursively visits an attributes-vector.
    fn visit_attrs_mut(&mut self, attrs: &mut Vec<syn::Attribute>) {
        debug!("Visiting attributes: {:?}", attrs);
        check_error!(self);
        *attrs = match substitute_idents(&OuterAttributes(attrs.clone()), &self.substitutions) {
            Ok(attrs) => attrs.into(),
            Err(err) => {
                self.error = Some(err);
                return;
            }
        };
    }
    /// Recursively visits fields in structs and enums.
    fn visit_fields_mut(&mut self, fields: &mut Fields) {
        debug!("Visiting fields: {:?}", fields);
        check_error!(self);
        match fields {
            Fields::Named(fields_named) => {
                for field in fields_named.named.iter_mut() {
                    self.visit_field_mut(field);
                    check_error!(self);
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                for field in fields_unnamed.unnamed.iter_mut() {
                    self.visit_field_mut(field);
                    check_error!(self);
                }
            }
            Fields::Unit => {}
        };
    }
}

/// Implements recursive incremental substitution in the provided code block.
///
/// `visit_block_mut` is the main entry point of the visitor.
impl VisitMut for AliasSubstitutionVisitor {
    fn visit_block_mut(&mut self, item: &mut Block) {
        debug!("Visiting block: {:?}", item);
        let mut new_stmts = Vec::new();
        for stmt in item.stmts.iter_mut() {
            let new_stmts_ = match stmt {
                Stmt::Item(Item::Fn(item_fn)) => {
                    self.visit_item_fn_mut(item_fn);
                    vec![stmt.clone()]
                }
                Stmt::Item(Item::Struct(item_struct)) => {
                    self.visit_item_struct_mut(item_struct);
                    vec![stmt.clone()]
                }
                Stmt::Item(Item::Enum(item_enum)) => {
                    self.visit_item_enum_mut(item_enum);
                    vec![stmt.clone()]
                }
                Stmt::Item(Item::Union(item_union)) => {
                    self.visit_item_union_mut(item_union);
                    vec![stmt.clone()]
                }
                Stmt::Item(Item::Trait(item_trait)) => {
                    self.visit_item_trait_mut(item_trait);
                    vec![stmt.clone()]
                }
                Stmt::Item(Item::Impl(item_impl)) => {
                    self.visit_item_impl_mut(item_impl);
                    vec![stmt.clone()]
                }
                Stmt::Item(Item::Mod(item_mod)) => {
                    self.visit_item_mod_mut(item_mod);
                    vec![stmt.clone()]
                }
                Stmt::Item(Item::ForeignMod(item_foreign_mod)) => {
                    self.visit_item_foreign_mod_mut(item_foreign_mod);
                    vec![stmt.clone()]
                }
                // A workaround for the problem of `Stmt` being context-dependent AST type and
                // not supporting AST -> tokens -> AST conversion roundtrip.
                // It is solved by enclosing it within an auxiliary block and then extracting the
                // resulting statement(s).
                stmt => {
                    let mut block: Block = parse_quote!({
                        #stmt
                    });
                    debug!("Visiting generic statement within a block...");
                    self.visit_mut(&mut block);
                    block.stmts
                }
            };
            check_error!(self);
            new_stmts.extend(new_stmts_);
        }
        item.stmts = new_stmts;
    }
    fn visit_field_mut(&mut self, field: &mut syn::Field) {
        debug!("Visiting a field: {:?}", field);
        check_error!(self);
        self.visit_attrs_mut(&mut field.attrs);
        check_error!(self);
        debug!("Visiting field's visibility...");
        self.visit_mut(&mut field.vis);
        check_error!(self);
        if let Some(ident) = &mut field.ident {
            debug!("Visiting field's identifier...");
            self.visit_mut(ident);
            check_error!(self);
        }
        debug!("Visiting field's type...");
        self.visit_mut(&mut field.ty);
    }
    fn visit_impl_item_const_mut(&mut self, i: &mut ImplItemConst) {
        debug!("Visiting an associated const within an impl block: {:?}", i);
        check_error!(self);
        self.visit_mut(i);
    }
    fn visit_impl_item_fn_mut(&mut self, i: &mut ImplItemFn) {
        debug!("Visiting an associated fn within an impl block: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting an associated fn's within an impl block visibility...");
        self.visit_mut(&mut i.vis);
        check_error!(self);
        debug!("Visiting an associated fn's within an impl block signature...");
        self.visit_mut(&mut i.sig);
        check_error!(self);
        self.visit_block_mut(&mut i.block);
    }
    fn visit_impl_item_macro_mut(&mut self, i: &mut ImplItemMacro) {
        debug!("Visiting a macro invocation within an impl block: {:?}", i);
        check_error!(self);
        self.visit_mut(i);
    }
    fn visit_impl_item_type_mut(&mut self, i: &mut ImplItemType) {
        debug!("Visiting an associated type within an impl block: {:?}", i);
        check_error!(self);
        self.visit_mut(i);
    }
    fn visit_item_enum_mut(&mut self, i: &mut ItemEnum) {
        debug!("Visiting an enum: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting an enum's visibility...");
        self.visit_mut(&mut i.vis);
        check_error!(self);
        debug!("Visiting an enum's identifier...");
        self.visit_mut(&mut i.ident);
        check_error!(self);
        debug!("Visiting an enum's generic parameters...");
        self.visit_mut(&mut i.generics);
        check_error!(self);

        debug!("Visiting an enum's variants...");
        for variant in i.variants.iter_mut() {
            self.visit_attrs_mut(&mut variant.attrs);
            debug!("Visiting an enum variant's identifier...");
            check_error!(self);
            self.visit_mut(&mut variant.ident);
            check_error!(self);
            self.visit_fields_mut(&mut variant.fields);
            check_error!(self);
            if let Some((_eq, expr)) = &mut variant.discriminant {
                debug!("Visiting an enum variant's discriminant...");
                self.visit_mut(expr);
                check_error!(self);
            }
        }
    }
    fn visit_item_fn_mut(&mut self, i: &mut ItemFn) {
        debug!("Visiting function: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting function's visibility...");
        self.visit_mut(&mut i.vis);
        check_error!(self);
        debug!("Visiting function's signature...");
        self.visit_mut(&mut i.sig);
        check_error!(self);
        self.visit_block_mut(i.block.as_mut());
    }
    fn visit_item_foreign_mod_mut(&mut self, i: &mut ItemForeignMod) {
        debug!("Visiting a foreign module: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        if let Some(abi) = &mut i.abi.name {
            debug!("Visiting a foreign module's ABI name...");
            self.visit_mut(abi);
            check_error!(self);
        }
        debug!("Visiting a foreign module's items...");
        for foreign_item in i.items.iter_mut() {
            debug!("Visiting a foreign module's item...");
            self.visit_mut(foreign_item);
            check_error!(self);
        }
    }

    fn visit_item_impl_mut(&mut self, i: &mut ItemImpl) {
        debug!("Visiting an impl block: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting an impl's generic parameters...");
        self.visit_mut(&mut i.generics);
        check_error!(self);
        if let Some((_bang, path, _for_token)) = &mut i.trait_ {
            debug!("Visiting an impl's trait path...");
            self.visit_mut(path);
            check_error!(self);
        }
        debug!("Visiting an impl's self type...");
        self.visit_boxed_mut(&mut i.self_ty);
        check_error!(self);

        debug!("Visiting an impl's items...");
        for item in i.items.iter_mut() {
            debug!("Visiting an impl's item...");
            match item {
                ImplItem::Fn(item_fn) => self.visit_impl_item_fn_mut(item_fn),
                ImplItem::Const(item_const) => self.visit_impl_item_const_mut(item_const),
                ImplItem::Type(item_type) => self.visit_impl_item_type_mut(item_type),
                ImplItem::Macro(item_macro) => self.visit_impl_item_macro_mut(item_macro),
                _ => self.visit_mut(item),
            }
            check_error!(self);
        }
    }
    fn visit_item_mod_mut(&mut self, i: &mut ItemMod) {
        debug!("Visiting a module: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting a module's visibility...");
        self.visit_mut(&mut i.vis);
        check_error!(self);
        debug!("Visiting a module's identifier...");
        self.visit_mut(&mut i.ident);
        check_error!(self);
        if let Some((_brace, items)) = &mut i.content {
            debug!("Visiting a module's items...");
            for item in items.iter_mut() {
                debug!("Visiting a module's item...");
                match item {
                    Item::Fn(f) => self.visit_item_fn_mut(f),
                    Item::Struct(s) => self.visit_item_struct_mut(s),
                    Item::Enum(e) => self.visit_item_enum_mut(e),
                    Item::Union(u) => self.visit_item_union_mut(u),
                    Item::Trait(t) => self.visit_item_trait_mut(t),
                    Item::Impl(im) => self.visit_item_impl_mut(im),
                    Item::Mod(m) => self.visit_item_mod_mut(m),
                    Item::ForeignMod(fm) => self.visit_item_foreign_mod_mut(fm),
                    other => {
                        debug!("Visiting a generic module item: {:?}", other);
                        self.visit_mut(other)
                    }
                }
                check_error!(self);
            }
        }
    }
    fn visit_item_struct_mut(&mut self, i: &mut ItemStruct) {
        debug!("Visiting a struct: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting a struct's visibility...");
        self.visit_mut(&mut i.vis);
        check_error!(self);
        debug!("Visiting a struct's identifier...");
        self.visit_mut(&mut i.ident);
        check_error!(self);
        debug!("Visiting a struct's generic parameters...");
        self.visit_mut(&mut i.generics);
        check_error!(self);
        debug!("Visiting a struct's fields...");
        self.visit_fields_mut(&mut i.fields);
    }
    fn visit_item_trait_mut(&mut self, i: &mut ItemTrait) {
        debug!("Visiting a trait: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting a trait's visibility...");
        self.visit_mut(&mut i.vis);
        check_error!(self);
        debug!("Visiting a trait's identifier...");
        self.visit_mut(&mut i.ident);
        check_error!(self);
        debug!("Visiting a trait's generic parameters...");
        self.visit_mut(&mut i.generics);
        check_error!(self);
        debug!("Visiting a trait's supertraits...");
        for bound in i.supertraits.iter_mut() {
            debug!("Visiting a trait's supertrait...");
            self.visit_mut(bound);
            check_error!(self);
        }
        debug!("Visiting a trait's items...");
        for item in i.items.iter_mut() {
            match item {
                TraitItem::Fn(item_fn) => self.visit_trait_item_fn_mut(item_fn),
                TraitItem::Const(item_const) => self.visit_trait_item_const_mut(item_const),
                TraitItem::Type(item_type) => self.visit_trait_item_type_mut(item_type),
                TraitItem::Macro(item_macro) => self.visit_trait_item_macro_mut(item_macro),
                _ => self.visit_mut(item),
            }
            check_error!(self);
        }
    }
    fn visit_item_union_mut(&mut self, i: &mut ItemUnion) {
        debug!("Visiting a union: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting a union's visibility...");
        self.visit_mut(&mut i.vis);
        check_error!(self);
        debug!("Visiting a union's identifier...");
        self.visit_mut(&mut i.ident);
        check_error!(self);
        debug!("Visiting a union's generic parameters...");
        self.visit_mut(&mut i.generics);
        check_error!(self);
        debug!("Visiting a union's fields...");
        for field in i.fields.named.iter_mut() {
            debug!("Visiting a union's field...");
            self.visit_field_mut(field);
            check_error!(self);
        }
    }
    fn visit_trait_item_const_mut(&mut self, i: &mut TraitItemConst) {
        debug!("Visiting an associated const within a trait: {:?}", i);
        check_error!(self);
        self.visit_mut(i);
    }
    fn visit_trait_item_fn_mut(&mut self, i: &mut TraitItemFn) {
        debug!("Visiting a trait function: {:?}", i);
        check_error!(self);
        self.visit_attrs_mut(&mut i.attrs);
        check_error!(self);
        debug!("Visiting a trait fn's signature...");
        self.visit_mut(&mut i.sig);
        check_error!(self);
        if let Some(block) = &mut i.default {
            debug!("Visiting a trait fn's default block...");
            self.visit_block_mut(block);
        }
    }
    fn visit_trait_item_macro_mut(&mut self, i: &mut TraitItemMacro) {
        debug!("Visiting a macro invocation within a trait: {:?}", i);
        check_error!(self);
        self.visit_mut(i);
    }
    fn visit_trait_item_type_mut(&mut self, i: &mut TraitItemType) {
        debug!("Visiting an associated type within a trait: {:?}", i);
        check_error!(self);
        self.visit_mut(i);
    }
}

#[cfg(test)]
mod tests {
    use super::super::test::make_substitutions;
    use super::*;
    use crate::ast::Value;
    use proc_macro2::{Ident, Span};
    use rstest::rstest;
    use std::collections::HashMap;
    use std::rc::Rc;
    use syn::parse_quote;

    /// Various basic token substitution cases.
    #[rstest]
    #[case::substituting_single_token(
        parse_quote!{{
            fn foo() -> u32 { 1 }
        }},
        parse_quote!{{
            fn bar() -> u32 { 1 }
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    #[case::substituting_multiple_tokens(
        parse_quote!{{
            let foo = 1;
            let bar = foo + 1;
        }},
        parse_quote!{{
            let baz = 1;
            let bar = baz + 1;
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("baz", Span::call_site())),
        ),
    )]
    #[case::substituting_with_multiple_tokens(
        parse_quote!{{
            fn foo() -> T { 1 }
        }},
        parse_quote!{{
            fn foo() -> Result<u32, String> { 1 }
        }},
        make_substitutions!(
            "T" => Value::from_type(syn::parse_str::<syn::Type>("Result<u32, String>").unwrap()),
        ),
    )]
    #[case::string_formatting(
        parse_quote!{{
            fn foo() -> &str { "Hello, % name %!" }
        }},
        parse_quote!{{
            fn foo() -> &str { "Hello, World!" }
        }},
        make_substitutions!(
            "name" => Value::from_ident(Ident::new("World", Span::call_site())),
        ),
    )]
    fn basic_substitution(
        #[case] mut input: Block,
        #[case] expected: Block,
        #[case] substitutions: HashMap<String, Rc<Value>>,
    ) {
        let mut visitor = AliasSubstitutionVisitor::new(substitutions);
        visitor.visit_block_mut(&mut input);
        assert!(
            visitor.error().is_none(),
            "Visitor error during substitution: {:?}",
            visitor.error(),
        );

        assert_eq!(input, expected);
    }

    /// Various cases that verify correctness of recursive AST traversal and substitution.
    #[rstest]
    // Function cases.
    #[case::fn_identifier_substitution(
        parse_quote!{{
            fn foo() -> u32 { 42 }
        }},
        parse_quote!{{
            fn bar() -> u32 { 42 }
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    #[case::fn_block_recursive_substitution(
        parse_quote!{{
            fn foo() -> u32 { bar }
        }},
        parse_quote!{{
            fn foo() -> u32 { baz }
        }},
        make_substitutions!(
            "bar" => Value::from_ident(Ident::new("baz", Span::call_site())),
        ),
    )]
    // Struct cases.
    #[case::struct_identifier_substitution(
        parse_quote!{{
            struct Foo { a: u32 }
        }},
        parse_quote!{{
            struct Bar { a: u32 }
        }},
        make_substitutions!(
            "Foo" => Value::from_ident(Ident::new("Bar", Span::call_site())),
        ),
    )]
    #[case::struct_fields_recursive_type_substitution(
        parse_quote!{{
            struct S { a: T }
        }},
        parse_quote!{{
            struct S { a: Result<u32, String> }
        }},
        make_substitutions!(
            "T" => Value::from_type(syn::parse_str::<syn::Type>("Result<u32, String>").unwrap()),
        ),
    )]
    #[case::struct_field_identifier_substitution(
        parse_quote!{{
            struct S { foo: u32 }
        }},
        parse_quote!{{
            struct S { bar: u32 }
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    // Enum cases.
    #[case::enum_identifier_substitution(
        parse_quote!{{
            enum Foo { A }
        }},
        parse_quote!{{
            enum Bar { A }
        }},
        make_substitutions!(
            "Foo" => Value::from_ident(Ident::new("Bar", Span::call_site())),
        ),
    )]
    #[case::enum_variant_identifier_substitution(
        parse_quote!{{
            enum E { Foo }
        }},
        parse_quote!{{
            enum E { Bar }
        }},
        make_substitutions!(
            "Foo" => Value::from_ident(Ident::new("Bar", Span::call_site())),
        ),
    )]
    #[case::enum_variant_discriminant_recursive_substitution(
        parse_quote!{{
            enum E { A = foo }
        }},
        parse_quote!{{
            enum E { A = bar }
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    // Union cases.
    #[case::union_identifier_substitution(
        parse_quote!{{
            union U { a: u32 }
        }},
        parse_quote!{{
            union V { a: u32 }
        }},
        make_substitutions!(
            "U" => Value::from_ident(Ident::new("V", Span::call_site())),
        ),
    )]
    #[case::union_field_type_recursive_substitution(
        parse_quote!{{
            union U { a: T }
        }},
        parse_quote!{{
            union U { a: Result<u32, String> }
        }},
        make_substitutions!(
            "T" => Value::from_type(syn::parse_str::<syn::Type>("Result<u32, String>").unwrap()),
        ),
    )]
    // Trait cases.
    #[case::trait_identifier_substitution(
        parse_quote!{{
            trait Foo {}
        }},
        parse_quote!{{
            trait Bar {}
        }},
        make_substitutions!(
            "Foo" => Value::from_ident(Ident::new("Bar", Span::call_site())),
        ),
    )]
    #[case::trait_supertrait_recursive_substitution(
        parse_quote!{{
            trait T: Foo {}
        }},
        parse_quote!{{
            trait T: Bar {}
        }},
        make_substitutions!(
            "Foo" => Value::from_ident(Ident::new("Bar", Span::call_site())),
        ),
    )]
    #[case::trait_fn_default_block_recursive_substitution(
        parse_quote!{{
            trait T {
                fn f() { foo }
            }
        }},
        parse_quote!{{
            trait T {
                fn f() { bar }
            }
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    // Impl block cases.
    #[case::impl_self_type_substitution(
        parse_quote!{{
            impl Foo { fn g() -> u32 { 1 } }
        }},
        parse_quote!{{
            impl Bar { fn g() -> u32 { 1 } }
        }},
        make_substitutions!(
            "Foo" => Value::from_ident(Ident::new("Bar", Span::call_site())),
        ),
    )]
    #[case::impl_trait_path_substitution(
        parse_quote!{{
            struct S;
            impl Foo for S {}
        }},
        parse_quote!{{
            struct S;
            impl Bar for S {}
        }},
        make_substitutions!(
            "Foo" => Value::from_ident(Ident::new("Bar", Span::call_site())),
        ),
    )]
    #[case::impl_associated_const_identifier_substitution(
        parse_quote!{{
            impl S { const FOO: u32 = 1; }
        }},
        parse_quote!{{
            impl S { const BAR: u32 = 1; }
        }},
        make_substitutions!(
            "FOO" => Value::from_ident(Ident::new("BAR", Span::call_site())),
        ),
    )]
    // Module cases.
    #[case::module_identifier_substitution(
        parse_quote!{{
            mod foo {}
        }},
        parse_quote!{{
            mod bar {}
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    #[case::module_recursive_item_substitution(
        parse_quote!{{
            mod m { fn foo() -> u32 { 1 } }
        }},
        parse_quote!{{
            mod m { fn bar() -> u32 { 1 } }
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    // Foreign module cases.
    #[case::foreign_mod_abi_string_formatting(
        parse_quote!{{
            extern "% abi %" {}
        }},
        parse_quote!{{
            extern "C" {}
        }},
        make_substitutions!(
            "abi" => Value::from_ident(Ident::new("C", Span::call_site())),
        ),
    )]
    #[case::foreign_mod_recursive_item_substitution(
        parse_quote!{{
            extern "C" { fn foo(); }
        }},
        parse_quote!{{
            extern "C" { fn bar(); }
        }},
        make_substitutions!(
            "foo" => Value::from_ident(Ident::new("bar", Span::call_site())),
        ),
    )]
    // Block cases.
    #[case::block_trailing_semicolon(
        parse_quote!{{
            fn foo() {
                bar();
                baz();
            }
        }},
        parse_quote!{{
            fn foo() {
                bar();
                qux();
            }
        }},
        make_substitutions!(
            "baz" => Value::from_ident(Ident::new("qux", Span::call_site())),
        ),
    )]
    #[case::block_no_trailing_semicolon(
        parse_quote!{{
            fn foo() {
                bar();
                baz()
            }
        }},
        parse_quote!{{
            fn foo() {
                bar();
                qux()
            }
        }},
        make_substitutions!(
            "baz" => Value::from_ident(Ident::new("qux", Span::call_site())),
        ),
    )]
    fn ast_recursive_substitution(
        #[case] mut input: Block,
        #[case] expected: Block,
        #[case] substitutions: HashMap<String, Rc<Value>>,
    ) {
        let mut visitor = AliasSubstitutionVisitor::new(substitutions);
        visitor.visit_block_mut(&mut input);
        assert!(
            visitor.error().is_none(),
            "Visitor error during substitution: {:?}",
            visitor.error(),
        );

        assert_eq!(input, expected);
    }
}
