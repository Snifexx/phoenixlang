use crate::{compiler::{token::Token, chunk::Chunk}, error::{PhoenixError, CompErrID}};

use super::types::Type;


#[inline(always)]
fn type_error(row: u16, col: u16, msg: String) -> Result<Type, PhoenixError> { Err(PhoenixError::Compile { id: CompErrID::TypeError, row, col, msg }) }

pub fn plus(chunk: &mut Chunk, lht: (Type, (u16, u16)), rht: (Type, (u16, u16)), op: &Token) -> Result<Type, PhoenixError> {
    match lht.0 {
        Type::Int => {
            let ret_ty = match rht.0 {
                ty @ (Type::Int | Type::Dec) => ty,
                ty => return type_error(lht.1.0, lht.1.1, format!("Type '{}' has not 'plus' function", ty)),
            };

            Ok(ret_ty)
        }
        ty => type_error(lht.1.0, lht.1.1, format!("Type '{}' has not 'plus' function", ty)),
    }
}
