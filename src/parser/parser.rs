use crate::{
    error::ParserError,
    lexer::{Token, TokenType},
    types::{Type, Variable},
};

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_var(&self, ident: &str, value: &Token) -> Result<Variable, ParserError> {
        match value.token_type {
            TokenType::Number => {
                let val = value.literal.parse().unwrap();
                Ok(Variable::new(ident, val))
            }
            TokenType::String => {
                let t = Type::Str(value.literal.clone());
                Ok(Variable::new(ident, t))
            }
            _ => Err(ParserError::InvalidVariableValue {
                line: value.line,
                column: value.column,
            }),
        }
    }

    pub fn parse_type(&self, token: &Token) -> Option<Type> {
        match token.token_type {
            TokenType::Number => Some(token.literal.parse().unwrap()),
            TokenType::String => Some(Type::Str(token.literal.clone())),
            _ => None,
        }
    }
}
