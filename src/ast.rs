use crate::lexer::span::Span;

#[derive(Clone, Copy)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

#[derive(Clone, Copy)]
pub struct UnOp(Spanned<UnOpKind>);

impl UnOp {
    pub fn new(span: Span, kind: UnOpKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }
}

#[derive(Clone, Copy)]
pub enum UnOpKind {
    Neg,
}

#[derive(Clone, Copy)]
pub struct BinOp(Spanned<BinOpKind>);

impl BinOp {
    pub fn new(span: Span, kind: BinOpKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }
}

#[derive(Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[derive(Clone)]
pub struct Expr(Spanned<ExprKind>);

impl Expr {
    pub fn new(span: Span, kind: ExprKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }

    pub fn unary(operator: UnOp, operand: Expr) -> Self {
        Self::new(
            Span::new(operator.span().start(), operand.span().end()),
            ExprKind::Unary(operator, Box::new(operand)),
        )
    }

    pub fn binary(operator: BinOp, operand_1: Expr, operand_2: Expr) -> Self {
        Self::new(
            Span::new(operand_1.span().start(), operand_2.span().end()),
            ExprKind::Binary(operator, Box::new(operand_1), Box::new(operand_2)),
        )
    }
}

#[derive(Clone)]
pub enum ExprKind {
    Number(f64),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
}
