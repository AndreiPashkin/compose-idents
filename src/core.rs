/// Defines core types used throughout the project.
use crate::eval::Eval;
use quote::format_ident;
use std::collections::{BTreeSet, HashMap};
use syn::visit_mut::VisitMut;
use syn::{
    parse_quote, visit_mut, Attribute, Block, Field, File, ForeignItem, Ident, Item, LitStr,
    TraitItem, Variant,
};

/// State of a particular macro invocation.
///
/// Contains data useful for internal components and used within the scope of a single macro
/// invocation.
#[derive(Debug)]
pub struct State {
    /// Random seed.
    pub(super) seed: u64,
}

/// Argument to the [`compose_idents`] macro.
///
/// Its [`Parse`] impl parses the input entirely, until the end.
///
/// Accepted inputs:
/// - Literal strings (enclosed in double quotes) are recognized and their content is used.
/// - Identifiers, literal numbers, underscores are used as is.
/// - Arbitrary sequences of tokens that do not include `,`.
#[derive(Debug, Clone)]
pub struct Arg {
    pub(super) value: String,
}

/// Function call in form of `upper(arg)` or `lower(arg)`, etc.
#[derive(Debug, Clone)]
pub enum Func {
    Upper(Expr),
    Lower(Expr),
    SnakeCase(Expr),
    CamelCase(Expr),
    PascalCase(Expr),
    Hash(Expr),
    Normalize(Expr),
    SignatureMismatch(String),
    Undefined,
}

/// Expression in form of an argument or a function call.
///
/// Just like [`Arg`] - parses the input entirely, until the end.
#[derive(Debug, Clone)]
pub(super) enum Expr {
    ArgExpr(Box<Arg>),
    FuncCallExpr(Box<Func>),
}

/// A value of a form `(foo, (bar, baz))`.
pub(super) struct Tuple<V> {
    values: Vec<TupleValue<V>>,
}

impl<V> Tuple<V> {
    /// Creates a new tuple.
    pub fn new(values: Vec<TupleValue<V>>) -> Self {
        Self { values }
    }

    pub fn values(&self) -> &[TupleValue<V>] {
        &self.values
    }
}

/// Allowed values in a tuple.
pub(super) enum TupleValue<V> {
    Tuple(Tuple<V>),
    Value(V),
}

/// Alias produced by a loop.
pub(super) enum LoopAlias {
    Simple(Ident),
    Tuple(Tuple<Ident>),
}

/// Source value of a loop.
pub(super) enum LoopSourceValue {
    Expr(Expr),
    Tuple(Tuple<Expr>),
}

/// A single loop specification.
pub(super) struct LoopSpec {
    var: LoopAlias,
    source: Vec<LoopSourceValue>,
}

/// A single alias specification.
pub(super) struct AliasSpecItem {
    pub(super) alias: Ident,
    pub(super) exprs: Vec<Expr>,
}

/// Specification of aliases provided to the [`compose_idents`] macro by the user.
pub(super) struct AliasSpec {
    pub(super) items: Vec<AliasSpecItem>,
    pub(super) is_comma_used: Option<bool>,
}

impl AliasSpecItem {
    pub(super) fn replacement(
        &self,
        state: &State,
        arg_replacements: &HashMap<String, String>,
    ) -> Ident {
        let ident = self.exprs.iter().fold("".to_string(), |acc, item| {
            let arg = item.eval(state);
            let replacement = arg_replacements.get(&arg);
            let arg = match replacement {
                Some(arg) => arg,
                None => &arg,
            };
            format!("{}{}", acc, arg)
        });
        format_ident!("{}", ident)
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(super) struct DeprecationWarning {
    note: String,
    since: String,
}

impl DeprecationWarning {
    pub fn new(note: String, since: String) -> Self {
        Self { note, since }
    }

    fn with_prefix(&self, prefix: &str) -> Self {
        let DeprecationWarning { note, since } = self;
        DeprecationWarning {
            note: format!("{}{}", prefix, note),
            since: since.clone(),
        }
    }

    fn to_attribute(&self) -> Attribute {
        let DeprecationWarning { note, since } = self;
        parse_quote! {
            #[deprecated(
                since=#since,
                note=#note,
            )]
        }
    }
}

/// Arguments to the [`compose_idents`] macro.
pub(super) struct ComposeIdentsArgs {
    pub(super) spec: AliasSpec,
    pub(super) block: Block,
    pub(super) deprecation_warnings: BTreeSet<DeprecationWarning>,
}

impl AliasSpec {
    pub(super) fn replacements(&self, state: &State) -> HashMap<Ident, Ident> {
        let mut arg_replacements = HashMap::new();
        self.items
            .iter()
            .map(|item| {
                let replacement = item.replacement(state, &arg_replacements);
                arg_replacements.insert(format!("{}", item.alias), format!("{}", replacement));

                (item.alias.clone(), replacement)
            })
            .collect()
    }
}

