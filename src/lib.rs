#![allow(warnings)]

use compiler::chunk::Chunk;
use debug::debug_chunk;
use flamebytecode::FBOpCode;
use compiler::chunk::Const;

use crate::flamebytecode::FBOpCode::*;

mod flamebytecode;
mod vm;
mod compiler;
mod debug;
mod utils;
pub mod error;

const IDENTIFIER_MAX_LENGTH: usize = 32;

//#[cfg(test)]
mod test {
    use std::{rc::Rc, mem, collections::HashSet, str::FromStr, path::PathBuf};


    use clap::error::ErrorKind;
    use toml::Table;

    use crate::{compiler::{scanner::Scanner, module::Module, Compiler}, debug::debug_chunk, error::PhoenixError, vm::{Vm, value::Value}};

    #[test]
    pub fn test() -> Result<(), Vec<PhoenixError>> {

        let chunk = Compiler::compile(PathBuf::from(r"/home/matteo/rust/phoenixlang/test/"))?;
        debug_chunk(&chunk);
        let vm = Vm { chunk, pc: 0, stack: vec![], str_intern: HashSet::default(), globals: Default::default() }.run(false);
        Ok(())
    }

    #[test]
    pub fn test_general() {
    }
}


pub fn execute(bytes: Vec<u8>) {

}

fn compile(str: String) {

}
