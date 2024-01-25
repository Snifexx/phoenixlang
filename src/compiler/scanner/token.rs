use std::collections::HashMap;
use std::string::String;
use std::ops::RangeBounds;
use TokenType::*;
use rustc_hash::FxHashMap;

use super::Scanner;

pub struct Token {
    pub ty: TokenType,
    pub lexeme: Option<String>,
    pub pos: (u16, u16)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LParen = 0, RParen, LBrace, RBrace, LSquare, RSquare,
    Comma, Dot, SemiColon,
    Plus, Minus, Arrow, Star, Slash,
    Ampersand, Caret, Bar, Pipe, Hash,
    IndentUp, IndentDown,

    Bang, BangEq,
    Eq, EqEq, ArrowEq,
    More, MoreEq,
    Less, LessEq,

    Identifier, String, Int, Dec,

    And, Alias, As, Else, False, Fn, If, Infix, Let, Loop,
    Macro, Mod, Mut, Or, Prefix, Print, Pub, Return, Selff,
    Struct, Super, Suffix, Trait, True, While,
    Eof,
}

impl Token {
    pub fn make(scanner: &Scanner, ty: TokenType, lexeme: Option<String>) -> Token { Token { ty, lexeme: lexeme, pos: (scanner.row, scanner.col) }}
}
