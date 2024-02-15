use std::fmt::{Display, write, Debug};

use clap::error::ErrorKind;



//#[derive(Debug)]
pub enum PhoenixError {
    Cli(ErrorKind, String),
    Config(String),
    Compile { id: CompErrID, row: u16, col: u16, msg: String },
    Runtime(String)
}

impl Debug for PhoenixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoenixError::Compile { id, row, col, msg } => write!(f, "Compile error of type {id:?} at {row}::{col}: \n{msg}"),
            PhoenixError::Runtime(msg) => write!(f, "Program panicked! {msg}"),
            PhoenixError::Cli(_, msg) | PhoenixError::Config(msg) => write!(f, "{msg}"),
        }
    }
}

#[derive(Debug)]
pub enum CompErrID {
    // Feather.toml errors
    ConfigError,
    // Scanner errors
    InvalidCharacter, UnterminatedComment, UnterminatedString, UnterminatedChar, InvalidCharLiteral,
    IdentifierTooLong, 
    // Compiler errors
    TypeError, InvalidSymbol,
    MissingGlobalSymbol,
}
