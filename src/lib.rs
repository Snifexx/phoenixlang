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

//#[cfg(test)]
mod test {
    use std::{rc::Rc, mem, collections::{HashSet, HashMap}, str::FromStr, path::PathBuf};


    use clap::error::ErrorKind;
    use toml::Table;

    use crate::{compiler::{scanner::Scanner, module::Module, Compiler}, debug::debug_chunk, error::PhoenixError, vm::{Vm, value::Value}, strings::{InternStrSync, InternStr}};

    #[test]
    pub fn test() -> Result<(), Vec<PhoenixError>> {

        let chunk = Compiler::compile(PathBuf::from(r"/home/matteo/rust/phoenixlang/test/"))?;
        debug_chunk(&chunk);
        let vm = Vm { chunk, pc: 0, stack: vec![], globals: Default::default(), strings: InternStr::new() }.run(false);
        Ok(())
    }

    #[test]
    pub fn test_general() {
        let a: Box<str> = "sesso".into();
        let mut b = InternStrSync::new();
            
        let res = b.intern_str(&*a);
        println!("{res}");
    }
}


pub fn execute(bytes: Vec<u8>) {

}

fn compile(str: String) {

}
