
use std::{borrow::Cow, iter::Peekable, slice::Iter};

use crate::captions::{Token, TokenKind, parser::CaptionParseError};

#[derive(Debug, Clone)]
pub struct CaptionParser<'token, 'src> {
    iter: Peekable<Iter<'token, Token<'src>>>
}

impl<'token, 'src> CaptionParser<'token, 'src> {
    #[inline]
    pub fn new(tokens: &'token [Token<'src>]) -> Self {
        Self { iter: tokens.iter().peekable() }
    }

    pub fn peek(&mut self) -> Option<TokenKind> {
        self.iter.peek().map(|x| x.kind)
    }

    pub fn expect_token(&mut self, kind: TokenKind) -> Result<(), CaptionParseError> {
        match self.iter.next() {
            Some(token) => {
                if kind == token.kind {
                    return Ok(());
                }

                Err(CaptionParseError::UnexpectedToken { line: token.line, expected: kind, found: token.kind })
            },
            None => Err(CaptionParseError::UnexpectedEof { expected: kind })
        }
    }

    pub fn expect_literal(&mut self, s: &'src str) -> Result<(), CaptionParseError> {
        match self.iter.next() {
            Some(token) => {
                if TokenKind::String == token.kind && s == token.lexeme {
                    return Ok(());
                }

                Err(CaptionParseError::LiteralMismatch { line: token.line, expected: s.into(), found: token.lexeme.clone().into() })
            },
            None => Err(CaptionParseError::UnexpectedEof { expected: TokenKind::String })
        }
    }

    pub fn consume_literal(&mut self) -> Result<Cow<'src, str>, CaptionParseError> {
        match self.iter.next() {
            Some(token) => {
                if let TokenKind::String = token.kind {
                    return Ok(token.lexeme.clone());
                }

                Err(CaptionParseError::UnexpectedToken { line: token.line, expected: TokenKind::String, found: token.kind })
            },
            None => Err(CaptionParseError::UnexpectedEof { expected: TokenKind::String })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expect_token() {
        let open_bracket_tokens = [
            Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 1 }
        ];
        let open_bracket = CaptionParser::new(&open_bracket_tokens);

        let close_bracket_tokens = [
            Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 1 }
        ];
        let close_bracket = CaptionParser::new(&close_bracket_tokens);

        let string_tokens = [
            Token { kind: TokenKind::String, lexeme: "Test".into(), line: 1 }
        ];
        let string = CaptionParser::new(&string_tokens);
        
        let empty = CaptionParser::new(&[]);

        assert_eq!(
            open_bracket.clone().expect_token(TokenKind::OpenBracket),
            Ok(())
        );
        assert_eq!(
            close_bracket.clone().expect_token(TokenKind::CloseBracket),
            Ok(())
        );
        assert_eq!(
            string.clone().expect_token(TokenKind::String),
            Ok(())
        );

        assert_eq!(
            open_bracket.clone().expect_token(TokenKind::String),
            Err(CaptionParseError::UnexpectedToken { line: 1, expected: TokenKind::String, found: TokenKind::OpenBracket })
        );
        assert_eq!(
            open_bracket.clone().expect_token(TokenKind::CloseBracket),
            Err(CaptionParseError::UnexpectedToken { line: 1, expected: TokenKind::CloseBracket, found: TokenKind::OpenBracket })
        );
        assert_eq!(
            close_bracket.clone().expect_token(TokenKind::String),
            Err(CaptionParseError::UnexpectedToken { line: 1, expected: TokenKind::String, found: TokenKind::CloseBracket })
        );
        assert_eq!(
            close_bracket.clone().expect_token(TokenKind::OpenBracket),
            Err(CaptionParseError::UnexpectedToken { line: 1, expected: TokenKind::OpenBracket, found: TokenKind::CloseBracket })
        );
        assert_eq!(
            string.clone().expect_token(TokenKind::OpenBracket),
            Err(CaptionParseError::UnexpectedToken { line: 1, expected: TokenKind::OpenBracket, found: TokenKind::String })
        );
        assert_eq!(
            string.clone().expect_token(TokenKind::CloseBracket),
            Err(CaptionParseError::UnexpectedToken { line: 1, expected: TokenKind::CloseBracket, found: TokenKind::String })
        );

        assert_eq!(
            empty.clone().expect_token(TokenKind::String),
            Err(CaptionParseError::UnexpectedEof { expected: TokenKind::String })
        );
    }

    #[test]
    fn expect_literal() {
        let open_bracket_tokens = [
            Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 1 }
        ];
        let mut open_bracket = CaptionParser::new(&open_bracket_tokens);

        let close_bracket_tokens = [
            Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 1 }
        ];
        let mut close_bracket = CaptionParser::new(&close_bracket_tokens);

        let string_correct_tokens = [
            Token { kind: TokenKind::String, lexeme: "Test".into(), line: 1 }
        ];
        let mut string_correct = CaptionParser::new(&string_correct_tokens);

        let string_incorrect_tokens = [
            Token { kind: TokenKind::String, lexeme: "Tes".into(), line: 1 }
        ];
        let mut string_incorrect = CaptionParser::new(&string_incorrect_tokens);

        let mut empty = CaptionParser::new(&[]);

        assert_eq!(
            string_correct.expect_literal("Test"),
            Ok(())
        );

        assert_eq!(
            string_incorrect.expect_literal("Test"),
            Err(CaptionParseError::LiteralMismatch { line: 1, expected: "Test".into(), found: "Tes".into() })
        );
        assert_eq!(
            open_bracket.expect_literal("Test"),
            Err(CaptionParseError::LiteralMismatch { line: 1, expected: "Test".into(), found: "{".into() })
        );
        assert_eq!(
            close_bracket.expect_literal("Test"),
            Err(CaptionParseError::LiteralMismatch { line: 1, expected: "Test".into(), found: "}".into() })
        );
        assert_eq!(
            empty.expect_literal("Test"),
            Err(CaptionParseError::UnexpectedEof { expected: TokenKind::String })
        );
    }

    #[test]
    fn consume_literal() {
        let open_bracket_tokens = [
            Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 1 }
        ];
        let mut open_bracket = CaptionParser::new(&open_bracket_tokens);

        let close_bracket_tokens = [
            Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 1 }
        ];
        let mut close_bracket = CaptionParser::new(&close_bracket_tokens);

        let string_tokens = [
            Token { kind: TokenKind::String, lexeme: "Test".into(), line: 1 }
        ];
        let mut string = CaptionParser::new(&string_tokens);

        let mut empty = CaptionParser::new(&[]);

        assert_eq!(
            string.consume_literal(),
            Ok("Test".into())
        );

        assert_eq!(
            open_bracket.consume_literal(),
            Err(CaptionParseError::UnexpectedToken { line: 1, expected: TokenKind::String, found: TokenKind::OpenBracket })
        );
        assert_eq!(
            close_bracket.consume_literal(),
            Err(CaptionParseError::UnexpectedToken { line: 1, expected: TokenKind::String, found: TokenKind::CloseBracket })
        );
        assert_eq!(
            empty.consume_literal(),
            Err(CaptionParseError::UnexpectedEof { expected: TokenKind::String })
        );
    }

    #[test]
    fn peek() {
        let open_bracket_tokens = [
            Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 1 }
        ];
        let mut open_bracket = CaptionParser::new(&open_bracket_tokens);

        let close_bracket_tokens = [
            Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 1 }
        ];
        let mut close_bracket = CaptionParser::new(&close_bracket_tokens);

        let string_tokens = [
            Token { kind: TokenKind::String, lexeme: "Test".into(), line: 1 }
        ];
        let mut string = CaptionParser::new(&string_tokens);

        let mut empty = CaptionParser::new(&[]);

        assert_eq!(open_bracket.peek(), Some(TokenKind::OpenBracket));
        assert_eq!(close_bracket.peek(), Some(TokenKind::CloseBracket));
        assert_eq!(string.peek(), Some(TokenKind::String));
        assert_eq!(empty.peek(), None);
    }
}