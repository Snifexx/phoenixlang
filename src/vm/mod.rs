use std::{cell::RefCell, rc::Rc, usize};
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
    pub line: u16,
}

impl Vm {
    fn run(mut self, debug_flag: bool) -> u8 {
        loop {
            let byte = self.chunk.code[self.pc as usize];
            let size = FBOpCode::size()[byte as usize] as usize;
            let exit_code = run(&mut self, size);
            if let Some(code) = exit_code { return code }
            if debug_flag { debug(self.line, self.pc, &self.chunk.code[self.pc as usize..self.pc as usize + size]); }
        }
    }
}
