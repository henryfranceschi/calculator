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
            TokenKind::Eof => "<eof>",
        };

        f.write_str(s)
    }
}
