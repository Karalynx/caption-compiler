
use core::fmt;
use std::borrow::Cow;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenKind {
    OpenBracket,
    CloseBracket,
    String
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            Self::OpenBracket => "Open Bracket",
            Self::CloseBracket => "Close Bracket",
            Self::String => "String"
        };
        write!(f, "{text}")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Token<'src> {
    pub kind: TokenKind,
    pub lexeme: Cow<'src, str>,
    pub line: usize
}

impl<'src> fmt::Display for Token<'src> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} \"{}\" at line {}", self.kind, self.lexeme, self.line)
    }
}
