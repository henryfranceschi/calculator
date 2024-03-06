use crate::{
    ast::{Ast, BinOpKind, Decl, DeclKind, Expr, ExprKind, Stmt, StmtKind, UnOpKind},
    bytecode::{Bytecode, Opcode},
};

pub fn generate(ast: &Ast) -> Bytecode {
    let mut bytecode = Bytecode::default();
    for decl in ast.decls() {
        decl_generate(&mut bytecode, decl);
    }
    bytecode
}

fn decl_generate(bytecode: &mut Bytecode, decl: &Decl) {
    match decl.kind() {
        DeclKind::Stmt(stmt) => stmt_generate(bytecode, stmt),
    }
}

fn stmt_generate(bytecode: &mut Bytecode, stmt: &Stmt) {
    match stmt.kind() {
        StmtKind::Expr(expr) => {
            expr_generate(bytecode, expr);
            bytecode.write_opcode(Opcode::Pop);
        }
    }
}

fn expr_generate(bytecode: &mut Bytecode, expr: &Expr) {
    match expr.kind() {
        ExprKind::Number(value) => {
            let idx = bytecode.add_constant(*value);
            bytecode.write_opcode(Opcode::Constant);
            bytecode.write_byte(idx.try_into().unwrap());
        }
        ExprKind::Unary(op, expr) => {
            expr_generate(bytecode, expr);
            bytecode.write_opcode(match op.kind() {
                UnOpKind::Neg => Opcode::Negate,
            });
        }
        ExprKind::Binary(op, expr_l, expr_r) => {
            expr_generate(bytecode, expr_l);
            expr_generate(bytecode, expr_r);
            bytecode.write_opcode(match op.kind() {
                BinOpKind::Add => Opcode::Add,
                BinOpKind::Sub => Opcode::Subtract,
                BinOpKind::Mul => Opcode::Multiply,
                BinOpKind::Div => Opcode::Divide,
                BinOpKind::Rem => Opcode::Remainder,
            });
        }
    }
}
