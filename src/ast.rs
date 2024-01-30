use crate::lexer::span::Span;

#[derive(Debug, Clone, Copy)]
pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnOp(Spanned<UnOpKind>);

impl UnOp {
    pub fn new(span: Span, kind: UnOpKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }

    pub fn kind(&self) -> &UnOpKind {
        &self.0.node
    }
}

#[derive(Debug, Clone, Copy)]
pub enum UnOpKind {
    Neg,
}

#[derive(Debug, Clone, Copy)]
pub struct BinOp(Spanned<BinOpKind>);

impl BinOp {
    pub fn new(span: Span, kind: BinOpKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }

    pub fn kind(&self) -> &BinOpKind {
        &self.0.node
    }
}

#[derive(Debug, Clone, Copy)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Debug, Clone)]
pub struct Expr(Spanned<ExprKind>);

impl Expr {
    pub fn new(span: Span, kind: ExprKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }

    pub fn kind(&self) -> &ExprKind {
        &self.0.node
    }

    pub fn unary(operator: UnOp, operand: Expr) -> Self {
        Self::new(
            Span::between(operator.span(), operand.span()),
            ExprKind::Unary(operator, Box::new(operand)),
        )
    }

    pub fn binary(operator: BinOp, operand_1: Expr, operand_2: Expr) -> Self {
        Self::new(
            Span::between(operand_1.span(), operand_2.span()),
            ExprKind::Binary(operator, Box::new(operand_1), Box::new(operand_2)),
        )
    }
}

#[derive(Debug, Clone)]
pub enum ExprKind {
    Number(f64),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
}
