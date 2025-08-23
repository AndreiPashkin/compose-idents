//! Encapsulates the alias substitution and everything related to it.

mod format_string;
use format_string::*;
mod stream_visitor;
use stream_visitor::*;
mod substitute_idents;
use substitute_idents::*;
mod alias_substitution_visitor;

pub use alias_substitution_visitor::*;
#[cfg(test)]
mod test;
