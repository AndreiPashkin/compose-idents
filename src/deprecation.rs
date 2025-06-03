use syn::{parse_quote, Attribute};

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
