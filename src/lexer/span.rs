use std::ops::Add;

/// Represents a region of the source code.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Span {
    /// Start index of the token.
    start: usize,
    /// End index of the token (exclusive).
    end: usize,
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

pub fn slice<'a>(source: &'a str, span: &Span) -> &'a str {
    &source[span.start..span.end]
}

/// Returns the number of the line on which the span begins.
pub fn starting_line_number(source: &str, span: &Span) -> usize {
    source[..span.start]
        .chars()
        .filter(|c| *c == '\n')
        .count()
        .add(1)
}

/// Returns the number of the column on which the span begins.
pub fn starting_column_number(source: &str, span: &Span) -> usize {
    source[..span.start]
        .chars()
        .rev()
        .take_while(|c| *c != '\n')
        .count()
        .add(1)
}
