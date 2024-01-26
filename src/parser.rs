use crate::{
    ast::{BinOp, BinOpKind, Expr, ExprKind, UnOp, UnOpKind},
    lexer::{
        span::Span,
        token::{Token, TokenKind},
        Lexer,
    },
};

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
                message: format!("expected {:?}", kind),
            })
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParseError> {
        let expr = self.expr(0)?;
        self.expect(TokenKind::Eof)?;

        Ok(expr)
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        self.advance();
        let mut expr = match self.previous.kind {
            TokenKind::Number => {
                let number = self.previous.slice.parse().unwrap();
                Expr::new(self.previous.span, ExprKind::Number(number))
            }
            TokenKind::LParen => {
                let expr = self.expr(0)?;
                self.expect(TokenKind::RParen)?;
                expr
            }
            _ => {
                if let Some(op) = prefix_op(&self.previous) {
                    let (_, r_bp) = prefix_binding_power(&op);
                    Expr::unary(op, self.expr(r_bp)?)
                } else {
                    return Err(ParseError {
                        span: self.current.span,
                        message: "expected expression".to_owned(),
                    });
                }
            }
        };

        loop {
            if let Some(op) = infix_op(&self.current) {
                let (l_bp, r_bp) = infix_binding_power(&op);
                if l_bp < min_bp {
                    break;
                }

                self.advance();
                expr = Expr::binary(op, expr, self.expr(r_bp)?);
                continue;
            }

            break;
        }

        Ok(expr)
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

fn prefix_op(token: &Token) -> Option<UnOp> {
    let unop = match token.kind {
        TokenKind::Minus => UnOp::new(token.span, UnOpKind::Neg),
        _ => return None,
    };

    Some(unop)
}

fn infix_op(token: &Token) -> Option<BinOp> {
    let kind = match token.kind {
        TokenKind::Plus => BinOpKind::Add,
        TokenKind::Minus => BinOpKind::Sub,
        TokenKind::Star => BinOpKind::Mul,
        TokenKind::Slash => BinOpKind::Div,
        TokenKind::Percent => BinOpKind::Rem,
        _ => return None,
    };

    Some(BinOp::new(token.span, kind))
}

fn prefix_binding_power(unop: &UnOp) -> ((), u8) {
    match unop.kind() {
        UnOpKind::Neg => ((), 5),
    }
}

fn infix_binding_power(binop: &BinOp) -> (u8, u8) {
    match binop.kind() {
        BinOpKind::Add | BinOpKind::Sub => (1, 2),
        BinOpKind::Mul | BinOpKind::Div | BinOpKind::Rem => (3, 4),
    }
}

#[derive(Debug, Clone)]
pub struct ParseError {
    span: Span,
    message: String,
}

impl ParseError {
    pub fn span(&self) -> Span {
        self.span
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
