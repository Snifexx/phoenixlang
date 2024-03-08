use std::collections::HashMap;
use std::string::String;
use std::ops::RangeBounds;
use TokenType::*;

use crate::compiler::scanner::Scanner;

#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: Option<Box<str>>,
    pub pos: (u16, u16)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenType {
    LParen = 0, RParen, LBrace, RBrace, LSquare, RSquare,
    Comma, Dot, Colon, SemiColon,

    Plus, PlusEq,
    Minus, MinusEq, Arrow,
    Star, StarEq, 
    Slash, SlashEq,

    Ampersand, Caret, Bar, Pipe,
    Hash, Dollar, Tilde, Backtick,
    IndentUp, IndentDown, 

    Bang, BangEq,
    Eq, EqEq, ArrowEq,
    More, MoreEq,
    Less, LessEq,

    Identifier, String, Int, Dec, Char,

    And, Alias, As, Else, False, Fn, If, Infix, Let, Loop,
    Macro, Mod, Mut, Not, Or, Print, Pub, Return, Selff,

    Struct, Super, Trait, True, While, Xor,
    Eof,
}

impl Token {
    pub fn make(scanner: &Scanner, ty: TokenType, lexeme: Option<&str>) -> Token { Token { ty, lexeme: lexeme.map(|str| str.into()), pos: (scanner.row, scanner.col) }}
    pub fn make_pos(ty: TokenType, lexeme: Option<&str>, pos: (u16, u16)) -> Token { Token { ty, lexeme: lexeme.map(|str| str.into()), pos } }
}








