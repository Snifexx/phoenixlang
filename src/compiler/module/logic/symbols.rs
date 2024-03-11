use either::Either;

use crate::compiler::module::Local;
use std::sync::Arc;
use crate::compiler::module::logic::parse_type;
use crate::compiler::module::{TokenType, TokenType::*};
use crate::Const;
use crate::FBOpCode;
use crate::error::CompErrID;
use crate::error::PhoenixError;
use crate::compiler::module::Type;
use crate::compiler::module::Module;

impl Module {
    pub fn variable(&mut self) -> Result<Type, PhoenixError> {
        let name = &self.curr_tok().lexeme.take().unwrap()[1..];
        let pos = self.curr_tok().pos;

        if [Eq, PlusEq, MinusEq, StarEq, SlashEq].contains(&self.tokens[self.i + 1].ty) {
            self.assignment(name, pos) } else { self.get_symbol(name, pos) }
    }
    

    pub fn _let(&mut self) -> Result<Type, PhoenixError> {
        self.i += 1;
        let name = &self.tokens[self.i].lexeme.take().ok_or_else(|| PhoenixError::Compile { id: CompErrID::InvalidSymbol, row: self.curr_tok().pos.0, col: self.curr_tok().pos.1, 
            msg: format!("No symbol name was provided") })?[1..];
        let pos = self.curr_tok().pos;

        self.i += 1;
        let (req_ty, req_pos): (Option<Type>, (u16, u16)) = 
                                if self.curr_tok().ty == TokenType::Colon {
                                    self.i += 1;
                                    let pos = self.curr_tok().pos;
                                    (Some(parse_type(self)?), pos)
                                } else { (None, (0, 0)) };

        self.consume(TokenType::Eq);
        let req_pos = if req_pos == (0, 0) { self.curr_tok().pos } else { req_pos };

        let ty = self.expression_parsing(0)?;

        let ty = match (req_ty, ty) {
            (None, Type::Unknown) => return Err(PhoenixError::Compile { id: CompErrID::TypeError, row: req_pos.0, col: req_pos.1,
                msg: format!("Type cannot be inferred, must be specified") }),
            (Some(req_ty), ty) if req_ty != ty => return Err(PhoenixError::Compile { id: CompErrID::TypeError, row: req_pos.0, col: req_pos.1,
                msg: format!("Expected value of type '{}' as specified, type '{}' was instead provided", req_ty, ty) }),
            (_, ty) => ty,
        };

        let name = self.compiler.as_mut().unwrap().lock().unwrap().strings.intern_str(name);

        self.set_symbol(&*name, pos, ty, true);
        Ok(Type::Void)
    }

    pub fn assignment(&mut self, name: &str, pos: (u16, u16)) -> Result<(), PhoenixError> {
        self.i += 1;
        let lht = self.resolve_symbol(name).map(|either| match either { Either::Left((_, local)) => local.ty , Either::Right(ty) => ty })
            .ok_or_else(|| PhoenixError::Compile { id: CompErrID::UnknownSymbol, row: pos.0, col: pos.1,
                msg: format!("Cannot assign to unknown symbol") })?;

        let op = match self.curr_tok().ty {
            Eq => None,
            PlusEq | MinusEq | StarEq | SlashEq => { self.get_symbol(name, pos); Some(&self.tokens[self.i]) }
            _ => unreachable!()
        };
        self.i += 1;

        let rht_pos = self.curr_tok().pos;
        let rht = self.expression_parsing(0)?;

        let expr_ty = match op {
            Some(op) => Self::operation(self.chunk.as_mut().unwrap(), Some((lht, pos)), (rht, rht_pos), op)?,
            None => rht,
        };

        if lht != expr_ty { return Err(PhoenixError::Compile { id: CompErrID::TypeError, row: rht_pos.0, col: rht_pos.1,
            msg: format!("Cannot assign expression of type '{expr_ty}' to symbol '{name}' of type '{lht}'") }) }

        self.set_symbol(name, pos, ty, declare_absent)
        Ok(())
    }

    fn get_symbol(&mut self, name: &str, pos: (u16, u16)) -> Result<Type, PhoenixError> {
        let symbol = self.resolve_symbol(name).ok_or_else(|| PhoenixError::Compile { id: CompErrID::UnknownSymbol, row: pos.0, col: pos.1,
            msg: format!("Unknown symbol '{name}'")})?;

        match symbol {
            Either::Left((addr, loc)) => {
                self.chunk.as_mut().unwrap().write_op(FBOpCode::OpLocGet);
                self.chunk.as_mut().unwrap().write(&addr.to_le_bytes()[..3]);
                Ok(loc.ty)
            }
            Either::Right(ty) => {
                self.chunk.as_mut().unwrap().write_op(FBOpCode::OpGlobGet);
                let name_const = self.chunk.as_mut().unwrap().add_get_const(Const::String(name.into()));
                self.chunk.as_mut().unwrap().write(&name_const.to_le_bytes()[..3]);
                Ok(ty)
            }
        }
    }


    fn set_symbol(&mut self, name: &str, pos: (u16, u16), ty: Type, declare: bool) -> Result<(), PhoenixError> {
        let symbol = self.resolve_symbol(name);

        match symbol {
            Some(Either::Right(ty)) => {
                let const_addr = self.chunk.as_mut().unwrap().add_get_const(Const::String(name.into()));

                self.chunk.as_mut().unwrap().write_op(FBOpCode::OpGlobSet);
                self.chunk.as_mut().unwrap().write(&const_addr.to_le_bytes()[0..3]);
            }
            _ if declare => {
                let name = self.compiler.as_mut().unwrap().lock().unwrap().strings.intern_str(name);
                self.locals.push(Local { name, depth: self.scope_depth, ty })
            }
            Some(Either::Left((addr, local))) => {
                self.chunk.as_mut().unwrap().write_op(FBOpCode::OpLocSet);
                self.chunk.as_mut().unwrap().write(&addr.to_le_bytes()[0..3]);
            }
            None => return Err(PhoenixError::Compile { id: CompErrID::UnknownSymbol, row: pos.0, col: pos.1,
                msg: format!("Cannot assign to unknown symbol '{name}'") })
        }
        Ok(())
    }

    fn resolve_symbol(&mut self, name: &str) -> Option<Either<(usize, &Local), Type>> {
        if let Some((addr, loc)) = self.locals.iter().enumerate().rev()
            .filter(|(_, loc)| loc.depth <= self.scope_depth)
                .find(|(addr, loc)| &*loc.name == name) {
                    Some(Either::Left((addr, loc)))
                } else if let Some(ty) = self.globals.get(name) {
                    Some(Either::Right(*ty))
                } else {
                    None
                }
    }
}
