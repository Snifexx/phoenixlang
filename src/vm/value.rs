use std::{str::Bytes, vec, u8, cell::{RefCell, Cell}, mem, rc::Rc, fmt::Display};

use crate::compiler::chunk::{Chunk, Const};

#[derive(Clone, Debug)]
pub enum Pointer { Stack(usize), Heap(Rc<Value>) }

impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Pointer::Stack(ptr) => write!(f, "[stack pointer #{ptr}]"), Pointer::Heap(ptr) => write!(f, "{}", ptr) }}
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Dec(f64),
    Str(Rc<str>),
    Char(char),
    Ptr(Pointer)
}

impl Default for Value { fn default() -> Self { Self::Bool(true) }}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(i) => write!(f, "{i}"),
            Value::Dec(d) => write!(f, "{d:?}"),
            Value::Str(str) => write!(f, "{}", *str),
            Value::Char(c) => write!(f, "{c}"),
            Value::Ptr(ptr) => write!(f, "{ptr}"),
        }
    }
}
