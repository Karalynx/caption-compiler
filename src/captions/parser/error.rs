
use core::fmt;

use crate::captions::TokenKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaptionParseError {
    UnexpectedToken { line: usize, expected: TokenKind, found: TokenKind },
    LiteralMismatch { line: usize, expected: Box<str>, found: Box<str> },
    UnexpectedEof { expected: TokenKind }
}

impl fmt::Display for CaptionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken { line, expected, found } => write!(
                f,
                "line {line}: expected '{expected}, found {found}"
            ),
            Self::LiteralMismatch { line, expected, found } => write!(
                f,
                "line {line}: expected string to be \"{expected}\", found \"{found}\""
            ),
            Self::UnexpectedEof { expected } => write!(
                f,
                "EOF reached before parsing {expected}"
            )
        }
    }
}
impl<'src> std::error::Error for CaptionParseError {}