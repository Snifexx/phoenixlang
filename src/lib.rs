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

#[cfg(test)]
mod test {
    use std::{rc::Rc, fs};

    use rustc_hash::{FxHashMap, FxHashSet};

    use crate::compiler::scanner::Scanner;

    #[test]
    pub fn test() {}
}


pub fn execute(bytes: Vec<u8>) {

}

fn compile(str: String) {

}
