//! Implements the parsing-phase logic
//!
//! It relies on [`syn::Parse`] and `syn` crate in general to do all the heavy-lifting.
#![allow(unused_imports)]

mod core;
pub use core::*;

mod arg;
pub use arg::*;

mod func;
pub use func::*;

mod expr;
pub use expr::*;

mod compose_idents_args;
pub use compose_idents_args::*;

mod alias;
pub use alias::*;

mod alias_value;
pub use alias_value::*;

mod alias_spec_item;
pub use alias_spec_item::*;

mod alias_spec;
pub use alias_spec::*;
