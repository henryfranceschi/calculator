use crate::{
    ast::{Ast, BinOpKind, Decl, DeclKind, Expr, ExprKind, Stmt, StmtKind, UnOpKind},
    bytecode::{Bytecode, Opcode},
};

#[derive(Debug, Default)]
pub struct CodeGenerator {
    bytecode: Bytecode,
    had_error: bool,
}

impl CodeGenerator {
    pub fn generate(&mut self, ast: &Ast) -> Bytecode {
        for decl in ast.decls() {
            self.decl(decl);
        }

        std::mem::take(&mut self.bytecode)
    }

    fn decl(&mut self, decl: &Decl) {
        match decl.kind() {
            DeclKind::Stmt(stmt) => self.stmt(stmt),
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt.kind() {
            StmtKind::Expr(expr) => {
                self.expr(expr);
                self.bytecode.write_byte(Opcode::Pop as u8);
            }
        }
    }

    fn expr(&mut self, expr: &Expr) {
        match expr.kind() {
            ExprKind::Number(value) => {
                let idx = self.bytecode.add_constant(*value);
                if let Ok(idx) = idx.try_into() {
                    self.bytecode.write_opcode(Opcode::Constant);
                    self.bytecode.write_byte(idx);
                } else {
                    self.had_error = true;
                }
            }
            ExprKind::Unary(op, expr) => {
                self.expr(expr);
                self.bytecode.write_opcode(match op.kind() {
                    UnOpKind::Neg => Opcode::Negate,
                });
            }
            ExprKind::Binary(op, expr_l, expr_r) => {
                self.expr(expr_l);
                self.expr(expr_r);
                self.bytecode.write_opcode(match op.kind() {
                    BinOpKind::Add => Opcode::Add,
                    BinOpKind::Sub => Opcode::Subtract,
                    BinOpKind::Mul => Opcode::Multiply,
                    BinOpKind::Div => Opcode::Divide,
                    BinOpKind::Rem => Opcode::Remainder,
                });
            }
        }
    }
}
