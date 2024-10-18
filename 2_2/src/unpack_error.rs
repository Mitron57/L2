use std::error::Error;
use std::fmt::{Debug, Display};

pub enum UnpackError {
    StartsWithNumber(char),
    InvalidEscaping(String),
    InvalidRepeatsNumber(String),
}

impl Display for UnpackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnpackError::StartsWithNumber(num) => {
                write!(f, "String starts with a number: {num}")
            }
            UnpackError::InvalidEscaping(escape) => {
                write!(f, "String contains invalid escaping symbol: {escape}")
            }
            UnpackError::InvalidRepeatsNumber(repeats) => {
                write!(
                    f,
                    "String contains invalid number of repetitions: {repeats}"
                )
            }
        }
    }
}

impl Debug for UnpackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Error for UnpackError {}
