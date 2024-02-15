use core::panic;
use std::collections::{HashMap, HashSet};
use std::hash::BuildHasherDefault;
use std::sync::{Arc, Mutex};
use std::{string::String, any::TypeId};
use std::rc::Rc;
use ahash::AHasher;
use clap::builder::Str;
use crate::error::CompErrID;
use crate::flamebytecode::FBOpCode;
use crate::{error::PhoenixError, debug::debug_chunk};

use self::logic::{plus, minus, star, slash, negate};
use self::types::Type;

use crate::FBOpCode::*;
use super::Compiler;
use super::token::TokenType;
use super::{token::{Token, self, TokenType::*}, chunk::{Chunk, Const}};

mod types;
mod logic;

type AHashMap<K, V> = HashMap<K, V, BuildHasherDefault<AHasher>>;

pub struct Module {
    id: Rc<String>,
    tokens: Vec<Token>, i: usize,

    imports: AHashMap<Rc<String>, Rc<String>>,
    funcs: AHashMap<Rc<String>, Funcs>,
    globals: AHashMap<Rc<String>, Type>,
    panic_mode: bool,
    // TODO chunk is temporary, will return module result table
    pub chunk: Option<Chunk>,
    compiler: Option<Arc<Mutex<Compiler>>>,
}

struct Funcs {
    name: Rc<String>,
    code: Chunk,
    dependencies: HashSet<Rc<String>, BuildHasherDefault<AHasher>>,
}

impl Module {
    pub fn new(tokens: Vec<Token>, id: Rc<String>, compiler: Arc<Mutex<Compiler>>) -> Self { 
        Self { tokens, id, compiler: Some(compiler), i: 0, imports:  AHashMap::default(), funcs: Default::default(), globals: Default::default(), chunk: Some(Chunk::new()), panic_mode: false }
    }

    #[inline(always)]
    pub fn curr_tok(&mut self) -> &mut Token { &mut self.tokens[self.i] }
    
    pub fn compile(mut self) -> Result<Arc<Mutex<Compiler>>, Vec<PhoenixError>> {
        let mut errors = vec![];

        while self.curr_tok().ty != Eof {
            let err = self.loose_statement();
            if err.is_err() { 
                errors.push(err.unwrap_err());
                loop {
                    let end_statement = self.curr_tok().pos.1 != self.tokens[self.i + 1].pos.1 || self.curr_tok().ty == Eof;
                    self.i += 1;
                    if end_statement { break; }
                }
            }
        }

        if errors.is_empty() { 
            let compiler = self.compiler.take().unwrap();
            compiler.lock().unwrap().modules.insert(self.id.clone(), self); 
            Ok(compiler)
        } else { Err(errors) }
    }

    pub fn loose_statement(&mut self) -> Result<(), PhoenixError> {
        if self.tokens[self.i].lexeme.as_ref().is_some_and(|str| &str[1..] == "print") { //TODO temporary print
            self.i += 1;
            let pos = self.tokens[self.i].pos;
            let ty = self.expression_parsing(0)?;
            match ty { Type::Void => return Err(PhoenixError::Compile { id: CompErrID::TypeError, row: pos.0, col: pos.1, msg: String::from("print statement requires a non-void expression") }), _ => {} }
            self.chunk.as_mut().unwrap().write_op(FBOpCode::OpPrint);
            return Ok(());
        }
        let ty = self.expression_parsing(0)?;
        match ty { Type::Void => {} _ => self.chunk.as_mut().unwrap().write_op(FBOpCode::OpPop), }
        Ok(())
    }

