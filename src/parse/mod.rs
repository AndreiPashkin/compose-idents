//! Implements the parsing-phase logic
//!
//! It relies on [`syn::Parse`] and `syn` crate in general to do all the heavy-lifting.
#![allow(unused_imports)]

mod core;
pub use core::*;

mod value;
pub use value::*;

mod call;
pub use call::*;

mod expr;
pub use expr::*;

mod raw_ast;
pub use raw_ast::*;

mod alias;
pub use alias::*;

mod alias_value;
pub use alias_value::*;

mod alias_spec_item;
pub use alias_spec_item::*;

mod alias_spec;

pub use alias_spec::*;

mod type_;
pub use type_::*;

mod tuple;
pub use tuple::*;

mod loop_alias;
pub use loop_alias::*;

mod loop_source_value_list;
pub use loop_source_value_list::*;

mod loop_spec_item;
pub use loop_spec_item::*;

mod loop_spec;

pub use loop_spec::*;

mod compose_item_spec;
pub use compose_item_spec::*;
