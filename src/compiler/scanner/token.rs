use std::ops::RangeBounds;


pub struct Token {
    ty: TokenType,
    lexeme: Option<String>,
    line: u16,
    pos: (u64, u64)
}

pub enum TokenType {
    LParen, RParen, LBrace, RBrace, LSquare, RSquare,
    Comma, Dot, SemiColon,
    Plus, Minus, Star, Slash,
    Ampersand, Caret, Bar, Pipe, Hash,
    IndentUp, IndentDown,

    Bang, BangEq,
    Eq, EqEq,
    More, MoreEq,
    Less, LessEq,

    Identifier, String, Int, Dec,

    And, Alias, As, Else, False, Fn, If, Let, Loop, Macro, Mod, Mut, Or, Print, Pub, Return, Selff, Struct, Super, Trait, True, While,
}
