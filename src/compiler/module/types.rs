use std::{fmt::{Display, Debug}, any::Any};



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Void, Bool, Dec, Int, Str, Char
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "Void"), Type::Bool => write!(f, "Bool"),
            Type::Dec => write!(f, "Dec"), Type::Int => write!(f, "Int"),
            Type::Str => write!(f, "Str"), Type::Char => write!(f, "Char"),
        }
    }
}

impl Type {
    pub fn bytes() -> Vec<u8> {
        // TODO
        vec![]
    }
}
