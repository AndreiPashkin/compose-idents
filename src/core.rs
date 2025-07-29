//! Defines core types used throughout the project.

use crate::util::unique_id::next_unique_id;

/// State of a particular macro invocation.
///
/// Contains data useful for internal components and used within the scope of a single macro
/// invocation.
#[derive(Debug)]
pub struct State {
    /// Random seed.
    seed: u64,
}

impl State {
    /// Creates a new State with the given `seed`.
    pub fn new() -> Self {
        Self {
            seed: next_unique_id(),
        }
    }

    /// Reads the seed value.
    #[inline]
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
