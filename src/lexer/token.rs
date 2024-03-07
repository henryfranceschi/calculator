use super::span::Span;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Token<'a> {
    pub lexeme: &'a str,
    pub span: Span,
    pub kind: TokenKind,
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
    Eof,
}
