use std::{
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int(isize),
    Float(f32),
    Str(String),
    Invalid,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseTypeError(String);

impl FromStr for Type {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('.') {
            let float = s
                .parse::<f32>()
                .map_err(|e| ParseTypeError(e.to_string()))?;
            Ok(Type::Float(float))
        } else {
            let int = s
                .parse::<isize>()
                .map_err(|e| ParseTypeError(e.to_string()))?;
            Ok(Type::Int(int))
        }
    }
}

impl Add for Type {
    type Output = Type;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a + b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a + b),
            (Type::Str(a), Type::Str(b)) => Type::Str(format!("{a}{b}")),
            _ => Type::Invalid,
        }
    }
}

impl Sub for Type {
    type Output = Type;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a - b),
            (Type::Float(a), Type::Float(b)) => Type::Float(a - b),
            _ => Type::Invalid,
        }
    }
}
