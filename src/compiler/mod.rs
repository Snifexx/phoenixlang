use std::rc::Rc;
use rustc_hash::{FxHashMap, FxHashSet};
use self::module::Module;

pub mod chunk;
mod scanner;
mod token;
mod module;

pub struct Compiler {
    interned_str: FxHashSet<Rc<String>>,
    modules: FxHashMap<Rc<String>, Module>,
}

impl Compiler {
    pub fn new() -> Self { Self { interned_str: FxHashSet::default(), modules: FxHashMap::default() }}

    pub fn compile_mod(&mut self, module: Rc<String>) {
        // TODO
    }
}

