use crate::parser::Rule;
use std::error::Error;
use std::fmt;
use std::result;

pub type LiResult<T> = result::Result<T, LisperError>;

// TODO: use anyhow?
// https://crates.io/crates/anyhow
#[derive(Debug)]
pub enum LisperError {
    Parser(pest::error::Error<Rule>),
    ZeroDiv,
    SymbolNotFound,
}

impl fmt::Display for LisperError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LisperError::Parser(ref err) => err.fmt(f),
            LisperError::ZeroDiv => write!(f, "Attempt to divide by zero!"),
            LisperError::SymbolNotFound => write!(f, "S-expression Does not start with symbol!"),
        }
    }
}

impl Error for LisperError {}

impl From<pest::error::Error<Rule>> for LisperError {
    fn from(err: pest::error::Error<Rule>) -> LisperError {
        LisperError::Parser(err)
    }
}
