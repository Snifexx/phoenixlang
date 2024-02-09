use std::{rc::Rc, collections::HashMap, hash::BuildHasherDefault};
use ahash::AHasher;

use crate::error::PhoenixError;

use self::{module::Module, scanner::Scanner, chunk::Chunk};

pub mod chunk;
pub mod scanner;
mod token;
pub mod module;

type AHashMap<K, V> = HashMap<K, V, BuildHasherDefault<AHasher>>;

pub struct Compiler {
    modules: AHashMap<Rc<String>, Module>,
}

impl Compiler {
    pub fn new() -> Self { Self { modules: AHashMap::default() }}

    pub fn compile_string(str: String) -> Result<Chunk, Vec<PhoenixError>> {
        let tokens = Scanner::new(str).scan();
        if tokens.is_err() { return Err(vec![tokens.unwrap_err()]) }
        Module::new(tokens.unwrap(), Rc::new(String::from("test.str"))).compile()
    }

}

