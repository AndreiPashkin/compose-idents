use crate::ast::AliasSpecItem;
use crate::core::{Environment, Type};
use crate::error::Error;
use crate::resolve::{Resolve, Scope};

impl Resolve for AliasSpecItem {
    /// Resolves an [`AliasSpecItem`] by adding its alias to the global scope and checking for
    /// redefinition of aliases.
    fn resolve(
        &self,
        environment: &Environment,
        scope: &mut Scope,
        expected_type: Option<&Type>,
    ) -> Result<(), Error> {
        let name = self.alias().ident().to_string();
        self.value()
            .expr()
            .resolve(environment, scope, expected_type)?;
        scope.try_add_name(name, self.value().expr())?;
        Ok(())
    }
}
