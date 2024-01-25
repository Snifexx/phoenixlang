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


pub fn test() {
    test_vm()
}

fn test_vm() {
    let mut chunk = Chunk::new();

    chunk.write_const(Const::String("Test String".to_string()));
    chunk.write_op(FBOpCode::OpLine);
    chunk.write(&0x7B_u16.to_le_bytes());
    chunk.write_op(OpTrue);

    debug_chunk(&chunk.build());
}

pub fn execute(bytes: Vec<u8>) {

}

fn compile(str: String) {

}
