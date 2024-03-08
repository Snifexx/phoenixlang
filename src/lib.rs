#![allow(warnings)]

use compiler::chunk::Chunk;
use debug::debug_chunk;
use flamebytecode::FBOpCode;
use compiler::chunk::Const;

use crate::flamebytecode::FBOpCode::*;

mod strings;
mod flamebytecode;
mod vm;
mod compiler;
mod debug;
mod utils;
pub mod error;

const IDENTIFIER_MAX_LENGTH: usize = 32;
const STACK_LENGTH: usize = 512;

//#[cfg(test)]
mod test {
    use std::{rc::Rc, mem::{self, size_of}, collections::{HashSet, HashMap}, str::FromStr, path::PathBuf, fs, cell::RefCell};


    use clap::error::ErrorKind;
    use toml::Table;

    use crate::{compiler::Compiler, debug::debug_chunk, error::PhoenixError, vm::{Vm, Stack, value::{Value, Pointer}}, strings::InternStr};

    #[test]
    pub fn test() -> Result<(), Vec<PhoenixError>> {
        let chunk = Compiler::compile(PathBuf::from(r"/home/matteo/rust/phoenixlang/test/"))?;
        debug_chunk(&chunk);
        let vm = Vm { chunk, pc: 0, stack: Stack::new(), globals: Default::default(), strings: InternStr::new() }.run(false);
        Ok(())
    }

    //#[test]
    pub fn test_general() {
        let a = Rc::new(RefCell::new(1));
        println!("{}", a.borrow());
    }
}

pub fn execute(bytes: Vec<u8>) {

}

fn compile(str: String) {

}
