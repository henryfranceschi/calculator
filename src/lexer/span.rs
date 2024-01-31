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

    pub fn between(start: Span, end: Span) -> Span {
        Self {
            start: start.start,
            end: end.end,
        }
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn slice(self, source: &str) -> &str {
        &source[self.start..self.end]
    }

    /// Returns the number of the line on which the span begins.
    pub fn starting_line_number(self, source: &str) -> usize {
        source[..self.start]
            .chars()
            .filter(|c| *c == '\n')
            .count()
            .add(1)
    }

    /// Returns the number of the column on which the span begins.
    pub fn starting_column_number(self, source: &str) -> usize {
        source[..self.start]
            .chars()
            .rev()
            .take_while(|c| *c != '\n')
            .count()
            .add(1)
    }
}
