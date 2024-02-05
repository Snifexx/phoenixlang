use crate::{compiler::{token::Token, chunk::Chunk}, error::{PhoenixError, CompErrID}, flamebytecode::FBOpCode};

use super::types::Type;


#[inline(always)]
fn type_error(ty: Type, acceptable: &[Type], row: u16, col: u16, msg: String) -> Result<Type, PhoenixError> { 
    if acceptable.contains(&ty) { Ok(ty) }
    else { Err(PhoenixError::Compile { id: CompErrID::TypeError, row, col, msg }) }
}

pub fn plus(chunk: &mut Chunk, lht: (Type, (u16, u16)), rht: (Type, (u16, u16)), op: &Token) -> Result<Type, PhoenixError> {
    match lht.0 {
        ty @ (Type::Int | Type::Dec) => {
            let ret_ty = type_error(rht.0, &[ty], lht.1.0, lht.1.1, format!("Type '{}' cannot be added to a {}", rht.0, ty))?;
            chunk.write_op(FBOpCode::OpAdd); Ok(ret_ty)
        }
        Type::Str => {
            let ret_ty = match rht.0 {
                Type::Str | Type::Char => Type::Str,
                _ => type_error(rht.0, &[], lht.1.0, lht.1.1, format!("Cannot concat Str with {}", rht.0))?,
            };
            chunk.write_op(FBOpCode::OpAdd); Ok(ret_ty)
        }
        ty => type_error(rht.0, &[], lht.1.0, lht.1.1, format!("Type '{}' has not 'plus' function", ty)),
    }
}

macro_rules! int_float_arithmetics {
    ($name:ident, $name_str:literal, $op:expr, $verb:literal) => {
        pub fn $name(chunk: &mut Chunk, lht: (Type, (u16, u16)), rht: (Type, (u16, u16)), op: &Token) -> Result<Type, PhoenixError> {
            match lht.0 {
                ty @ (Type::Int | Type::Dec) => {
                    let ret_ty = type_error(rht.0, &[ty], lht.1.0, lht.1.1, format!("Type '{}' cannot be {} to a {}", rht.0, $verb, ty))?;
                    chunk.write_op($op); Ok(ret_ty)
                }
                ty => type_error(rht.0, &[], lht.1.0, lht.1.1, format!("Type '{}' has not '{}' function", ty, $name_str)),
            }
        }
    };
}

int_float_arithmetics!(minus, "minus", FBOpCode::OpSub, "subtracted");
int_float_arithmetics!(star, "mul", FBOpCode::OpMul, "multiplied");
int_float_arithmetics!(slash, "div", FBOpCode::OpDiv, "divided");







