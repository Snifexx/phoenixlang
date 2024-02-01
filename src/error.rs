use std::fmt::{Display, write};



#[derive(Debug)]
pub enum PhoenixError {
    Compile { id: CompErrID, row: u16, col: u16, msg: String },
    Runtime(String)
}

impl Display for PhoenixError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PhoenixError::Compile { id, row, col, msg } => write!(f, "Compile error of type {id:?} at {row}::{col}: \n{msg}"),
            PhoenixError::Runtime(msg) => write!(f, "Program panicked! {msg}")
        }
    }
}

#[derive(Debug)]
pub enum CompErrID {
    InvalidChar, UnterminatedComment, UnterminatedString, IdentifierTooLong,
    TypeError,
}
