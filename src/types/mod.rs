pub mod display;
mod input;
mod options;
mod parts;
mod result;

pub use input::PuzzleInput;
pub use options::AocOption;
pub use parts::Parts;
pub use result::{AocResult, AocYear};

// Re-export common traits
pub use std::fmt::{Debug, Display};
