use crate::error::LexerError;

use super::{Token, TokenType};

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    cursor: usize,
    peek: usize,
    ch: char,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.chars().collect(),
            cursor: 0,
            peek: 0,
            ch: '\0',
            line: 1,
            column: 1,
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

        let line = self.line;
        let column = self.column;

        let token = match self.ch {
            'a'..='z' | 'A'..='Z' | '_' => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "swap" => return Ok(Token::new(&ident, TokenType::Swap, line, column)),
                    "dupe" => return Ok(Token::new(&ident, TokenType::Dupe, line, column)),
                    "print" => return Ok(Token::new(&ident, TokenType::Print, line, column)),
                    "debug" => return Ok(Token::new(&ident, TokenType::Debug, line, column)),
                    "drop" => return Ok(Token::new(&ident, TokenType::Drop, line, column)),
                    "var" => return Ok(Token::new(&ident, TokenType::Var, line, column)),
                    _ => return Ok(Token::new(&ident, TokenType::Ident, line, column)),
                }
            }
            '0'..='9' => {
                let num = self.read_number()?;
                return Ok(Token::new(&num, TokenType::Number, line, column));
            }
            '"' => {
                self.read(); // consume first double quote
                let string = self.read_str();
                Token::new(&string, TokenType::String, line, column)
            }
            '+' => Token::new("+", TokenType::Add, line, column),
            '-' => Token::new("-", TokenType::Sub, line, column),
            '*' => Token::new("*", TokenType::Mul, line, column),
            '/' => {
                if self.peek() == '/' {
                    let comment = self.read_comment();
                    Token::new(&comment, TokenType::Comment, line, column)
                } else {
                    Token::new("/", TokenType::Div, line, column)
                }
            }
            '\0' => Token::new("\0", TokenType::Eof, line, column),
            _ => Token::new(&self.ch.to_string(), TokenType::Invalid, line, column),
        };

        self.read();

        Ok(token)
    }

    fn read(&mut self) {
        if self.peek >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.peek];
        }

        self.cursor = self.peek;
        self.peek += 1;

        if self.ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
    }

    fn read_ident(&mut self) -> String {
        let start = self.cursor;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read();
        }

        let ident = self.input[start..self.cursor].to_vec();
        ident.into_iter().collect()
    }

    fn read_number(&mut self) -> Result<String, LexerError> {
        let start = self.cursor;
        let line = self.line;
        let column = self.column;
        while self.ch.is_digit(10) || self.ch == '.' {
            self.read();
        }

        if !self.input[self.cursor].is_whitespace() {
            return Err(LexerError::ReadNumberError(format!(
                "identifiers cannot start with a number (line: {}, column: {})",
                line, column
            )));
        }

        let number = self.input[start..self.cursor].to_vec();
        Ok(number.into_iter().collect())
    }

    fn read_str(&mut self) -> String {
        let start = self.cursor;
        while self.ch != '"' {
            self.read();
        }

        let string = self.input[start..self.cursor].to_vec();
        string.into_iter().collect()
    }

    fn read_comment(&mut self) -> String {
        let start = self.cursor;
        while self.ch != '\n' {
            self.read();
        }

        let comment = self.input[start..self.cursor].to_vec();
        comment.into_iter().collect()
    }

    fn peek(&self) -> char {
        if self.peek >= self.input.len() {
            '\0'
        } else {
            self.input[self.peek]
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read();
        }
    }
}
