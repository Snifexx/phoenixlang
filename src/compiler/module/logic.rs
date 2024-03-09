use crate::{compiler::{token::{Token, TokenType, TokenType::*}, chunk::{Chunk, Const}, Compiler}, error::{PhoenixError, CompErrID}, flamebytecode::FBOpCode};
use std::{string::String, sync::Arc};

use super::{types::{Type, parse_type}, Module, Local};


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

   pub fn variable(&mut self) -> Result<Type, PhoenixError> {
       let name = self.curr_tok().lexeme.take().unwrap()[1..].to_owned();
       let pos = self.curr_tok().pos;

        if !self.globals.contains_key(&*name) { return Err(PhoenixError::Compile { id: CompErrID::MissingGlobalSymbol, row: pos.0, col: pos.1, msg: format!("Global symbol '{name}' not found") }) }

        let ty = self.globals[&*name];

        // If it's not a setter (I.E. 'symbol [=, +=, -=, *=, /=]')
        if ![Eq, PlusEq, MinusEq, StarEq, SlashEq].contains(&self.tokens[self.i + 1].ty) {
            if self.scope_depth == 0 {
                self.chunk.as_mut().unwrap().write_op(FBOpCode::OpGlobGet);
                let name_const = self.chunk.as_mut().unwrap().add_get_const(Const::String(name.into()));
                self.chunk.as_mut().unwrap().write(&name_const.to_le_bytes()[..3]);
            } else if let Some(addr) = self.locals.iter().enumerate().rev()
                .find(|(pos, loc)| { &*loc.name == name }).map(|(addr, _)| addr) {
                    self.chunk.as_mut().unwrap().write_op(FBOpCode::OpLocGet);
                    self.chunk.as_mut().unwrap().write(&addr.to_le_bytes()[..3]);
                } else { return Err(PhoenixError::Compile { id: CompErrID::UnknownSymbol, row: pos.0, col: pos.1,
                    msg: format!("Unknown global or local symbol '{name}'") }) }
            Ok(ty)
        }
        else {
            let op_i = self.i + 1;
            let is_eq = self.tokens[op_i].ty == Eq;
            self.i += 2;
            let name_const = self.chunk.as_mut().unwrap().add_get_const(Const::String(name.into()));

            if !is_eq { self.chunk.as_mut().unwrap().write_op(FBOpCode::OpGlobGet); self.chunk.as_mut().unwrap().write(&name_const.to_le_bytes()[..3]); }

            let expr_pos = self.curr_tok().pos;
            let expr_ty = self.expression_parsing(0)?;

            if !is_eq { Self::operation(self.chunk.as_mut().unwrap(), Some((ty, pos)), (expr_ty, expr_pos), &self.tokens[op_i])?; }

            if expr_ty != ty && is_eq { return Err(PhoenixError::Compile { id: CompErrID::TypeError, row: expr_pos.0, col: expr_pos.1,
                msg: format!("Cannot assign value of type '{expr_ty}' to symbol of type '{ty}'") }); }
            self.chunk.as_mut().unwrap().write_op(FBOpCode::OpGlobSet);
            self.chunk.as_mut().unwrap().write(&name_const.to_le_bytes()[..3]);
            Ok(Type::Void)
        }
   }

    pub fn _let(&mut self) -> Result<Type, PhoenixError> {
        self.i += 1;
        let name = &self.tokens[self.i].lexeme.take().ok_or_else(|| PhoenixError::Compile { id: CompErrID::InvalidSymbol, row: self.curr_tok().pos.0, col: self.curr_tok().pos.1, 
            msg: format!("No symbol name was provided") })?[1..];
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

        let name = { 
            let mut lock = self.compiler.as_mut().unwrap().lock().unwrap();
            lock.strings.intern_str(name)
        };
        
        self.set_symbol(name, ty);
        Ok(Type::Void)
    }

    fn set_symbol(&mut self, name: Arc<str>, ty: Type) {
        if self.scope_depth == 0 {
            let i = self.chunk.as_mut().unwrap().add_get_const(Const::String((&*name).into()));
            self.globals.insert(name, ty);

            self.chunk.as_mut().unwrap().write_op(FBOpCode::OpGlobSet);
            self.chunk.as_mut().unwrap().write(&i.to_le_bytes()[0..3]);
        } else {
            let new_local = Local { name: name.clone(), depth: self.scope_depth, ty };

            if let Some(local) = self.locals.iter_mut().rev()
                .filter(|x| x.depth >= self.scope_depth)
                    .find(|x| x.name == name) { 
                        *local = new_local;
                        self.chunk.as_mut().unwrap().write_op(FBOpCode::OpLocSet);
                        self.chunk.as_mut().unwrap().write(&self.i.to_le_bytes()[0..3]);
                    } 
            else { if self.locals.len() < 0xFFFFFF { self.locals.push(new_local); } else { panic!("scope local_limit reached") }; }
        }
    }
}




