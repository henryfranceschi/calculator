mod cursor;
pub mod span;
pub mod token;

use self::{
    cursor::Cursor,
    span::Span,
    token::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Lexer<'a> {
    cursor: Cursor<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self { cursor: Cursor::new(source) }
    }

    pub fn next_token(&mut self) -> Result<Token<'a>, LexError<'a>> {
        self.cursor.advance_while(|c| c.is_ascii_whitespace());
        self.cursor.reset_start_index();

        let kind = match self.cursor.advance() {
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '%' => TokenKind::Percent,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '0'..='9' => {
                // Consume integral part.
                self.cursor.advance_while(|c| c.is_ascii_digit());

                // Consume fractional part.
                if self.cursor.peek_first() == '.' && self.cursor.peek_second().is_ascii_digit() {
                    self.cursor.advance();
                    self.cursor.advance_while(|c| c.is_ascii_digit());
                }

                TokenKind::Number
            }
            Cursor::EOF_CHAR => TokenKind::Eof,
            _ => return Err(self.error(LexErrorKind::Unexpected)),
        };

        Ok(self.token(kind))
    }

    fn token(&mut self, kind: TokenKind) -> Token<'a> {
        Token {
            slice: self.cursor.slice(),
            span: self.cursor.reset_span(),
            kind,
        }
    }

    fn error(&mut self, kind: LexErrorKind) -> LexError<'a> {
        LexError {
            slice: self.cursor.slice(),
            span: self.cursor.reset_span(),
            kind,
        }
    }
}

#[derive(Debug)]
pub struct LexError<'a> {
    slice: &'a str,
    span: Span,
    kind: LexErrorKind,
}

#[derive(Debug)]
enum LexErrorKind {
    UnterminatedString,
    Unexpected,
}

#[cfg(test)]
mod test {
    use crate::lexer::token::TokenKind;

    use super::{Lexer, LexError};

    #[test]
    fn number() -> Result<(), LexError<'static>> {
        let mut lexer = Lexer::new("3");
        assert_eq!(lexer.next_token()?.kind, TokenKind::Number);
        assert_eq!(lexer.next_token()?.kind, TokenKind::Eof);

        let mut lexer = Lexer::new("3.14");
        assert_eq!(lexer.next_token()?.kind, TokenKind::Number);
        assert_eq!(lexer.next_token()?.kind, TokenKind::Eof);

        Ok(())
    }

    #[test]
    fn unexpected() -> Result<(), LexError<'static>> {
        let mut lexer = Lexer::new("10 + #");
        assert_eq!(lexer.next_token()?.kind, TokenKind::Number);
        assert_eq!(lexer.next_token()?.kind, TokenKind::Plus);
        assert!(lexer.next_token().is_err());
        assert_eq!(lexer.next_token()?.kind, TokenKind::Eof);

        Ok(())
    }        
}
