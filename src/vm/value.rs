use std::{str::Bytes, vec, u8, cell::{RefCell, Cell, Ref}, mem, rc::Rc, fmt::{Display, }, borrow::Cow, ops::{DerefMut, Deref}};


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
    pub fn denested_pointer(&self, addr: usize) -> Value {
        match &self.stack[addr] {
            ptr @ Value::Ptr(_) => ptr.clone(),
            v => Value::Ptr(Pointer::Local(addr))
        }
    }
}

impl Vm {
    pub fn with_depnt_upved<R>(&mut self, addr: usize, f: impl FnOnce(Box<dyn DerefMut<Target = Value> + '_>) -> R) -> R {
        f( match match &self.stack[addr] {
            Value::Ptr(Pointer::Local(addr)) => { let addr = *addr; &mut self.stack[addr] }
            Value::Ptr(Pointer::Global(symname)) => self.globals.get_mut(symname).unwrap(),
            _ => &mut self.stack[addr]
        } {
            Value::Upv(rc) => Box::new(rc.borrow_mut()),
            mut_ref => Box::new(mut_ref)
        })
    }
}

impl Value {
    pub fn promote_upv(&mut self, vm: &mut Vm) {
        self.with_depointed(vm, |val| {
            match val {
                Value::Upv(_) => {}
                _ => {let value = mem::take(val); *val = Value::Upv(Rc::new(RefCell::new(value))); } 
            }
        })
    }
    pub fn with_depointed<R>(&mut self, vm: &mut Vm, f: impl FnOnce(&mut Value) -> R) -> R {
        f(match self {
            Value::Ptr(Pointer::Local(addr)) => &mut vm.stack[*addr],
            Value::Ptr(Pointer::Global(symname)) => vm.globals.get_mut(symname).unwrap(),
            ptr => ptr
        })
    }
    pub fn depoint(self, vm: &Vm) -> Cow<Value> {
        match self {
            Value::Ptr(ptr) => Cow::Borrowed(match ptr { 
                Pointer::Local(loc) => &vm.stack[loc], 
                Pointer::Global(glob) => &vm.globals[&glob], 
            }),
            v => return Cow::Owned(v)
        }
    }
    pub fn deupvalue(&self, vm: &Vm) -> Box<dyn Deref<Target = Value> + '_> {
        match self {
            Value::Upv(rc) => Box::new(rc.borrow()),
            val => Box::new(val)
        }
    }
}
