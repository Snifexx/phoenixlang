use crate::{compiler::{token::{Token, TokenType}, chunk::{Chunk, Const}, Compiler}, error::{PhoenixError, CompErrID}, flamebytecode::FBOpCode};

use super::{types::{Type, parse_type}, Module};


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
        ty => type_error(rht.0, &[], op.pos.0, op.pos.1, format!("Type '{}' has not 'plus' function", ty)),
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
                ty => type_error(rht.0, &[], op.pos.0, op.pos.1, format!("Type '{}' has no '{}' function", ty, $name_str)),
            }
        }
    };
}
int_float_arithmetics!(minus, "minus", FBOpCode::OpSub, "subtracted");
int_float_arithmetics!(star, "mul", FBOpCode::OpMul, "multiplied");
int_float_arithmetics!(slash, "div", FBOpCode::OpDiv, "divided");


pub fn negate(chunk: &mut Chunk, rht: (Type, (u16, u16)), op: &Token) -> Result<Type, PhoenixError> {
    match rht.0 {
        ty @ (Type::Int | Type::Dec) => { chunk.write_op(FBOpCode::OpNeg); Ok(ty) }
        ty => type_error(rht.0, &[], op.pos.0, op.pos.1, format!("Type '{}' has no 'negate' function", ty)),
    }
}

impl Module {
    pub fn consume(&mut self, ty: TokenType) -> Result<(), PhoenixError> {
        if self.tokens[self.i].ty != ty { 
            let ret = Err(PhoenixError::Compile { id: CompErrID::InvalidCharacter, row: self.curr_tok().pos.0, col: self.curr_tok().pos.1, msg: format!("Expected {ty:?}, found {:?}", self.curr_tok().ty) });
            self.i += 1; return ret
        } self.i += 1; Ok(())
    }

    pub fn _let(&mut self) -> Result<Type, PhoenixError> {
        self.i += 1;
        let name = self.tokens[self.i].lexeme.take().ok_or_else(|| PhoenixError::Compile { id: CompErrID::InvalidSymbol, row: self.curr_tok().pos.0, col: self.curr_tok().pos.1, 
            msg: format!("No symbol name was provided") })?[1..].to_string();
        let (req_ty, pos): (Option<Type>, (u16, u16)) = if self.curr_tok().ty == TokenType::Colon {
            self.i += 1;
            let pos = self.curr_tok().pos;
            (Some(parse_type(self)?), pos)
        } else { (None, (0, 0)) };

        self.i += 1;
        self.consume(TokenType::Eq);
        let pos = if pos == (0, 0) { self.curr_tok().pos } else { pos };
        let ty = self.expression_parsing(0)?;
        
        let ty = match (req_ty, ty) {
            (None, Type::Unknown) => return Err(PhoenixError::Compile { id: CompErrID::TypeError, row: pos.0, col: pos.1,
                msg: format!("Type cannot be inferred, must be specified") }),
            (None, ty) => ty,
            (Some(req_ty), ty) if req_ty == ty => ty,
            _ => return Err(PhoenixError::Compile { id: CompErrID::TypeError, row: pos.0, col: pos.1,
                msg: format!("Expected value of type '{}' as specified, type '{}' was instead provided", req_ty.unwrap(), ty) }),
        };

        let glob = Compiler::intern_str_ref(&mut self.compiler.as_mut().unwrap().lock().unwrap().interned_str, &name);
        self.globals.insert(glob, ty);
        let i = self.chunk.as_mut().unwrap().add_get_const(Const::String(name));
        self.chunk.as_mut().unwrap().write_op(FBOpCode::OpGlobSet);
        self.chunk.as_mut().unwrap().write(&i.to_le_bytes()[0..3]);
        Ok(Type::Void)
    }
}




