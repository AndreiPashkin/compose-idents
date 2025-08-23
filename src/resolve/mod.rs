//! Implementation of resolve-phase logic.
//!
//! Resolve-phase is supposed to be executed after the parse-phase and before the eval-phase. Its
//! general responsibility is to perform static analysis (which is pretty minimal at this point) and
//! binding function calls.
#![allow(unused_imports)]

mod core;
pub use core::*;

mod alias_spec;
pub use alias_spec::*;

mod alias_spec_item;
pub use alias_spec_item::*;

mod expr;
pub use expr::*;

mod call;
pub use call::*;

mod value;
pub use value::*;
