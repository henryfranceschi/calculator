use crate::{
    ast::{Ast, BinOp, BinOpKind, Decl, Expr, ExprKind, Stmt, UnOp, UnOpKind},
    diagnostics::report_error,
    lexer::{
        span::Span,
        token::{Token, TokenKind},
        Lexer, LexicalError,
    },
};

#[derive(Debug)]
pub struct Parser<'a> {
    source: &'a str,
    lexer: Lexer<'a>,
    current: Token<'a>,
    previous: Token<'a>,
    had_error: bool,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut parser = Self {
            source,
            lexer: Lexer::new(source),
            current: Token::dummy(),
            previous: Token::dummy(),
            had_error: false,
        };

        // For the parser to be in a valid state we need to advance here.
        if parser.advance().is_err() {
            parser.had_error = true;
            parser.synchronize();
        }

        parser
    }

    fn advance(&mut self) -> Result<(), LexicalError> {
        let token = self.lexer.next_token()?;
        self.previous = std::mem::replace(&mut self.current, token);
        Ok(())
    }

    fn expect(&mut self, kind: TokenKind) -> Result<(), ParseError> {
        if self.current.kind == kind {
            self.advance()?;
            Ok(())
        } else {
            Err(ParseError::SyntacticError(SyntacticError {
                span: self.current.span,
                message: format!("expected {}, got {}", kind, self.current.kind),
            }))
        }
    }

    fn synchronize(&mut self) {
        while self.current.kind != TokenKind::Eof {
            match self.current.kind {
                TokenKind::Semicolon => {
                    // If advance fails here need continue discarding until we reach the next
                    // declaration.
                    match self.advance() {
                        Ok(()) => break,
                        Err(err) => {
                            // Report error.
                            report_error("lexical error", err.span, self.source);
                            continue;
                        }
                    }
                }
                // Tokens marking the begining of a declaration.
                // TokenKind::Let | TokenKind::Func => break,
                _ => {
                    if let Err(err) = self.advance() {
                        report_error("lexical error", err.span, self.source);
                    }
                    continue;
                }
            }
        }
    }

    pub fn parse(&mut self) -> Ast {
        // Attempt to parse declarations, synchronize on failure.
        let mut decls = vec![];
        while self.current.kind != TokenKind::Eof {
            match self.decl() {
                Ok(decl) => {
                    decls.push(decl);
                }
                Err(err) => {
                    self.had_error = true;
                    match err {
                        ParseError::LexicalError(err) => {
                            report_error("lexical error", err.span, self.source);
                        }
                        ParseError::SyntacticError(err) => {
                            report_error(&err.message, err.span, self.source);
                        }
                    }
                    self.synchronize();
                }
            }
        }

        Ast::new(decls, !self.had_error)
    }

    fn decl(&mut self) -> Result<Decl, ParseError> {
        Ok(Decl::stmt(self.stmt()?))
    }

    fn stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expr(0)?;
        self.expect(TokenKind::Semicolon)?;
        Ok(Stmt::expr(expr))
    }

    fn expr(&mut self, min_bp: u8) -> Result<Expr, ParseError> {
        self.advance()?;
        let mut expr = match self.previous.kind {
            TokenKind::Number => {
                let number = self.previous.lexeme.parse().unwrap();
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
                    return Err(SyntacticError {
                        span: self.current.span,
                        message: "expected expression".to_owned(),
                    }
                    .into());
                }
            }
        };

        loop {
            if let Some(op) = infix_op(&self.current) {
                let (l_bp, r_bp) = infix_binding_power(&op);
                if l_bp < min_bp {
                    break;
                }

                self.advance()?;
                expr = Expr::binary(op, expr, self.expr(r_bp)?);
                continue;
            }

            break;
        }

        Ok(expr)
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

pub enum ParseError {
    LexicalError(LexicalError),
    SyntacticError(SyntacticError),
}

impl From<LexicalError> for ParseError {
    fn from(value: LexicalError) -> Self {
        ParseError::LexicalError(value)
    }
}

impl From<SyntacticError> for ParseError {
    fn from(value: SyntacticError) -> Self {
        ParseError::SyntacticError(value)
    }
}

#[derive(Debug, Clone)]
pub struct SyntacticError {
    pub span: Span,
    pub message: String,
}
