use std::collections::HashMap;
use std::string::String;
use std::ops::RangeBounds;
use TokenType::*;
use rustc_hash::FxHashMap;

use crate::compiler::scanner::Scanner;

#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: Option<String>,
    pub pos: (u16, u16)
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TokenType {
    LParen = 0, RParen, LBrace, RBrace, LSquare, RSquare,
    Comma, Dot, SemiColon,

    Plus, PlusEq,
    Minus, MinusEq, Arrow,
    Star, StarEq, 
    Slash, SlashEq,

    Ampersand, Caret, Bar, Pipe, Hash,
    IndentUp, IndentDown,

    Bang, BangEq,
    Eq, EqEq, ArrowEq,
    More, MoreEq,
    Less, LessEq,

    Identifier, String, Int, Dec,

    And, Alias, As, Else, False, Fn, If, Infix, Let, Loop,
    Macro, Mod, Mut, Not, Or, Prefix, Print, Pub, Return, Selff,
    Struct, Super, Suffix, Trait, True, While, Xor,
    Eof,
}

impl Token {
    pub fn make(scanner: &Scanner, ty: TokenType, lexeme: Option<String>) -> Token { Token { ty, lexeme: lexeme, pos: (scanner.row, scanner.col) }}
}
