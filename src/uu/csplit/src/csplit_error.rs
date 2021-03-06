// clippy bug https://github.com/rust-lang/rust-clippy/issues/7422
#![allow(clippy::nonstandard_macro_braces)]

use std::io;
use thiserror::Error;

/// Errors thrown by the csplit command
#[derive(Debug, Error)]
pub enum CsplitError {
    #[error("IO error: {}", _0)]
    IoError(io::Error),
    #[error("'{}': line number out of range", _0)]
    LineOutOfRange(String),
    #[error("'{}': line number out of range on repetition {}", _0, _1)]
    LineOutOfRangeOnRepetition(String, usize),
    #[error("'{}': match not found", _0)]
    MatchNotFound(String),
    #[error("'{}': match not found on repetition {}", _0, _1)]
    MatchNotFoundOnRepetition(String, usize),
    #[error("line number must be greater than zero")]
    LineNumberIsZero,
    #[error("line number '{}' is smaller than preceding line number, {}", _0, _1)]
    LineNumberSmallerThanPrevious(usize, usize),
    #[error("invalid pattern: {}", _0)]
    InvalidPattern(String),
    #[error("invalid number: '{}'", _0)]
    InvalidNumber(String),
    #[error("incorrect conversion specification in suffix")]
    SuffixFormatIncorrect,
    #[error("too many % conversion specifications in suffix")]
    SuffixFormatTooManyPercents,
}

impl From<io::Error> for CsplitError {
    fn from(error: io::Error) -> Self {
        CsplitError::IoError(error)
    }
}
