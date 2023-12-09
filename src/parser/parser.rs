use crate::{
    lexer::{Token, TokenType},
    types::Type,
};

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_type(&self, token: &Token) -> Option<Type> {
        match token.token_type {
            TokenType::Number => Some(token.literal.parse().unwrap()),
            TokenType::String => Some(Type::Str(token.literal.clone())),
            _ => None,
        }
    }
}
