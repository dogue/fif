use lexer::{Lexer, TokenType};
use types::Type;

pub mod error;
pub mod lexer;
pub mod types;

#[derive(Debug)]
pub struct FifVM {
    pub stack: Vec<Type>,
}

impl FifVM {
    pub fn new() -> Self {
        Self { stack: Vec::new() }
    }

    pub fn run(&mut self, input: &str) {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.tokenize().unwrap();

        tokens.iter().for_each(|token| match token.token_type {
            TokenType::Number => {
                let num = Lexer::parse_number(&token.literal);
                self.push(num);
            }
            TokenType::String => {
                let str = Type::Str(token.literal.clone());
                self.push(str);
            }
            TokenType::Add => self.add(),
            TokenType::Sub => self.sub(),
            TokenType::Mul => self.mul(),
            TokenType::Div => self.div(),
            TokenType::Eof => {}
            TokenType::Ident => {}
            TokenType::Comment => {}
            TokenType::Invalid => {}
            TokenType::Keyword => match token.literal.as_str() {
                "swap" => self.swap(),
                "dupe" => self.dupe(),
                _ => {}
            },
        });
    }

    fn pop(&mut self) -> Type {
        self.stack.pop().unwrap_or_default()
    }

    fn pop_two(&mut self) -> (Type, Type) {
        (self.pop(), self.pop())
    }

    fn push(&mut self, t: Type) {
        self.stack.push(t);
    }

    fn swap(&mut self) {
        let a = self.pop();
        let b = self.pop();
        self.push(a);
        self.push(b);
    }

    fn dupe(&mut self) {
        if let Some(top) = self.stack.last().cloned() {
            self.stack.push(top);
        }
    }

    fn add(&mut self) {
        let (a, b) = self.pop_two();
        self.push(a + b);
    }

    fn sub(&mut self) {
        let (a, b) = self.pop_two();
        self.push(a - b);
    }

    fn mul(&mut self) {
        let (a, b) = self.pop_two();
        self.push(a * b);
    }

    fn div(&mut self) {
        let (a, b) = self.pop_two();
        self.push(a / b);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_int() {
        let mut vm = FifVM::new();
        vm.push(Type::Int(1));
        vm.push(Type::Int(2));
        vm.add();

        assert_eq!(vm.stack[0], Type::Int(3));
    }

    #[test]
    fn test_add_float() {
        let mut vm = FifVM::new();
        vm.push(Type::Float(1.0));
        vm.push(Type::Float(2.0));
        vm.add();

        assert_eq!(vm.stack[0], Type::Float(3.0));
    }

    #[test]
    fn test_add_str() {
        let mut vm = FifVM::new();
        vm.push(Type::Str("world".to_string()));
        vm.push(Type::Str("hello ".to_string()));
        vm.add();

        assert_eq!(vm.stack[0], Type::Str("hello world".to_string()));
    }

    #[test]
    fn test_sub_int() {
        let mut vm = FifVM::new();
        vm.push(Type::Int(2));
        vm.push(Type::Int(3));
        vm.sub();

        assert_eq!(vm.stack[0], Type::Int(1));
    }

    #[test]
    fn test_sub_float() {
        let mut vm = FifVM::new();
        vm.push(Type::Float(2.0));
        vm.push(Type::Float(3.0));
        vm.sub();

        assert_eq!(vm.stack[0], Type::Float(1.0));
    }

    #[test]
    fn test_sub_str() {
        let mut vm = FifVM::new();
        vm.push(Type::Str("deez".to_string()));
        vm.push(Type::Str("nuts".to_string()));
        vm.sub();

        assert_eq!(vm.stack[0], Type::Invalid);
    }

    #[test]
    fn test_mul_int() {
        let mut vm = FifVM::new();
        vm.push(Type::Int(2));
        vm.push(Type::Int(3));
        vm.mul();

        assert_eq!(vm.stack[0], Type::Int(6));
    }

    #[test]
    fn test_mul_float() {
        let mut vm = FifVM::new();
        vm.push(Type::Float(2.0));
        vm.push(Type::Float(3.0));
        vm.mul();

        assert_eq!(vm.stack[0], Type::Float(6.0));
    }

    #[test]
    fn test_div_int() {
        let mut vm = FifVM::new();
        vm.push(Type::Int(2));
        vm.push(Type::Int(6));
        vm.div();

        assert_eq!(vm.stack[0], Type::Int(3));
    }

    #[test]
    fn test_div_float() {
        let mut vm = FifVM::new();
        vm.push(Type::Float(2.0));
        vm.push(Type::Float(6.0));
        vm.div();

        assert_eq!(vm.stack[0], Type::Float(3.0));
    }

    #[test]
    fn test_swap() {
        let mut vm = FifVM::new();
        vm.push(Type::Int(1));
        vm.push(Type::Int(2));
        vm.swap();

        assert_eq!(vm.stack, vec![Type::Int(2), Type::Int(1)]);
    }

    #[test]
    fn test_swap_empty_stack() {
        let mut vm = FifVM::new();
        vm.swap();

        assert_eq!(vm.stack, vec![Type::Nil, Type::Nil]);
    }

    #[test]
    fn test_duplicate() {
        let mut vm = FifVM::new();
        vm.push(Type::Int(1));
        vm.dupe();

        assert_eq!(vm.stack, vec![Type::Int(1), Type::Int(1)]);
    }
}
