use std::{str::Bytes, vec, u8, cell::{RefCell, Cell}, mem, rc::Rc, fmt::Display, borrow::Cow};

use crate::compiler::chunk::{Chunk, Const};

use super::Vm;

#[derive(Clone, Debug)]
pub enum Pointer { Local(usize), Global(Rc<str>) }
impl Display for Pointer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self { Pointer::Local(ptr) => write!(f, "[stack pointer #{ptr}]"), Pointer::Global(name) => write!(f, "[pointer glob_sym '{name}']") }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Dec(f64),
    Str(Rc<str>),
    Char(char),
    Ptr(Pointer),
    Upv(Rc<Value>)
}

impl Default for Value { fn default() -> Self { Self::Bool(true) }}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"), Value::Int(i) => write!(f, "{i}"), Value::Dec(d) => write!(f, "{d:?}"), Value::Str(str) => write!(f, "{}", *str), 
            Value::Char(c) => write!(f, "{c}"), Value::Ptr(ptr) => write!(f, "{ptr}"), Value::Upv(upv) => write!(f, "{upv}") }}
}

impl Value {
    pub fn point(self, vm: &Vm) -> Cow<Value> {
        match self {
            Value::Ptr(ptr) => Cow::Borrowed(match ptr { Pointer::Local(loc) => &vm.stack[loc], Pointer::Global(glob) => &vm.globals[&glob], }),
            v => Cow::Owned(v)
        }
    }
}
