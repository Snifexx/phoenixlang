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
    use std::rc::Rc;

    use rustc_hash::{FxHashMap, FxHashSet};

    use crate::{compiler::{scanner::Scanner, module::Module}, debug::debug_chunk, error::PhoenixError, vm::Vm};

    #[test]
    pub fn test() -> Result<(), PhoenixError> {
        let src = 
r#"
-10.0 / (+3.0 - 1.0)
"#;
        let scanned = Scanner::new(src.to_string()).scan()?;
        println!("{:?}",scanned);

        let chunk = Module::new(scanned, Rc::new("Test".to_string())).compile(&mut FxHashSet::default())?.build();
        debug_chunk(&chunk);
        let vm = Vm { chunk, pc: 0, stack: vec![] }.run(false);
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
