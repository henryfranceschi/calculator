use super::span::Span;

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    /// The slice of the source where the token is located.
    pub slice: &'a str,
    pub span: Span,
    pub kind: TokenKind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenKind {
    Number,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Eof,
}
