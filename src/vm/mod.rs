use std::{cell::RefCell, rc::Rc, usize, collections::{HashMap, HashSet}, hash::BuildHasherDefault};
use ahash::AHasher;

use crate::{compiler::chunk::{Chunk, Const}, flamebytecode::{FBOpCode, debug, run}};
use self::value::Value;

pub mod value;

pub type Stack = Vec<Rc<RefCell<Value>>>;

pub trait Stacktrait { fn push_val(&mut self, value: Value); } 
impl Stacktrait for Stack { fn push_val(&mut self, value: Value) { self.push(Rc::new(RefCell::new(value))) }}

pub struct Vm {
    pub chunk: Chunk,
    pub pc: u64,
    pub stack: Stack,
    pub str_intern: HashSet<Rc<String>, BuildHasherDefault<AHasher>>,
}

impl Vm {
    pub fn run(mut self, debug_flag: bool) -> u8 {
        loop {
            let byte = self.chunk.code[self.pc as usize];
            let size = FBOpCode::size()[byte as usize] as usize;
            let exit_code = run(&mut self, size);
            if let Some(code) = exit_code { return code }
            if debug_flag { debug(self.pc, &self.chunk.code[self.pc as usize..self.pc as usize + size]); }
            if self.chunk.code.len() - size <= self.pc as usize { break; } 
            self.pc += size as u64;
        }
        println!("\n\n{:?}\n", self.stack);
        0
    }
}