/// Visitor that replaces aliases in the provided code block with their definitions.
pub(super) struct ComposeIdentsVisitor {
    replacements: HashMap<Ident, Ident>,
}

impl ComposeIdentsVisitor {
    pub fn new(replacements: HashMap<Ident, Ident>) -> Self {
        Self { replacements }
    }
}

impl VisitMut for ComposeIdentsVisitor {
    fn visit_ident_mut(&mut self, ident: &mut Ident) {
        if let Some(replacement) = self.replacements.get(ident) {
            *ident = replacement.clone();
        }
    }

    fn visit_lit_str_mut(&mut self, i: &mut LitStr) {
        let value = i.value();
        let mut formatted = i.value().clone();

        for (alias, repl) in self.replacements.iter() {
            formatted = formatted.replace(&format!("%{}%", alias), &repl.to_string());
        }

        if formatted != value {
            *i = LitStr::new(&formatted, i.span());
        }
    }
}

/// Processes the code block tries to add deprecations to existing syntactic elements.
pub(super) struct DeprecationWarningVisitor {
    deprecation_warnings: BTreeSet<DeprecationWarning>,
    warning_prefix: String,
}

impl DeprecationWarningVisitor {
    pub fn new(deprecation_warnings: BTreeSet<DeprecationWarning>, warning_prefix: String) -> Self {
        Self {
            deprecation_warnings,
            warning_prefix,
        }
    }

    /// Try to place the deprecation attribute into the given attribute list.
    fn process_deprecations(&mut self, attrs: &mut Vec<Attribute>) {
        if self.deprecation_warnings.is_empty() {
            return;
        }
        for attr in attrs.iter() {
            if attr.path().is_ident("deprecated") {
                return;
            }
        }
        let warning = &self.deprecation_warnings.pop_first().unwrap();
        let attr = warning.with_prefix(&self.warning_prefix).to_attribute();
        attrs.push(attr);
    }
}

impl VisitMut for DeprecationWarningVisitor {
    fn visit_field_mut(&mut self, node: &mut Field) {
        self.process_deprecations(&mut node.attrs);
        visit_mut::visit_field_mut(self, node);
    }

    fn visit_file_mut(&mut self, node: &mut File) {
        self.process_deprecations(&mut node.attrs);
        visit_mut::visit_file_mut(self, node);
    }

    fn visit_foreign_item_mut(&mut self, node: &mut ForeignItem) {
        use ForeignItem::*;

        match node {
            Fn(item) => self.process_deprecations(&mut item.attrs),
            Static(item) => self.process_deprecations(&mut item.attrs),
            Type(item) => self.process_deprecations(&mut item.attrs),
            Macro(item) => self.process_deprecations(&mut item.attrs),
            _ => {}
        }
        visit_mut::visit_foreign_item_mut(self, node);
    }

    fn visit_item_mut(&mut self, node: &mut Item) {
        use Item::*;

        match node {
            Const(item) => self.process_deprecations(&mut item.attrs),
            Enum(item) => self.process_deprecations(&mut item.attrs),
            ExternCrate(item) => self.process_deprecations(&mut item.attrs),
            Fn(item) => self.process_deprecations(&mut item.attrs),
            ForeignMod(item) => self.process_deprecations(&mut item.attrs),
            Impl(item) => self.process_deprecations(&mut item.attrs),
            Macro(item) => self.process_deprecations(&mut item.attrs),
            Mod(item) => self.process_deprecations(&mut item.attrs),
            Static(item) => self.process_deprecations(&mut item.attrs),
            Struct(item) => self.process_deprecations(&mut item.attrs),
            Trait(item) => self.process_deprecations(&mut item.attrs),
            TraitAlias(item) => self.process_deprecations(&mut item.attrs),
            Type(item) => self.process_deprecations(&mut item.attrs),
            Union(item) => self.process_deprecations(&mut item.attrs),
            Use(item) => self.process_deprecations(&mut item.attrs),
            _ => {}
        }
        visit_mut::visit_item_mut(self, node);
    }

    fn visit_trait_item_mut(&mut self, node: &mut TraitItem) {
        use TraitItem::*;

        match node {
            Const(item) => self.process_deprecations(&mut item.attrs),
            Fn(item) => self.process_deprecations(&mut item.attrs),
            Type(item) => self.process_deprecations(&mut item.attrs),
            Macro(item) => self.process_deprecations(&mut item.attrs),
            _ => {}
        }
        visit_mut::visit_trait_item_mut(self, node);
    }

    fn visit_variant_mut(&mut self, node: &mut Variant) {
        self.process_deprecations(&mut node.attrs);
        visit_mut::visit_variant_mut(self, node);
    }
}
