use super::Debug;
use super::Display;

#[derive(Clone)]
pub enum AocOption {
    Str(String),
    Int(i32),
    BigInt(i64),
    None,
}

impl Debug for AocOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocOption::Str(s) => write!(f, "Str: {}", s),
            AocOption::Int(i) => write!(f, "Int: {}", i),
            AocOption::BigInt(i) => write!(f, "BigInt: {}", i),
            AocOption::None => write!(f, "None"),
        }
    }
}

impl Display for AocOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocOption::Str(s) => write!(f, "{}", s),
            AocOption::Int(i) => write!(f, "{}", i),
            AocOption::BigInt(i) => write!(f, "{}", i),
            AocOption::None => write!(f, "None"),
        }
    }
}
