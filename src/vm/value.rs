use std::{str::Bytes, vec, u8, cell::{RefCell, Cell}, mem};

use crate::compiler::chunk::{Chunk, Const};


#[derive(Clone)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Dec(f64),
    Str(Box<String>),
}
