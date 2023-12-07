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

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                match ident.as_str() {
                    "swap" => return Token::Operator(Operator::Swap),
                    "dupe" => return Token::Operator(Operator::Dupe),
                    _ => return Token::Ident(ident),
                }
            }
            b'0'..=b'9' => {
                let num = self.read_number();
                return Token::Number(num);
            }
            b'"' => {
                self.read(); // consume first double quote
                let str = self.read_str();
                Token::Str(str)
            }
            b'+' => Token::Operator(Operator::Add),
            b'-' => Token::Operator(Operator::Sub),
            b'*' => Token::Operator(Operator::Mul),
            b'/' => Token::Operator(Operator::Div),
            0 => Token::Eof,
            _ => Token::Invalid,
        };

        self.read();

        token
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

    fn read_ident(&mut self) -> String {
        let start = self.cursor;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read();
        }

        // this is peak rust type nonsense :)
        String::from_utf8_lossy(&self.input[start..self.cursor]).to_string()
    }

    fn read_number(&mut self) -> String {
        let start = self.cursor;
        while self.ch.is_ascii_digit() || self.ch == b'.' {
            self.read();
        }

        String::from_utf8_lossy(&self.input[start..self.cursor]).to_string()
    }

    fn read_str(&mut self) -> String {
        let start = self.cursor;
        while self.ch != b'"' {
            self.read();
        }

        String::from_utf8_lossy(&self.input[start..self.cursor]).to_string()
    }

    fn _peek(&self) -> u8 {
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
        if s.contains('.') {
            let num = s.parse::<f32>().unwrap_or_default();
            Type::Float(num)
        } else {
            let num = s.parse::<isize>().unwrap_or_default();
            Type::Int(num)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(String),
    Str(String),
    Ident(String),
    Operator(Operator),
    Comment(String),
    Invalid,
    Eof,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Swap,
    Dupe,
}