    pub fn expression_parsing(&mut self, min_bp: u8) -> Result<Type, PhoenixError> {
        let lht_pos = self.curr_tok().pos;
        let mut lht = match self.curr_tok().ty {
            Let => return self._let(),
            True | False => self.bool(),
            Int => self.int(),
            Dec => self.dec(), 
            String => self.string(),
            Identifier => { let ty = self.variable()?; if Type::Void == ty { return Ok(ty); } else { ty } }
            Plus => {
                self.i += 1;
                let ret_ty = self.expression_parsing(9)?;
                self.i -= 1; 
                ret_ty
            }
            Char => self.char(),
            LParen => {
                self.i += 1;
                let value = self.expression_parsing(0)?;
                assert_eq!(self.curr_tok().ty, RParen);
                value
            }
            op @ Minus => {
                let ((), r_bp) = prefix_bp(op);
                let tok_i = self.i; self.i += 1;
                let rht_pos = self.curr_tok().pos;
                let rhs = self.expression_parsing(r_bp)?;
                self.i -= 1;
                Self::operation(self.chunk.as_mut().unwrap(), None, (rhs, rht_pos), &self.tokens[tok_i])?
            }
            ty => unreachable!("{:?}", ty),
        };
        self.i += 1;

        loop {
            let op_i = self.i;
            if self.tokens[self.i - 1].pos.0 != self.curr_tok().pos.0 { break; }
            let op = match self.curr_tok().ty {
                SemiColon => { self.i += 1; break; }
                RParen | Eof  => break,
                op @ (Plus | Minus | Star | Slash) => &self.tokens[op_i], 
                op => unreachable!("{op:?}"),
            };
            
            if let Some((l_bp, ())) = postfix_bp(op.ty) { // Postfix
                if l_bp < min_bp { break; }
                self.i += 1;

                lht = if op.ty == LSquare {
                    let rht_pos = self.curr_tok().pos;
                    let rhs = self.expression_parsing(0)?;
                    assert_eq!(self.curr_tok().ty, RSquare);
                    // TODO Lquare get func
                    Type::Void // TODO type calculator
                } else { lht };
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_bp(op.ty) { // Infix
                if l_bp < min_bp { break; }
                self.i += 1;

                lht = {
                    let rht_pos = self.curr_tok().pos;
                    let rht = self.expression_parsing(r_bp)?;
                    let op = &self.tokens[op_i];
                    Self::operation(self.chunk.as_mut().unwrap(), Some((lht, lht_pos)), (rht, rht_pos), op)?
                };
                continue;
            }

            break
        }

        Ok(lht)
    }
    
   #[inline(always)]
   fn bool(&mut self) -> Type { let op = if self.curr_tok().ty == True { OpTrue } else { OpFalse }; self.chunk.as_mut().unwrap().write_op(op); Type::Bool }
   fn int(&mut self) -> Type {
       let num = self.curr_tok().lexeme.take().unwrap().parse::<i64>().unwrap();
       self.chunk.as_mut().unwrap().write_const(Const::Int(num));
       Type::Int
   }
   fn dec(&mut self) -> Type {
       let num = self.curr_tok().lexeme.take().unwrap().parse::<f64>().unwrap();
       self.chunk.as_mut().unwrap().write_const(Const::Dec(num.to_bits()));
       Type::Dec
   }
   fn string(&mut self) -> Type {
       let str = self.curr_tok().lexeme.take().unwrap();
       self.chunk.as_mut().unwrap().write_const(Const::String(str));
       Type::Str
   }
   fn char(&mut self) -> Type {
       let char = self.curr_tok().lexeme.take().unwrap().chars().next().unwrap();
       self.chunk.as_mut().unwrap().write_const(Const::Char(char));
       Type::Char
   }
   fn variable(&mut self) -> Result<Type, PhoenixError> {
       let name = self.curr_tok().lexeme.take().unwrap()[1..].to_owned();
       let pos = self.curr_tok().pos;

        if !self.globals.contains_key(&name) { return Err(PhoenixError::Compile { id: CompErrID::MissingGlobalSymbol, row: pos.0, col: pos.1, msg: format!("Global symbol '{name}' not found") }) }

        let ty = self.globals[&name];

        // If it's a setter (I.E. 'symbol [=, +=, -=, *=, /=]')
        if ![Eq, PlusEq, MinusEq, StarEq, SlashEq].contains(&self.tokens[self.i + 1].ty) {
            self.chunk.as_mut().unwrap().write_op(FBOpCode::OpGlobGet);
            let name_const = self.chunk.as_mut().unwrap().add_get_const(Const::String(name));
            self.chunk.as_mut().unwrap().write(&name_const.to_le_bytes()[..3]);
            Ok(ty)
        } else {
            let op_i = self.i + 1;
            let is_eq = self.tokens[op_i].ty == Eq;
            self.i += 2;
            let name_const = self.chunk.as_mut().unwrap().add_get_const(Const::String(name));

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


    fn operation(chunk: &mut Chunk, lht: Option<(Type, (u16, u16))>, rht: (Type, (u16, u16)), op: &Token) -> Result<Type, PhoenixError> {
        match op.ty {
            Plus | PlusEq => plus(chunk, lht.unwrap(), rht, op),
            Minus | MinusEq => if lht.is_some() { minus(chunk, lht.unwrap(), rht, op) } else { negate(chunk, rht, op) }
            Star | StarEq => star(chunk, lht.unwrap(), rht, op),
            Slash | SlashEq => slash(chunk, lht.unwrap(), rht, op),
            _ => todo!()
        }
    }
}

// bp stands for binding power

fn prefix_bp(op: TokenType) -> ((), u8) {
    match op {
        Plus | Minus => ((), 9),
        _ => panic!("bad op: {:?}", op),
    }
}
fn postfix_bp(op: TokenType) -> Option<(u8, ())> {
    let res = match op {
//        '[' => (11, ()),
        _ => return None,
    };
    Some(res)
}
fn infix_bp(op: TokenType) -> Option<(u8, u8)> {
    let res = match op {
        Eq | PlusEq | MinusEq | StarEq | SlashEq => (2, 1),
        Identifier => (4, 3),
        Plus | Minus => (5, 6),
        Star | Slash => (7, 8),
        Dot => (14, 13),
        _ => return None,
    };
    Some(res)
}






