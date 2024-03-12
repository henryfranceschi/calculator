use std::fmt::Display;

use super::span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<'a> {
    pub lexeme: &'a str,
    pub span: Span,
    pub kind: TokenKind,
}

impl Token<'_> {
    pub fn dummy() -> Token<'static> {
        Token {
            lexeme: "",
            span: Span::new(0, 0),
            kind: TokenKind::Dummy,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    LParen,
    RParen,
    Semicolon,
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,

    /// Never produced by the lexer, used only in places where we need a placeholder token. Should
    /// never be consumed by the parser.
    Dummy,
    // Always the last token produced by the lexer.
    Eof,
}

impl TokenKind {
    /// If the `lexeme`s of all `Token`s with this kind are identical.
    pub fn is_uniform(self) -> bool {
        !matches!(self, Self::Number)
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            TokenKind::LParen => "(",
            TokenKind::RParen => ")",
            TokenKind::Semicolon => ";",
            TokenKind::Number => "<number>",
            TokenKind::Plus => "+",
            TokenKind::Minus => "-",
            TokenKind::Star => "*",
            TokenKind::Slash => "/",
            TokenKind::Percent => "%",
            TokenKind::Dummy => "<dummy>",
            TokenKind::Eof => "<eof>",
        };

        if !self.is_uniform() || *self == Self::Eof {
            write!(f, "{}", s)
        } else {
            write!(f, "'{}'", s)
        }
    }
}
