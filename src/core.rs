/// Defines core types used throughout the project.
use crate::eval::Eval;
use crate::unique_id::next_unique_id;
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
    seed: u64,
}

impl State {
    /// Creates a new State with the given `seed`.
    pub fn new() -> Self {
        Self {
            seed: next_unique_id(),
        }
    }

    /// Reads the seed value.
    #[inline]
    pub fn seed(&self) -> u64 {
        self.seed
    }
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
    value: String,
}

impl Arg {
    /// Creates a new [`Arg`] with the given `value`.
    pub fn new(value: String) -> Self {
        Self { value }
    }

    /// Reads arg's value.
    #[inline]
    pub fn value(&self) -> &str {
        &self.value
    }
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
pub enum Expr {
    ArgExpr(Box<Arg>),
    FuncCallExpr(Box<Func>),
}

/// A single alias specification.
pub struct AliasSpecItem {
    alias: Ident,
    exprs: Vec<Expr>,
}

impl AliasSpecItem {
    /// Creates a new [`AliasSpecItem`] with the given alias and expressions.
    pub fn new(alias: Ident, exprs: Vec<Expr>) -> Self {
        Self { alias, exprs }
    }

    /// Reads the alias identifier.
    #[inline]
    pub fn alias(&self) -> &Ident {
        &self.alias
    }

    pub fn replacement(&self, state: &State, arg_replacements: &HashMap<String, String>) -> Ident {
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

/// Specification of aliases provided to the [`compose_idents`] macro by the user.
pub struct AliasSpec {
    items: Vec<AliasSpecItem>,
    is_comma_used: Option<bool>,
}

impl AliasSpec {
    /// Creates a new [`AliasSpec`] with the given items and separator information.
    pub fn new(items: Vec<AliasSpecItem>, is_comma_used: Option<bool>) -> Self {
        Self {
            items,
            is_comma_used,
        }
    }

    /// Whether a comma is used as a separator.
    #[inline]
    pub fn is_comma_used(&self) -> Option<bool> {
        self.is_comma_used
    }

    /// Replacements that correspond to the aliases.
    pub fn replacements(&self, state: &State) -> HashMap<Ident, Ident> {
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

/// Deprecation warning - could be used to warn user about usage of deprecated functionality while
/// still preserving backwards-compatibility.
#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct DeprecationWarning {
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
pub struct ComposeIdentsArgs {
    spec: AliasSpec,
    block: Block,
    deprecation_warnings: BTreeSet<DeprecationWarning>,
}

/// Arguments to the [`compose_idents`] macro.
impl ComposeIdentsArgs {
    /// Creates new ComposeIdentsArgs with the given components.
    pub fn new(
        spec: AliasSpec,
        block: Block,
        deprecation_warnings: BTreeSet<DeprecationWarning>,
    ) -> Self {
        Self {
            spec,
            block,
            deprecation_warnings,
        }
    }

    /// Reads the alias specification.
    #[inline]
    pub fn spec(&self) -> &AliasSpec {
        &self.spec
    }

    /// Reads a mutable reference to the code block.
    #[inline]
    pub fn block_mut(&mut self) -> &mut Block {
        &mut self.block
    }

    /// Reads the deprecation warnings.
    #[inline]
    pub fn deprecation_warnings(&self) -> &BTreeSet<DeprecationWarning> {
        &self.deprecation_warnings
    }
}

/// Visitor that replaces aliases in the provided code block with their definitions.
pub struct ComposeIdentsVisitor {
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
pub struct DeprecationWarningVisitor {
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
