#[derive(Debug)]
pub struct Lexer {
    input: Vec<u8>,
    cursor: usize,
    peek: usize,
    ch: u8,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        let mut lexer = Self {
            input: input.as_bytes().to_vec(),
            cursor: 0,
            peek: 0,
            ch: 0,
            line: 1,
            column: 1,
        };

        lexer.read();

        lexer
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let token = match self.ch {
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let ident = self.read_ident();
                return Token::Ident(ident);
            }
            b'0'..=b'9' => {
                let num = self.read_number();
                return Self::parse_number(&num);
            }
            b'"' => {
                self.read(); // consume first double quote
                let str = self.read_str();
                Token::Str(str)
            }
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

    fn parse_number(s: &str) -> Token {
        if s.contains('.') {
            let num = s.parse::<f32>().unwrap_or_default();
            Token::Float(num)
        } else {
            let num = s.parse::<isize>().unwrap_or_default();
            Token::Int(num)
        }
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
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Int(isize),
    Float(f32),
    Str(String),
    Ident(String),
    Operator(String),
    Comment(String),
    Invalid,
    Eof,
}
