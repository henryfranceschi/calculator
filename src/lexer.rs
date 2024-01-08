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
    pub fn new(cursor: Cursor<'a>) -> Self {
        Self { cursor }
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
