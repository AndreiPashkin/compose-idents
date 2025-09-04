//! Defines the AST nodes that describe the syntax of the macro.
//!
//! The convention is that AST elements are themselves immutable and their mutable metadata is
//! stored in a separate [`AstMetadata`] structure:
//!
//!   - Each AST element has a unique ID (of type [`NodeId`]).
//!   - [`AstMetadata`] allows to reference metadata by the ID of the related AST-elements.

mod core;
pub use core::*;

mod raw_ast;
pub use raw_ast::*;

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

mod value;
pub use value::*;

mod call;
pub use call::*;

mod tuple;
pub use tuple::*;

mod loop_source_value_list;
pub use loop_source_value_list::*;

mod loop_spec_item;
pub use loop_spec_item::*;

mod loop_spec;
pub use loop_spec::*;

mod loop_alias;
pub use loop_alias::*;

mod expanded;
pub use expanded::*;

mod compose_item_spec;
pub use compose_item_spec::*;
