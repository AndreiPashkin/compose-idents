use crate::ast::{Ast, Value, ValueInner};
use crate::core::{Environment, Type};
use crate::error::Error;
use crate::resolve::{Resolve, Scope};
use crate::util::log::debug;

impl Resolve for Value {
    /// Resolves a function call by resolving its arguments and binding the call to a built-in
    /// function.
    fn resolve(
        &self,
        _: &Environment,
        scope: &mut Scope,
        expected_type: Option<&Type>,
    ) -> Result<(), Error> {
        let mut metadata = scope.metadata_mut();
        let from_type = match self.inner() {
            ValueInner::Ident(ident) => match scope.get_name(ident.to_string().as_str()) {
                Some(value) => {
                    let id = value.id();
                    if let Some(metadata) = metadata.get_value_metadata(id) {
                        metadata.target_type.clone()
                    } else if let Some(metadata) = metadata.get_call_metadata(id) {
                        metadata.target_type.clone()
                    } else {
                        panic!(
                            "Expected metadata to be set for a value {} of a resolved alias {}",
                            id, ident
                        );
                    }
                }
                _ => self.type_(),
            },
            _ => self.type_(),
        };

        let expected_type = match expected_type {
            Some(expected_type) => expected_type,
            None => &from_type,
        };

        match Type::coercion_cost(&from_type, expected_type) {
            Some(coercion_cost) => {
                metadata.set_value_metadata(self.id(), expected_type.clone(), coercion_cost);
                Ok(())
            }
            None => Err(Error::make_coercion_error(&self.type_(), expected_type)),
        }
    }
}
