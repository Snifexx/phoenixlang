use std::{cell::RefCell, rc::Rc, usize, collections::{HashMap, HashSet}, hash::BuildHasherDefault, ops::DerefMut, ops::Deref, mem::{self, MaybeUninit}};
use ahash::AHasher;

use crate::{compiler::chunk::{Chunk, Const}, flamebytecode::{FBOpCode, debug, run}, strings::{InternStrSync, InternStr}, STACK_LENGTH};
use self::value::Value;

pub mod value;

#[derive(Debug)]
pub struct Stack { 
    array: [Value; STACK_LENGTH],
    top: usize
}


impl Deref for Stack { type Target = [Value]; fn deref(&self) -> &Self::Target { &self.array }}
impl DerefMut for Stack { fn deref_mut(&mut self) -> &mut Self::Target { &mut self.array }}

impl Stack {
    pub fn new() -> Self { Self { array: {
    let mut data: [MaybeUninit<Value>; STACK_LENGTH] = unsafe { MaybeUninit::uninit().assume_init() };

    for elem in &mut data[..] { elem.write(Value::default()); }

    unsafe { mem::transmute::<_, [Value; STACK_LENGTH]>(data) } },
    top: 0 } }
    pub fn push(&mut self, value: Value) { self.array[self.top] = value; self.top += 1; }
    pub fn pop(&mut self) -> Value { self.top -= 1; mem::take(&mut self.array[self.top]) }
}

pub struct Vm {
    pub chunk: Chunk,
    pub pc: u64,
    pub stack: Stack,
    pub strings: InternStr,
    pub globals: HashMap<Rc<str>, Value, BuildHasherDefault<AHasher>>
}

impl Vm {
    pub fn run(mut self, debug_flag: bool) -> u8 {
        loop {
            let byte = self.chunk.code[self.pc as usize];
            let size = FBOpCode::size()[byte as usize] as usize;
            let exit_code = run(&mut self, size);
            //println!("|{:?}\n", &self.stack[0..self.stack.top]);
            if let Some(code) = exit_code { return code }
            if debug_flag { debug(self.pc, &self.chunk.code[self.pc as usize..self.pc as usize + size]); }
            if self.chunk.code.len() - size <= self.pc as usize { break; } 
            self.pc += size as u64;
        }
        println!("\n\n{:?}\n", &self.stack[0..self.stack.top]);
        0
    }
}
