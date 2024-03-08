use std::{string::String, u8, collections::HashMap, hash::BuildHasherDefault, default};
use core::panic;
use std::str::Chars;

use ahash::AHasher;

use crate::{utils::{StringExt, OwnedChars}, error::{PhoenixError, CompErrID}, IDENTIFIER_MAX_LENGTH};
use crate::compiler::token::{Token, TokenType::{*, self}};

type AHashMap<K, V> = HashMap<K, V, BuildHasherDefault<AHasher>>;

pub struct Scanner {
    src: OwnedChars, 
    peek: Option<char>, peek_more: Option<char>,
    pub row: u16, pub col: u16, indent: u16,
}


impl Scanner {
    pub fn new(src: String) -> Self { let mut iter = src.into_chars(); Self { peek: iter.next(), peek_more: iter.next(), src: iter, row: 1, col: 0, indent: 0, }}
    fn next(&mut self) -> Option<char> {
        let ret = self.peek; self.peek = self.peek_more; self.peek_more = self.src.next(); 
        if ret.is_some_and(|x| x == '\n') { self.row += 1; self.col = 1 } else { self.col += 1 } 
        ret 
    }
    fn make_double(&mut self, expected: char, single: TokenType, double: TokenType) -> Token {
        if (self.peek.is_some_and(|x| x == expected)) { self.next(); Token::make_pos(double, None, (self.row, self.col - 1)) }
        else { Token::make(&self, single, None) }
    }
    fn next_err(&mut self, err_id: CompErrID, err_msg: String) -> Result<char, PhoenixError> { 
        match self.next() {
            None | Some('\0') => Err(PhoenixError::Compile { id: err_id, row: self.row, col: self.col, msg: err_msg }),
            Some(c) => Ok(c),
        }
    }
    fn expect_char(&mut self, expected: char) -> Result<char, PhoenixError> {
        match self.next() {
            Some(c) if c == expected => Ok(expected),
            Some(c) => Err(PhoenixError::Compile { id: CompErrID::InvalidCharacter, row: self.row, col: self.col, msg: format!("Expected '{expected}', found '{c}'") }),
            None => Err(PhoenixError::Compile { id: CompErrID::InvalidCharacter, row: self.row, col: self.col, msg: format!("Expected '{expected}'") }),
        }
    }

    pub fn scan(mut self) -> Result<Vec<Token>, PhoenixError> {
        let keywords = AHashMap::from_iter(
            [ ("and", And), ("alias", Alias), ("as", As), ("else", Else), ("false", False), ("fn", Fn), ("if", If),
            ("infix", Infix), ("let", Let), ("loop", Loop), ("not", Not), ("macro", Macro), ("mod", Mod), ("mut", Mut),
            ("or", Or), ("pub", Pub), ("return", Return), ("self", Selff), 
            ("struct", Struct), ("super", Super), ("trait", Trait), ("true", True), ("while", While), ("xor", Xor) ]);
        let mut res = vec![];
        let mut c_ = None; let mut c = '0';

        'scanner: loop {
            macro_rules! next_break {
                () => {
                    {
                        c_ = self.next();
                        if (!c_.is_some_and(|x| x != '\0')) { res.push(Token::make(&self, Eof, None)); break 'scanner }
                        c = c_.unwrap();
                    }
                };
            }

            next_break!();

            match c {
                '\n' => {
                    let mut spaces = 0;
                    while self.peek.is_some_and(|x| x == ' ') { spaces += 1; next_break!() }
                    let new_indent = spaces / 4; 
                    let diff = new_indent as i32 - self.indent as i32;
                    let tok = [IndentDown, IndentUp]; let tok_i = diff > 0;
                    (0..diff.abs()).for_each(|_| {
                        if res.last().is_some_and(|x| x.ty == tok[!tok_i as usize]) { res.pop(); }
                        else { res.push(Token::make(&self, tok[tok_i as usize], None)) }
                    });
                    self.indent = new_indent;
                    continue;
                }
                i if i.is_ascii_whitespace() => continue,
                '/' if self.peek.is_some_and(|x| x == '/') => { while self.peek.is_some_and(|x| x != '\n') { next_break!() }; continue; }
                '/' if self.peek.is_some_and(|x| x == '*') => { 
                    let (row, col) = (self.row, self.col);
                    while self.peek.is_some_and(|x| x != '*') && self.peek_more.is_some_and(|x| x != '/') {
                        self.next_err(CompErrID::UnterminatedComment, format!("Missing closing */ for multi-line comment started at {row}::{col}"))?;
                    }; continue; 
                }

                '(' => res.push(Token::make(&self, LParen, None)),
                ')' => res.push(Token::make(&self, RParen, None)),
                '{' => res.push(Token::make(&self, LBrace, None)),
                '}' => res.push(Token::make(&self, RBrace, None)),
                '[' => res.push(Token::make(&self, LSquare, None)),
                ']' => res.push(Token::make(&self, RSquare, None)),
                ':' => res.push(Token::make(&self, Colon, None)),
                ';' => res.push(Token::make(&self, SemiColon, None)),
                ',' => res.push(Token::make(&self, Comma, None)),
                '#' => res.push(Token::make(&self, Hash, None)),
                '$' => res.push(Token::make(&self, Dollar, None)),
                '~' => res.push(Token::make(&self, Tilde, None)),

                '.' => res.push(Token::make(&self, Dot, None)),
                '+' => res.push(self.make_double('=', Plus, PlusEq)),
                '-' => res.push(
                    match self.peek { 
                        Some('=') => { let mut a = Token::make(&self, MinusEq, None); a.pos.1 -= 1; self.next(); a }
                        Some('>') => { let mut a = Token::make(&self, Arrow, None); a.pos.1 -= 1; self.next(); a }
                        _ => Token::make(&self, Minus, None),
                    }),
                '/' => res.push(self.make_double('=', Slash, SlashEq)),
                '*' => res.push(self.make_double('=', Star, StarEq)),

                '&' => res.push(Token::make(&self, Ampersand, None)),
                '^' => res.push(Token::make(&self, Caret, None)),
                '|' => res.push(self.make_double('>', Bar, Pipe)),

                '!' => res.push(self.make_double('=', Bang, BangEq)),
                '=' => res.push(
                    match self.peek { 
                        Some('=') => { let mut a = Token::make(&self, EqEq, None); a.pos.1 -= 1; self.next(); a }
                        Some('>') => { let mut a = Token::make(&self, ArrowEq, None); a.pos.1 -= 1; self.next(); a }
                        _ => Token::make(&self, Eq, None),
                    }),
                '>' => res.push(self.make_double('=', More, MoreEq)),
                '<' => res.push(self.make_double('=', Less, LessEq)),

                '"' => self.string(&mut res)?,
                '\'' => self.char(&mut res)?,
                '`' => res.push(Token::make(&self, Backtick, None)),
                c if c.is_ascii_digit() => self.number(&mut res, c),
                c if c.is_ascii_alphabetic() || c == '_' => self.identifier(&mut res, c, &keywords)?,
                _ => return Err(PhoenixError::Compile { id: CompErrID::InvalidCharacter, row: self.row, col: self.col, msg: format!("Invalid character {c} at {}::{}", self.row, self.col) })
            }
        }
        Ok(res)
    }


