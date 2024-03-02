use std::{str::Bytes, vec, u8, cell::{RefCell, Cell, Ref}, mem, rc::Rc, fmt::{Display, }, borrow::Cow, ops::DerefMut};

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
    Upv(Rc<RefCell<Value>>)
}

impl Default for Value { fn default() -> Self { Self::Bool(true) }}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"), Value::Int(i) => write!(f, "{i}"), Value::Dec(d) => write!(f, "{d:?}"), Value::Str(str) => write!(f, "{}", *str), 
            Value::Char(c) => write!(f, "{c}"), Value::Ptr(ptr) => write!(f, "{ptr}"), Value::Upv(upv) => write!(f, "{}", upv.borrow()) }}
}

impl Vm {
    pub fn with_attached_val<R>(&mut self, addr: usize, f: impl FnOnce(&mut Value) -> R) -> R {
        let reference = &self.stack[addr];
        let mut reference = match reference {
            Value::Ptr(ptr) => match ptr { 
                Pointer::Local(loc) => {let loc = *loc; drop(reference); &mut self.stack[loc]}, 
                Pointer::Global(glob) => self.globals.get_mut(&*glob).unwrap(), 
            }
            _ => &mut self.stack[addr]
        };
        match reference {
            Value::Upv(rc) => f(&mut *rc.borrow_mut()),
            r => f(r)
        }
    }
    pub fn with_pointed_val<R>(&mut self, addr: usize, f: impl FnOnce(&Value) -> R) -> R {
        let rf = match &self.stack[addr] {
            Value::Ptr(ptr) => match ptr { Pointer::Local(loc) => &self.stack[*loc], Pointer::Global(glob) => &self.globals[&*glob] }
            r => return f(r)
        };
        match rf {
            Value::Upv(rc) => f(&*rc.borrow()),
            r => f(r)
        }
    }
    pub fn denested_pointer(&self, addr: usize) -> Value {
        match &self.stack[addr] {
            ptr @ Value::Ptr(_) => ptr.clone(),
            v => Value::Ptr(Pointer::Local(addr))
        }
    }
}

impl Value {
    pub fn depoint(self, vm: &Vm) -> Cow<Value> {
        let res = match self {
            Value::Ptr(ptr) => Cow::Borrowed(match ptr { 
                Pointer::Local(loc) => &vm.stack[loc], 
                Pointer::Global(glob) => &vm.globals[&glob], 
            }),
            v => return Cow::Owned(v)
        };
        match *res {
            Value::Upv(rc) => Cow::Borrowed(&*rc.borrow()),
            _ => res
        }
    }
}


