use crate::core::Func;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Global execution environment a particular macro invocation.
///
/// Contains data useful for internal components and exists within the scope of a single macro
/// invocation.
#[derive(Debug)]
pub struct Environment {
    /// Random seed.
    seed: u64,
    /// Available function-types.
    funcs: HashMap<String, Vec<Rc<Func>>>,
}

thread_local! {
    static GLOBAL_ENVIRONMENT: OnceCell<Rc<Environment>> = const { OnceCell::new() };
}

impl Environment {
    pub fn new(funcs: HashMap<String, Vec<Rc<Func>>>, seed: u64) -> Self {
        Self { funcs, seed }
    }

    pub fn new_initialized(seed: u64) -> Self {
        let funcs = Self::init_funcs();
        Self { funcs, seed }
    }

    /// Returns function variants for the function with given name.
    pub fn get_func_variants(&self, name: &str) -> Option<&[Rc<Func>]> {
        self.funcs.get(name).map(|funcs| funcs.as_slice())
    }

    pub fn has_func(&self, name: &str) -> bool {
        self.funcs.contains_key(name)
    }

    pub fn get_global() -> Option<Rc<Environment>> {
        GLOBAL_ENVIRONMENT.with(|cell| cell.get().cloned())
    }

    pub fn maybe_set_global(environment: Rc<Environment>) {
        GLOBAL_ENVIRONMENT.with(|cell| {
            let _ = cell.set(environment);
        });
    }

    /// Pretty-prints the signatures of all variants of the function with the given name.
    pub fn make_pretty_func_sig(&self, name: &str) -> Option<String> {
        self.funcs.get(name).map(|funcs| {
            funcs
                .iter()
                .map(|f| f.signature())
                .fold(String::new(), |acc, sig| {
                    if acc.is_empty() {
                        sig
                    } else {
                        format!("{} | {}", acc, sig)
                    }
                })
        })
    }
    pub fn seed(&self) -> u64 {
        self.seed
    }
}
