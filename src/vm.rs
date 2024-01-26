use crate::bytecode::{Bytecode, Opcode};
use std::convert::TryInto;

pub struct Vm {
    bytecode: Bytecode,
    ip: usize,
    stack: Vec<f64>,
}

impl Vm {
    pub fn new(bytecode: Bytecode) -> Self {
        Self {
            bytecode,
            ip: 0,
            stack: vec![],
        }
    }

    pub fn run(&mut self) -> Result<f64, VmError> {
        let value = loop {
            match self
                .read_byte()
                .try_into()
                .map_err(|_| VmError::InvalidOpcode)?
            {
                Opcode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a + b)?;
                }
                Opcode::Subtract => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a - b)?;
                }
                Opcode::Multiply => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a * b)?;
                }
                Opcode::Divide => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a / b)?;
                }
                Opcode::Remainder => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.push(a % b)?;
                }
                Opcode::Negate => {
                    let a = self.pop()?;
                    self.push(-a)?;
                }
                Opcode::Constant => {
                    let offset = self.read_byte();
                    let value = self.bytecode.constant(offset);
                    self.push(value)?;
                }
                Opcode::Return => {
                    break self.pop()?;
                }
            }
        };

        Ok(value)
    }

    fn read_byte(&mut self) -> u8 {
        let slice = self.bytecode.as_ref();
        let byte = slice[self.ip];
        self.ip += 1;
        byte
    }

    fn push(&mut self, value: f64) -> Result<(), VmError> {
        self.stack.push(value);

        Ok(())
    }

    fn pop(&mut self) -> Result<f64, VmError> {
        self.stack.pop().ok_or(VmError::MissingOperand)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VmError {
    MissingOperand,
    InvalidOpcode,
}
