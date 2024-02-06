use core::panic;
use std::{string::String, any::TypeId};
use std::rc::Rc;
use clap::builder::Str;
use rustc_hash::{FxHashMap, FxHashSet};
use crate::{error::PhoenixError, debug::debug_chunk};

use self::logic::{plus, minus, star, slash, negate};
use self::types::Type;

use crate::FBOpCode::*;
use super::token::TokenType;
use super::{token::{Token, self, TokenType::*}, chunk::{Chunk, Const}};

mod types;
mod logic;

pub struct Module {
    id: Rc<String>,
    tokens: Vec<Token>, i: usize,
    imports: FxHashMap<Rc<String>, Rc<String>>,
    items: Vec<Items>,
    // TODO temp
    chunk: Chunk,
}

struct Items {
    name: Rc<String>,
    code: Chunk,
    dependencies: FxHashSet<Rc<String>>,
}

impl Module {
    pub fn new(tokens: Vec<Token>, id: Rc<String>) -> Self { Self { tokens, id, i: 0, imports:  FxHashMap::default(), items: Vec::new(), chunk: Chunk::new() }}
    #[inline(always)]
    pub fn curr_tok(&mut self) -> &mut Token { &mut self.tokens[self.i] }
    
    pub fn compile(mut self, interned_str: &mut FxHashSet<Rc<String>>) -> Result<Chunk, PhoenixError> {
        
        while self.curr_tok().ty != Eof {
            self.expression(0, interned_str)?;
        }

        //debug_chunk(&self.chunk.build());
        Ok(self.chunk)
    }

    pub fn expression(&mut self, min_bp: u8, interned_str: &mut FxHashSet<Rc<String>>) -> Result<Type, PhoenixError> {
        let lht_pos = self.curr_tok().pos;
        let mut lht = match self.curr_tok().ty {
            True | False => self.bool(),
            Int => self.int(),
            Dec => self.dec(), 
            String => {println!("{:?}",self.curr_tok().ty); self.string()} 
            Plus => {
                self.i += 1;
                let ret_ty = self.expression(9, interned_str)?;
                self.i -= 1; 
                ret_ty
            }
            Char => self.char(),
            LParen => {
                self.i += 1;
                let value = self.expression(0, interned_str)?;
                assert_eq!(self.curr_tok().ty, RParen);
                value
            }
            op @ Minus => {
                let ((), r_bp) = prefix_bp(op);
                let tok_i = self.i; self.i += 1;
                let rht_pos = self.curr_tok().pos;
                let rhs = self.expression(r_bp, interned_str)?;
                self.i -= 1;
                Self::operation(&mut self.chunk, None, (rhs, rht_pos), &self.tokens[tok_i])?
            }
            ty => unreachable!("{:?}", ty),
        };
        self.i += 1;

        loop {
            let op_i = self.i;
            if self.tokens[self.i - 1].pos.0 != self.curr_tok().pos.0 { break; }
            let op = match self.curr_tok().ty {
                RParen | Eof => break,
                op @ (Plus | Minus | Star | Slash) => &self.tokens[op_i], 
                op => unreachable!("{op:?}"),
            };
            
            if let Some((l_bp, ())) = postfix_bp(op.ty) { // Postfix
                if l_bp < min_bp {
                    break;
                }
                self.i += 1;

                lht = if op.ty == LSquare {
                    let rht_pos = self.curr_tok().pos;
                    let rhs = self.expression(0, interned_str)?;
                    assert_eq!(self.curr_tok().ty, RSquare);
                    // TODO Lquare get func
                    Type::Void // TODO type calculator
                } else { lht };
                continue;
            }

            if let Some((l_bp, r_bp)) = infix_bp(op.ty) {
                if l_bp < min_bp { break; }
                self.i += 1;

                lht = {
                    let rht_pos = self.curr_tok().pos;
                    let rht = self.expression(r_bp, interned_str)?;
                    let op = &self.tokens[op_i];
                    Self::operation(&mut self.chunk, Some((lht, lht_pos)), (rht, rht_pos), op)?
                };
                continue;
            }

            break
        }

        Ok(lht)
    }
    
    #[inline(always)]
    fn bool(&mut self) -> Type { let op = if self.curr_tok().ty == True { OpTrue } else { OpFalse }; self.chunk.write_op(op); Type::Bool }

    fn int(&mut self) -> Type {
        let num = self.curr_tok().lexeme.take().unwrap().parse::<i64>().unwrap();
        self.chunk.write_const(Const::Int(num));
        Type::Int
    }

    fn dec(&mut self) -> Type {
        let num = self.curr_tok().lexeme.take().unwrap().parse::<f64>().unwrap();
        self.chunk.write_const(Const::Dec(num.to_bits()));
        Type::Dec
    }

    fn string(&mut self) -> Type {
        println!("{:?}", self.curr_tok());
        let str = self.curr_tok().lexeme.take().unwrap();
        self.chunk.write_const(Const::String(str));
        Type::Str
    }

    fn char(&mut self) -> Type {
        let char = self.curr_tok().lexeme.take().unwrap().chars().next().unwrap();
        self.chunk.write_const(Const::Char(char));
        Type::Char
    }

    fn operation(chunk: &mut Chunk, lht: Option<(Type, (u16, u16))>, rht: (Type, (u16, u16)), op: &Token) -> Result<Type, PhoenixError> {
        match op.ty {
            Plus => plus(chunk, lht.unwrap(), rht, op),
            Minus => if lht.is_some() { minus(chunk, lht.unwrap(), rht, op) } else { negate(chunk, rht, op) }
            Star => star(chunk, lht.unwrap(), rht, op),
            Slash => slash(chunk, lht.unwrap(), rht, op),
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






