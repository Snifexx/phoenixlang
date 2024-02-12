use std::{fmt::{Display, Debug}, any::Any, str::FromStr};

use crate::{compiler::token::TokenType, error::{PhoenixError, CompErrID}};

use super::Module;



#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Void, Bool, Dec, Int, Str, Char, Unknown,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTypeError;

impl FromStr for Type {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Void" | "()" => Ok(Self::Void), "Bool" => Ok(Self::Bool), "Dec" => Ok(Self::Dec), 
            "Int" => Ok(Self::Int), "Str" => Ok(Self::Str), "Char" => Ok(Self::Char),
            _ => Err(ParseTypeError)
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Void => write!(f, "Void"), Type::Bool => write!(f, "Bool"),
            Type::Dec => write!(f, "Dec"), Type::Int => write!(f, "Int"),
            Type::Str => write!(f, "Str"), Type::Char => write!(f, "Char"),
            Type::Unknown => unreachable!("Tried to print Unknown"),
        }
    }
}

impl Type {
    pub fn bytes() -> Vec<u8> {
        // TODO
        //      todo string encoding implementation, for now only utf-8
        vec![]
    }
}

pub fn parse_type(module: &mut Module) -> Result<Type, PhoenixError> {
    let pos = module.curr_tok().pos;
    let t = &module.tokens[module.i];
    match t.ty {
        TokenType::LParen if module.tokens[module.i].ty == TokenType::RParen => {
            module.i += 1;
            Ok(Type::Void)
        }
        TokenType::Identifier => {
            let str = module.curr_tok().lexeme.take().unwrap();
            Type::from_str(&*str).map_err(|_|
                PhoenixError::Compile { id: CompErrID::TypeError, row: pos.0, col: pos.1,
                msg: format!("Type '{}' is non-existent", str) })
        }
        _ => Err(PhoenixError::Compile { id: CompErrID::TypeError, row: pos.0, col: pos.1,
            msg: format!("Invalid or non-existent type") })
    }
}
