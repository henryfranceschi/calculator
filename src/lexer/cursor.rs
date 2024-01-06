use std::str::Chars;

use super::span::Span;

#[derive(Debug)]
pub struct Cursor<'a> {
    source: &'a str,
    it: Chars<'a>,
    start: usize,
}

impl<'a> Cursor<'a> {
    pub const EOF_CHAR: char = '\0';

    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            it: source.chars(),
            start: 0,
        }
    }

    pub fn advance(&mut self) -> char {
        self.it.next().unwrap_or(Self::EOF_CHAR)
    }

    pub fn peek_first(&self) -> char {
        let mut it = self.it.clone();
        it.next().unwrap_or(Self::EOF_CHAR)
    }

    pub fn peek_second(&self) -> char {
        let mut it = self.it.clone();
        it.next();
        it.next().unwrap_or(Self::EOF_CHAR)
    }

    /// Advances if the predicate returns `true` and returns whether the cursor advanced.
    pub fn advance_if<F>(&mut self, predicate: F) -> bool
    where
        F: FnOnce(char) -> bool + Copy,
    {
        let mut advanced = false;
        if predicate(self.peek_first()) {
            self.advance();
            advanced = true;
        }

        advanced
    }

    /// Advances if the next character equals `c` and returns whether the cursor advanced.
    pub fn advance_if_eq(&mut self, c: char) -> bool {
        self.advance_if(|p| p == c)
    }

    /// Advances while the next token matches the predicate and returns whether the cursor
    /// advanced.
    pub fn advance_while<F>(&mut self, predicate: F) -> bool
    where
        F: FnOnce(char) -> bool + Copy,
    {
        let mut advanced = false;
        while !self.is_at_end() && predicate(self.peek_first()) {
            self.advance();
            advanced = true;
        }

        advanced
    }

    pub fn is_at_end(&self) -> bool {
        self.peek_first() == Self::EOF_CHAR
    }

    pub fn start_index(&self) -> usize {
        self.start
    }

    pub fn reset_start_index(&mut self) -> usize {
        let start = self.start;
        self.start = self.current_index();
        start
    }

    pub fn current_index(&self) -> usize {
        self.source.len() - self.it.as_str().len()
    }

    pub fn slice(&self) -> &'a str {
        &self.source[self.start_index()..self.current_index()]
    }

    pub fn span(&mut self) -> Span {
        Span::new(self.start, self.current_index())
    }

    pub fn reset_span(&mut self) -> Span {
        Span::new(self.reset_start_index(), self.current_index())
    }
}
