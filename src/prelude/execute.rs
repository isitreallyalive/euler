use color_eyre::Result;
use std::fmt;

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Return {
    u32(u32),
    u64(u64),
    i32(i32),
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Return::u32(u32) => u32.to_string(),
                Return::u64(u64) => u64.to_string(),
                Return::i32(i32) => i32.to_string(),
            }
        )
    }
}

pub trait Execute {
    fn execute(&self) -> Result<Return>;
}
