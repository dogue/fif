use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ParserError {
    InvalidVariableValue { line: usize, column: usize },
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::InvalidVariableValue { line: l, column: c } => {
                write!(f, "invalid variable value (line: {l}, column: {c})")
            }
        }
    }
}

impl Error for ParserError {}
