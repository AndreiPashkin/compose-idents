use crate::ast::AliasSpec;
use crate::error::Error;
use crate::resolve::{Resolve, Scope};

impl Resolve for AliasSpec {
    /// Resolves [`AliasSpec`] by delegating the resolution process further down to each of the
    /// items it contains.
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error> {
        for item in self.items() {
            item.resolve(scope)?;
        }
        Ok(())
    }
}
