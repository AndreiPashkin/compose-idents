use crate::ast::Ast;
use crate::error::Error;

/// A syntactic structure that can be expanded (desugared) into a more primitive form.
///
/// Serves as a backbone for the expand-phase.
pub trait Expand: Ast {
    type Expanded: Ast;

    fn expand(&self) -> Result<Self::Expanded, Error>;
}
