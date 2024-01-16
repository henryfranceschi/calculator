use crate::{lexer::{
    span::Span,
    token::{Token, TokenKind},
    Lexer,
}, ast::Expr};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current: Token<'a>,
    previous: Token<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let current = report_errs_until_ok(&mut lexer);
        let previous = current.clone();

        Self {
            lexer,
            current,
            previous,
        }
    }

    fn advance(&mut self) {
        let token = report_errs_until_ok(&mut self.lexer);
        self.previous = std::mem::replace(&mut self.current, token);
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.current.kind == kind {
            self.advance();
            Ok(())
        } else {
            Err(ParseError {
                span: self.current.span,
                kind: ParseErrorKind::Expected(kind),
            })
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        self.expr(0)
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        todo!()
    }
}

fn report_errs_until_ok<'a>(lexer: &mut Lexer<'a>) -> Token<'a> {
    loop {
        let result = lexer.next_token();
        match result {
            Ok(token) => break token,
            Err(err) => eprintln!("lexical error: {err:?}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ParseError {
    span: Span,
    kind: ParseErrorKind,
}

#[derive(Debug, Clone, Copy)]
pub enum ParseErrorKind {
    Unexpected,
    Expected(TokenKind),
}
