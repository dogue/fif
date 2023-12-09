use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum LexerError {
    ReadNumberError(String),
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::ReadNumberError(s) => write!(f, "could not read number: {s}"),
        }
    }
}

impl Error for LexerError {}
