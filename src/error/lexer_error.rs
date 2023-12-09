use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    ReadIdentError(String),
    ReadNumberError(String),
    ReadStringError(String),
    ReadCommentError(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::ReadIdentError(s) => write!(f, "could not read ident: {s}"),
            LexerError::ReadNumberError(s) => write!(f, "could not read number: {s}"),
            LexerError::ReadStringError(s) => write!(f, "could not read string: {s}"),
            LexerError::ReadCommentError(s) => write!(f, "could not read comment: {s}"),
        }
    }
}

impl Error for LexerError {}
