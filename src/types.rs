use std::{
    num::{ParseFloatError, ParseIntError},
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

#[derive(Default, Debug, PartialEq, Clone)]
pub enum Type {
    Int(isize),
    Float(f32),
    Str(String),
    #[default]
    Nil,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable {
    pub value: Type,
    pub ident: String,
}

impl Variable {
    pub fn new(ident: &str, value: Type) -> Self {
        Self {
            ident: ident.to_string(),
            value,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseTypeError {
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    MalformedNumber,
}
// pub struct ParseTypeError(String);

impl FromStr for Type {
    type Err = ParseTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.matches('.').count() {
            0 => {
                let int = s
                    .parse::<isize>()
                    .map_err(|e| ParseTypeError::ParseIntError(e))?;
                Ok(Type::Int(int))
            }
            1 => {
                let float = s
                    .parse::<f32>()
                    .map_err(|e| ParseTypeError::ParseFloatError(e))?;
                Ok(Type::Float(float))
            }
            _ => Err(ParseTypeError::MalformedNumber),
        }
    }
}

impl Add for Type {
    type Output = Type;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a + b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f32 + b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a + (b as f32)),
            (Type::Float(a), Type::Float(b)) => Type::Float(a + b),
            (Type::Str(a), Type::Str(b)) => Type::Str(format!("{a}{b}")),
            _ => Type::Nil,
        }
    }
}

impl Sub for Type {
    type Output = Type;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a - b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f32 - b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a - (b as f32)),
            (Type::Float(a), Type::Float(b)) => Type::Float(a - b),
            _ => Type::Nil,
        }
    }
}

impl Mul for Type {
    type Output = Type;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a * b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f32 * b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a * (b as f32)),
            (Type::Float(a), Type::Float(b)) => Type::Float(a * b),
            (Type::Str(s), Type::Int(r)) => Type::Str(s.repeat(r as usize)),
            _ => Type::Nil,
        }
    }
}

impl Div for Type {
    type Output = Type;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Type::Int(a), Type::Int(b)) => Type::Int(a / b),
            (Type::Int(a), Type::Float(b)) => Type::Float(a as f32 / b),
            (Type::Float(a), Type::Int(b)) => Type::Float(a / (b as f32)),
            (Type::Float(a), Type::Float(b)) => Type::Float(a / b),
            _ => Type::Nil,
        }
    }
}
