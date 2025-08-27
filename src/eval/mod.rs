//! Implementation of eval-phase logic.
//!
//! Eval-phase is the final phase of the interpreter's execution cycle. It is responsible for
//! producing final values of expressions.
#![allow(unused_imports)]

mod core;
pub use core::*;

mod value;
pub use value::*;

mod call;
pub use call::*;

mod expr;
pub use expr::*;

mod alias_value;
pub use alias_value::*;

mod alias_spec;
pub use alias_spec::*;

mod alias_spec_item;
pub use alias_spec_item::*;

mod compose_idents_args;
pub use compose_idents_args::*;
