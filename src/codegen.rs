use crate::{
    ast::{BinOpKind, Expr, ExprKind, UnOpKind},
    bytecode::{Bytecode, Opcode},
};

pub fn generate(expr: &Expr) -> Bytecode {
    let mut bytecode = Bytecode::default();
    expr_generate(&mut bytecode, expr);
    bytecode.write_opcode(Opcode::Return);
    bytecode
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
