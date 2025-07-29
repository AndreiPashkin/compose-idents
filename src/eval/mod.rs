//! Implementation of eval-phase logic.
//!
//! Eval-phase is the final phase of the interpreter's execution cycle. It is responsible for
//! producing final values of expressions.
#![allow(unused_imports)]

mod core;
pub use core::*;

mod arg;
pub use arg::*;

mod func;
pub use func::*;

mod expr;
pub use expr::*;

mod alias_value;
pub use alias_value::*;
