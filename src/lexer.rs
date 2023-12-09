use crate::error::LexerError;
use crate::types::Type;

#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    cursor: usize,
    peek: usize,
    ch: u8,
    // line: usize,
    // column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.as_bytes().to_vec(),
            cursor: 0,
            peek: 0,
            ch: 0,
            // line: 1,
            // column: 1,
        };

        lexer.read();

        lexer
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();
        loop {
            let token = self.next_token()?;
            tokens.push(token.clone());
            if token.token_type == TokenType::Eof {
                break;
            }
        }
        Ok(tokens)
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.skip_whitespace();

        let token = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident()?;
                match ident.as_str() {
                    "swap" => return Ok(Token::new(&ident, TokenType::Swap)),
                    "dupe" => return Ok(Token::new(&ident, TokenType::Dupe)),
                    "var" => return Ok(Token::new(&ident, TokenType::Var)),
                    _ => return Ok(Token::new(&ident, TokenType::Ident)),
                }
            }
            b'0'..=b'9' => {
                let num = self.read_number()?;
                return Ok(Token::new(&num, TokenType::Number));
            }
            b'"' => {
                self.read(); // consume first double quote
                let string = self.read_str()?;
                println!("read str: {string}");
                Token::new(&string, TokenType::String)
            }
            b'+' => Token::new("+", TokenType::Add),
            b'-' => Token::new("-", TokenType::Sub),
            b'*' => Token::new("*", TokenType::Mul),
            b'/' => {
                if self.peek() == b'/' {
                    let comment = self.read_comment()?;
                    Token::new(&comment, TokenType::Comment)
                } else {
                    Token::new("/", TokenType::Div)
                }
            }
            0 => Token::new("\0", TokenType::Eof),
            _ => Token::new(&self.ch.to_string(), TokenType::Invalid),
        };

        self.read();

        Ok(token)
    }

    fn read(&mut self) {
        if self.peek >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input[self.peek];
        }

        self.cursor = self.peek;
        self.peek += 1;
    }

    fn read_ident(&mut self) -> Result<String, LexerError> {
        let start = self.cursor;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read();
        }

        let ident = self.input[start..self.cursor].to_vec();
        String::from_utf8(ident).map_err(|e| LexerError::ReadIdentError(e.to_string()))
    }

    fn read_number(&mut self) -> Result<String, LexerError> {
        let start = self.cursor;
        while self.ch.is_ascii_digit() || self.ch == b'.' {
            self.read();
        }

        if !self.input[self.cursor].is_ascii_whitespace() {
            return Err(LexerError::ReadNumberError(
                "identifiers cannot start with a number".to_string(),
            ));
        }

        let number = self.input[start..self.cursor].to_vec();
        String::from_utf8(number).map_err(|e| LexerError::ReadNumberError(e.to_string()))
    }

    fn read_str(&mut self) -> Result<String, LexerError> {
        let start = self.cursor;
        while self.ch != b'"' {
            self.read();
        }

        let string = self.input[start..self.cursor].to_vec();
        String::from_utf8(string).map_err(|e| LexerError::ReadStringError(e.to_string()))
    }

    fn read_comment(&mut self) -> Result<String, LexerError> {
        let start = self.cursor;
        while self.ch != b'\n' {
            self.read();
        }

        let comment = self.input[start..self.cursor].to_vec();
        String::from_utf8(comment).map_err(|e| LexerError::ReadCommentError(e.to_string()))
    }

    fn peek(&self) -> u8 {
        if self.peek >= self.input.len() {
            0
        } else {
            self.input[self.peek]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read();
        }
    }

    pub fn parse_number(s: &str) -> Type {
        s.parse().unwrap()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub literal: String,
    pub token_type: TokenType,
}

impl Token {
    pub fn new(literal: &str, token_type: TokenType) -> Self {
        Self {
            literal: literal.to_string(),
            token_type,
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
    Var,
    Add,
    Sub,
    Mul,
    Div,
    Comment,
    Invalid,
    Eof,
}
