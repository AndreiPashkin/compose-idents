use crate::ast::AliasSpec;
use crate::core::{Environment, Type};
use crate::error::Error;
use crate::resolve::{Resolve, Scope};

impl Resolve for AliasSpec {
    /// Resolves [`AliasSpec`] by delegating the resolution process further down to each of the
    /// items it contains.
    fn resolve(
        &self,
        environment: &Environment,
        scope: &mut Scope,
        expected_type: Option<&Type>,
    ) -> Result<(), Error> {
        for item in self.items() {
            item.resolve(environment, scope, expected_type)?;
        }
        Ok(())
    }
}
