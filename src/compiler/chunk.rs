use core::panic;
use std::{collections::{HashSet, HashMap}, io::Read, fmt::Debug};

use crate::{flamebytecode::FBOpCode, vm::value::Value};

pub struct Chunk {
    pub consts: ConstPool,
    pub code: Vec<u8>,
}

pub enum ConstPool {
    Compiler { hash: HashMap<Const, u32>, len: u32, },
    Vm(Vec<Const>),
}


#[derive(Eq, Hash, PartialEq)]
pub enum Const {
    Int(i64),
    Dec(u64),
    String(Box<str>),
    Char(char)
}

impl Debug for Const { fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
        Const::String(v) => write!(f, "str: '{v}'"),
        Const::Int(v) => write!(f, "int: {v}"),
        Const::Dec(v) => write!(f, "dec: {}", f64::from_bits(*v)),
        Const::Char(v) => write!(f, "char: '{v}'"),
    }}}

impl ConstPool {
    pub fn compiler() -> Self { Self::Compiler { hash: HashMap::default(), len: 0 } }
    pub fn vm() -> Self { Self::Vm(vec![]) }
    pub fn as_vm(&self) -> &Vec<Const> { let Self::Vm(v) = self else { unreachable!() }; v }
    pub fn to_compiler(self) -> (HashMap<Const, u32>, u32) { let Self::Compiler{hash, len} = self else { unreachable!() }; (hash, len) }
}

impl Chunk {
    pub fn new() -> Chunk {
        return Chunk {
            consts: ConstPool::compiler(),
            code: vec![] }}

    pub fn build(mut self) -> Self {
        let (hash, _) = self.consts.to_compiler();
        let mut vec: Vec<_> = hash.into_iter().collect(); vec.sort_by(|(_, a), (_, b)| a.cmp(b));
        self.consts = ConstPool::Vm(vec.into_iter().map(|x| x.0).collect()); self
    }

    pub fn write(&mut self, byte: &[u8]) { byte.into_iter().for_each(|b| self.code.push(*b)) }
    pub fn write_op(&mut self, byte: FBOpCode) { self.code.push(byte as u8) }
    pub fn write_const(&mut self, constant: Const) {
        self.write_op(FBOpCode::OpConstant);
        let i = self.add_get_const(constant);
        self.write(&i.to_le_bytes()[..3]);
    }
    pub fn add_get_const(&mut self, constant: Const) -> u32 {
        let ConstPool::Compiler { hash, len } = &mut self.consts else { unreachable!() }; 
        if let Some(i) = hash.get(&constant) { *i } else { 
            if (*len >= 0xFFFFFF) { panic!("Too much constants. Report this error to us.")}
            let i = *len; *len += 1;
            hash.insert(constant, i); i
        }
    }
}