    fn string(&mut self, res: &mut Vec<Token>) -> Result<(), PhoenixError> {
        macro_rules! next_string { () => { self.next_err(CompErrID::UnterminatedString, format!("Missing closing \" for string started at {}::{}", self.row, self.col)) };}
        let mut str = String::new();
        let pos = (self.row, self.col - 1);
        loop {
            let mut c = next_string!()?;
            if c == '"' { break }
            match c {
                '"' => break,
                '\\' => c = match next_string!()? { 'n' => '\n', 'r' => '\r', 't' => '\t', '0' => '\0', p => p },
                _ => {}
            }

            str.push(c);
            if c == '\n' { let skip = self.indent as u32 * 4; for _ in 0..skip { next_string!()?;}}
        }
        res.push(Token::make_pos(String, Some(&*str), pos));
        Ok(())
    }

    fn char(&mut self, res: &mut Vec<Token>) -> Result<(), PhoenixError> {
        macro_rules! next_char { () => { self.next_err(CompErrID::UnterminatedChar, format!("Missing closing ' for string started at {}::{}", self.row, self.col)) };}

        let pos = (self.row, self.col - 1);
        let mut c = next_char!()?;

        if c == '\'' { return Err(PhoenixError::Compile { id: CompErrID::InvalidCharLiteral, row: self.row, col: self.row, msg: format!("Char literal cannot be empty") }); }

        match c {
            '\\' => c = match next_char!()? { 'n' => '\n', 'r' => '\r', 't' => '\t', '0' => '\0', p => p },
            '\n' | '\r' | '\t' => return Err(PhoenixError::Compile { id: CompErrID::InvalidCharacter, row: self.row, col: self.col, msg: format!("Cannot use special characters in char literals") }),
            _ => {}
        }

        self.expect_char('\'')?;

        res.push(Token::make_pos(Char, Some(c.encode_utf8(&mut [0])), pos));
        Ok(())
    }

    fn number(&mut self, res: &mut Vec<Token>, c: char) {
        let pos = (self.row, self.col - 1);
        let mut str = String::from(c);
        let mut dot = false;
        loop {
            let c = match self.peek { Some('\0') | None => break, Some(c) => c, };
            match &c {
                '.' => if dot { break; } else { dot = true; }
                c if c.is_ascii_digit() => {}
                _ => break,
            }
            str.push(c);
            self.next();
        }
        res.push(Token::make_pos(if dot { Dec } else { Int }, Some(&*str), pos));
    }
    
    fn identifier(&mut self, res: &mut Vec<Token>, c: char, keywords: &AHashMap<&'static str, TokenType>) -> Result<(), PhoenixError> {
        let pos = (self.row, self.col - 1);
        let mut str = String::from(c);
        let mut can_be_type = true;
        loop {
            let c = match self.peek { Some('\0') | None => break, Some(c) => c, };
            match &c {
                '?' | '!'  => can_be_type = false,
                c if c.is_ascii_digit() => can_be_type = false, 
                '_' => {}
                c if c.is_ascii_alphabetic() => {}
                _ => break,
            }

            if str.len() >= IDENTIFIER_MAX_LENGTH { return Err(PhoenixError::Compile { id: CompErrID::IdentifierTooLong, row: self.row, col: self.col, 
                    msg: format!("A type or an identifier can have a maximum length of {}", IDENTIFIER_MAX_LENGTH)})}
            str.push(c);
            self.next();
        }
        
        let is_keyword = if can_be_type { keywords.get(&*str) } else { None };
        let str = format!("{}{str}", can_be_type as u8);
        res.push(Token::make_pos(if is_keyword.is_some() { *is_keyword.unwrap() } else { Identifier }, if is_keyword.is_none() { Some(&str) } else { None }, pos));
        Ok(())
    }
}







