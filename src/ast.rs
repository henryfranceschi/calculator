use crate::lexer::span::Span;

pub struct Spanned<T> {
    pub node: T,
    pub span: Span,
}

impl<T> Spanned<T> {
    pub fn new(node: T, span: Span) -> Self {
        Self { node, span }
    }
}

pub struct UnOp(Spanned<UnOpKind>);

impl UnOp {
    pub fn new(span: Span, kind: UnOpKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }
}
pub enum UnOpKind {
    Neg,
}

pub struct BinOp(Spanned<BinOpKind>);

impl BinOp {
    pub fn new(span: Span, kind: BinOpKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }
}

pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

pub struct Expr(Spanned<ExprKind>);

impl Expr {
    pub fn new(span: Span, kind: ExprKind) -> Self {
        Self(Spanned::new(kind, span))
    }

    pub fn span(&self) -> Span {
        self.0.span
    }
}

pub enum ExprKind {
    Number(f64),
    Unary(UnOp, Box<Expr>),
    Binary(BinOp, Box<Expr>, Box<Expr>),
}
