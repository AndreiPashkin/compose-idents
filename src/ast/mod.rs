//! Defines the AST nodes that describe the syntax of the macro.
//!
//! The convention is that AST elements are themselves immutable and their mutable metadata is
//! stored in a separate [`AstMetadata`] structure:
//!
//!   - Each AST element has a unique ID (of type [`NodeId`]).
//!   - [`AstMetadata`] allows to reference metadata by the ID of the related AST-elements.

mod core;
pub use core::*;

mod compose_idents_args;
pub use compose_idents_args::*;

mod alias_spec;
pub use alias_spec::*;

mod alias_spec_item;
pub use alias_spec_item::*;

mod alias;
pub use alias::*;

mod alias_value;
pub use alias_value::*;

mod expr;
pub use expr::*;

mod arg;
pub use arg::*;

mod func;
pub use func::*;
