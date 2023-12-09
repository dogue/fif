#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
}

impl Token {
    pub fn new(literal: &str, token_type: TokenType, line: usize, column: usize) -> Self {
        Self {
            literal: literal.to_string(),
            token_type,
            line,
            column,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number,
    String,
    Ident,
    Swap,
    Dupe,
    Print,
    Debug,
    Drop,
    Var,
    Add,
    Sub,
    Mul,
    Div,
    Comment,
    Invalid,
    Eof,
}
