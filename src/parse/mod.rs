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

mod type_;
pub use type_::*;
