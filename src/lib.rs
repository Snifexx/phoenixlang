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
    use std::{rc::Rc, mem, collections::HashSet};


    use crate::{compiler::{scanner::Scanner, module::Module, Compiler}, debug::debug_chunk, error::PhoenixError, vm::{Vm, value::Value}};

    #[test]
    pub fn test() -> Result<(), Vec<PhoenixError>> {
        let src = 
r#"
print "sesso" + 1 
print "sesso1" + 8
"#;

        let chunk = Compiler::compile_string(src.to_string())?;
        debug_chunk(&chunk);
        let vm = Vm { chunk, pc: 0, stack: vec![], str_intern: HashSet::default() }.run(false);
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
