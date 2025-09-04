//! Implements the deprecation mechanism.

use std::cell::OnceCell;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::Rc;
use syn::visit_mut::VisitMut;
use syn::{
    parse_quote, visit_mut, Attribute, Block, Field, File, ForeignItem, Item, TraitItem, Variant,
};

thread_local! {
    static GLOBAL_DEPRECATION_SERVICE: OnceCell<Rc<RefCell<DeprecationService>>> = const { OnceCell::new() };
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

    pub(crate) fn with_prefix(&self, prefix: &str) -> Self {
        let DeprecationWarning { note, since } = self;
        DeprecationWarning {
            note: format!("{}{}", prefix, note),
            since: since.clone(),
        }
    }

    pub(crate) fn to_attribute(&self) -> Attribute {
        let DeprecationWarning { note, since } = self;
        parse_quote! {
            #[deprecated(
                since=#since,
                note=#note,
            )]
        }
    }
}

/// Processes the code block tries to add deprecations to existing syntactic elements.
pub struct DeprecationWarningVisitor {
    warnings: Vec<DeprecationWarning>,
    prefix: String,
}

impl DeprecationWarningVisitor {
    pub fn new(mut warnings: Vec<DeprecationWarning>, prefix: String) -> Self {
        warnings.reverse();

        Self { warnings, prefix }
    }

    /// Try to place the deprecation attribute into the given attribute list.
    fn process_deprecations(&mut self, attrs: &mut Vec<Attribute>) {
        if self.warnings.is_empty() {
            return;
        }
        for attr in attrs.iter() {
            if attr.path().is_ident("deprecated") {
                return;
            }
        }
        let warning = &self.warnings.pop().unwrap();
        let attr = warning.with_prefix(&self.prefix).to_attribute();
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

/// A utility for collecting and publishing deprecation warnings during macro expansion.
///
/// It attaches `#[deprecated(...)]` attributes to the generated code (when it's possible) -
/// this way it makes it not necessary to create additional syntactic elements and pollute
/// the generated code.
///
/// ## Usage
///
/// The main usage pattern is through a thread-local singleton accessed via scoped handles
/// [`DeprecationServiceScope`], which serves as a facade for places in the code where the
/// service instance can't be passed normally through arguments. The service must be
/// initialized with a prefix at the macro entrypoint before use by setting the global:
///
/// ```rust,ignore
/// let service = DeprecationService::new_rc("compose_idents!: ");
/// DeprecationService::maybe_set_global(service);
/// let scope = DeprecationService::scoped();
/// scope.add_semicolon_separator_warning();
/// scope.emit(&mut generated_block);
/// ```
pub struct DeprecationService {
    warnings: BTreeSet<DeprecationWarning>,
    borrowed: usize,
    prefix: String,
}

impl DeprecationService {
    pub fn new(prefix: impl Into<String>) -> Self {
        Self {
            warnings: BTreeSet::new(),
            borrowed: 0,
            prefix: prefix.into(),
        }
    }

    pub fn new_rc(prefix: impl Into<String>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self::new(prefix)))
    }

    pub fn add_warning(&mut self, warning: DeprecationWarning) {
        self.warnings.insert(warning);
    }

    pub fn make_semicolon_separator_warning() -> DeprecationWarning {
        DeprecationWarning::new(
            "Using semicolons as separators is deprecated, use commas instead".to_string(),
            "0.0.5".to_string(),
        )
    }

    pub fn add_semicolon_separator_warning(&mut self) {
        self.add_warning(Self::make_semicolon_separator_warning());
    }

    pub fn clear(&mut self) {
        self.warnings.clear();
    }

    pub fn emit(&self, block: &mut Block) {
        if self.warnings.is_empty() {
            return;
        }
        let mut deprecation_visitor = DeprecationWarningVisitor::new(
            self.warnings.iter().cloned().collect(),
            self.prefix.clone(),
        );
        deprecation_visitor.visit_block_mut(block);
    }

    pub fn maybe_set_global(service: Rc<RefCell<DeprecationService>>) {
        GLOBAL_DEPRECATION_SERVICE.with(|cell| {
            let _ = cell.set(service);
        });
    }

    pub fn get_global() -> Option<Rc<RefCell<DeprecationService>>> {
        GLOBAL_DEPRECATION_SERVICE.with(|cell| cell.get().cloned())
    }

    pub fn scoped() -> DeprecationServiceScope {
        let service = Self::get_global()
            .expect("DeprecationService is not initialized. Call maybe_set_global() first");
        service.borrow_mut().borrowed += 1;
        DeprecationServiceScope {}
    }
}

pub struct DeprecationServiceScope;

impl DeprecationServiceScope {
    pub fn add_semicolon_separator_warning(&self) {
        if let Some(service) = DeprecationService::get_global() {
            service.borrow_mut().add_semicolon_separator_warning();
        }
    }

    pub fn emit(&self, block: &mut Block) {
        if let Some(service) = DeprecationService::get_global() {
            service.borrow().emit(block);
        }
    }
}

impl Drop for DeprecationServiceScope {
    fn drop(&mut self) {
        if let Some(service) = DeprecationService::get_global() {
            let mut service_ = service.borrow_mut();
            service_.borrowed -= 1;
            if service_.borrowed == 0 {
                service_.clear();
            }
        }
    }
}
