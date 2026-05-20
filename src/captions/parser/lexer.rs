
use std::borrow::Cow;

use crate::captions::{Token, TokenKind};

#[derive(Debug, Clone)]
pub struct CaptionLexer<'src> {
    source: &'src [u8],
    pos: usize,
    line: usize
}

impl<'src> CaptionLexer<'src> {
    #[inline]
    pub fn new(source: &'src str) -> Self {
        Self { source: source.as_bytes(), pos: 0, line: 1 }
    }

    pub fn tokenise(&mut self) -> Box<[Token<'src>]> {
        let mut tokens = Vec::<Token>::with_capacity(self.source.len() >> 2);
        while let Some(x) = self.next() {
            tokens.push(x);
        }
        tokens.into_boxed_slice()
    }

    #[inline]
    const fn is_at_end(&self) -> bool {
        return self.pos >= self.source.len();
    }

    #[inline]
    const fn advance(&mut self) {
        if !self.is_at_end() {
            self.pos += 1;
        }
    }

    #[inline]
    fn peek(&self) -> Option<&u8> {
        self.source.get(self.pos)
    }

    #[inline]
    fn peek_next(&self) -> Option<&u8> {
        self.source.get(self.pos + 1)
    }

    fn take_while<P: Fn(&u8) -> bool>(&mut self, predicate: P) -> &'src [u8] {
        let start = self.pos;
        while let Some(ch) = self.peek() {
            if !predicate(ch) {
                break;
            }
            self.advance();
        }
        let end = self.pos;

