use crate::ast::AliasSpecItem;
use crate::error::Error;
use crate::resolve::{Resolve, Scope};

impl Resolve for AliasSpecItem {
    /// Resolves an [`AliasSpecItem`] by adding its alias to the global scope and checking for
    /// redefinition of aliases.
    fn resolve(&self, scope: &mut Scope) -> Result<(), Error> {
        let name = self.alias().ident().to_string();
        self.value().expr().resolve(scope)?;
        scope.try_add_name(name, self.value())?;
        Ok(())
    }
}
