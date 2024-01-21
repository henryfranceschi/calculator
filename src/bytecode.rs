#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
}

#[derive(Debug, Clone)]
pub struct Bytecode {
    code: Vec<u8>,
    constants: Vec<f64>,
}

impl Bytecode {
    pub fn write_byte(&mut self, byte: u8) {
        self.code.push(byte);
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
        let next_idx = self.constants.len();
        assert!(next_idx <= u8::MAX.into());

        self.constants.push(value);

        next_idx
    }
}