        return &self.source[start .. end]
    }

    #[inline]
    fn take_unquoted(&mut self) -> &'src str {
        let bytes = self.take_while(|ch|
            !matches!(ch, b'}' | b'{' | b'"' | b'\t' | b'\x0C' | b'\r' | b'\n' | b' ')
        );

        unsafe { str::from_utf8_unchecked(bytes) }
    }

    #[inline]
    fn take_quoted(&mut self) -> Cow<'src, str> {
        self.advance();
        let (start, mut was_escaped) = (self.pos, false);

        while let Some(ch) = self.peek() {
            match ch {
                b'\\' => {
                    if matches!(self.peek_next(), Some(b'\\' | b't' | b'n' | b'"')) {
                        self.advance();
                        self.advance();
                        was_escaped = true;
                    }
                },
                b'"' => break,
                _ => {}
            }
            self.advance();
        }

        let end = self.pos;
        self.advance();

        let str = unsafe {
            str::from_utf8_unchecked(&self.source[start .. end])
        };

        if !was_escaped {
            return Cow::Borrowed(str)
        }

        let mut out = String::with_capacity(str.len());
        let mut iter = str.as_bytes().iter().peekable();

        while let Some(ch) = iter.next().copied() {
            let ch = if ch == b'\\' {
                match iter.peek() {
                    Some(b'\\') => { iter.next(); '\\' }
                    Some(b'n') => { iter.next(); '\n' }
                    Some(b't') => { iter.next(); '\t' }
                    Some(b'"') => { iter.next(); '"' }
                    _ => '\\'
                }
            }
            else {
                ch as char
            };
            
            out.push(ch);
        }

        Cow::Owned(out)
    }

    fn skip_spaces(&mut self) {
        while let Some(ch) = self.peek() {
            match ch {
                b'\n' => {
                    self.advance();
                    self.line += 1
                }

                b'\t' | b'\x0C' | b'\r' | b' ' => {
                    self.advance()
                }

                b'/' => {
                    if matches!(self.peek_next(), Some(b'/')) {
                        self.advance();
                        self.advance();
                        self.take_while(|ch| *ch != b'\n');
                    }
                    else {
                        return;
                    }
                }

                _ => return
            }
        }
    }

    fn next(&mut self) -> Option<Token<'src>> {
        self.skip_spaces();

        let ch = self.peek()?;
        let token = match ch {
            b'{' => {
                self.advance();
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: self.line }
            }

            b'}' => {
                self.advance();
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: self.line }
            }

            b'"' => {
                let value = self.take_quoted();
                Token { kind: TokenKind::String, lexeme: value.into(), line: self.line }
            }

            _ => {
                let value = self.take_unquoted();
                Token { kind: TokenKind::String, lexeme: value.into(), line: self.line }
            }
        };

        Some(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_captions() {
        let content = r#""lang"
            {
            "Language" "English"
            "Tokens"
            {
            "barn.anotherdropship"	"Another dropship! "
            }
            }
        "#;

        let mut lexer = CaptionLexer::new(content);
        assert_eq!(
            lexer.tokenise(),
            [
                Token { kind: TokenKind::String, lexeme: "lang".into(), line: 1 },
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 2 },
                Token { kind: TokenKind::String, lexeme: "Language".into(), line: 3 },
                Token { kind: TokenKind::String, lexeme: "English".into(), line: 3 },
                Token { kind: TokenKind::String, lexeme: "Tokens".into(), line: 4 },
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 5 },
                Token { kind: TokenKind::String, lexeme: "barn.anotherdropship".into(), line: 6 },
                Token { kind: TokenKind::String, lexeme: "Another dropship! ".into(), line: 6 },
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 7 },
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 8 }
            ].into()
        )
    }

    #[test]
    fn with_unquoted() {
        let content = r#"lang
            {
            "Language "English" 
            "Tokens
            {
            barn.anotherdropship	"Another dropship! "
            }
            }
        "#;

        let mut lexer = CaptionLexer::new(content);
        assert_eq!(
            lexer.tokenise(),
            [
                Token { kind: TokenKind::String, lexeme: "lang".into(), line: 1 },
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 2 },
                Token { kind: TokenKind::String, lexeme: "Language ".into(), line: 3 },
                Token { kind: TokenKind::String, lexeme: "English".into(), line: 3 },
                Token { kind: TokenKind::String, lexeme: " \n            ".into(), line: 3 },
                Token { kind: TokenKind::String, lexeme: "Tokens".into(), line: 3 },
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 4 },
                Token { kind: TokenKind::String, lexeme: "barn.anotherdropship".into(), line: 5 },
                Token { kind: TokenKind::String, lexeme: "Another dropship! ".into(), line: 5 },
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 6 },
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 7 }
            ].into()
        )
    }

    #[test]
    fn with_comments() {
        let content = r#"//
            lang{
            Language //English
            Tokens {
            // aaaaa
            //
            // 
            // bbbbbbbbbbbbbbbb b b b
            barn.anotherdropship	Another dropship!
            }
            }
        "#;

        let mut lexer = CaptionLexer::new(content);
        assert_eq!(
            lexer.tokenise(),
            [
                Token { kind: TokenKind::String, lexeme: "lang".into(), line: 2 },
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 2 },
                Token { kind: TokenKind::String, lexeme: "Language".into(), line: 3 },
                Token { kind: TokenKind::String, lexeme: "Tokens".into(), line: 4 },
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 4 },
                Token { kind: TokenKind::String, lexeme: "barn.anotherdropship".into(), line: 9 },
                Token { kind: TokenKind::String, lexeme: "Another".into(), line: 9 },
                Token { kind: TokenKind::String, lexeme: "dropship!".into(), line: 9 },
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 10 },
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 11 }
            ].into()
        )
    }

    #[test]
    fn with_escaped() {
        let content = r#""\tlang"
            {
            Language "Eng\nlish"
            Tokens
            {
            "barn\\.anotherdropship"	"\"Another\" dropship! "
            }
            }
        "#;

        let mut lexer = CaptionLexer::new(content);
        assert_eq!(
            lexer.tokenise(),
            [
                Token { kind: TokenKind::String, lexeme: "\tlang".into(), line: 1 },
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 2 },
                Token { kind: TokenKind::String, lexeme: "Language".into(), line: 3 },
                Token { kind: TokenKind::String, lexeme: "Eng\nlish".into(), line: 3 },
                Token { kind: TokenKind::String, lexeme: "Tokens".into(), line: 4 },
                Token { kind: TokenKind::OpenBracket, lexeme: "{".into(), line: 5 },
                Token { kind: TokenKind::String, lexeme: "barn\\.anotherdropship".into(), line: 6 },
                Token { kind: TokenKind::String, lexeme: "\"Another\" dropship! ".into(), line: 6 },
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 7 },
                Token { kind: TokenKind::CloseBracket, lexeme: "}".into(), line: 8 }
            ].into()
        )
    }

    #[test]
    fn empty_input() {
        let mut lexer = CaptionLexer::new("");
        assert_eq!(lexer.tokenise(), [].into())
    }
}