use std::fmt::Debug;
use std::fmt::Display;

#[derive(Clone, Hash, Eq, PartialEq)]
pub enum AocOption {
    Str(String),
    Int(i64),
    None,
}

impl From<String> for AocOption {
    fn from(value: String) -> Self {
        AocOption::Str(value)
    }
}

// Implement for numeric types
macro_rules! impl_from_number {
    ($($t:ty),*) => {
        $(
            impl From<$t> for AocOption {
                fn from(value: $t) -> Self {
                    AocOption::Int(value as i64)
                }
            }
        )*
    }
}

impl_from_number!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

impl<T: Into<AocOption>> From<Option<T>> for AocOption {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(value) => value.into(),
            None => AocOption::None,
        }
    }
}

impl Debug for AocOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocOption::Str(s) => write!(f, "Str: {}", s),
            AocOption::Int(i) => write!(f, "Int: {}", i),
            AocOption::None => write!(f, "None"),
        }
    }
}

impl Display for AocOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AocOption::Str(s) => write!(f, "{}", s),
            AocOption::Int(i) => write!(f, "{}", i),
            AocOption::None => write!(f, "None"),
        }
    }
}
