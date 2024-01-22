#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Constant,
    Add,
    Subtract,
    Multiply,
    Divide,
    Negate,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, ()> {
        let opcode = match value {
            0 => Self::Constant,
            1 => Self::Add,
            2 => Self::Subtract,
            3 => Self::Multiply,
            4 => Self::Divide,
            5 => Self::Negate,
            _ => return Err(()),
        };

        Ok(opcode)
    }
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

    pub fn write_opcode(&mut self, opcode: Opcode) {
        self.code.push(opcode as u8);
    }

    pub fn add_constant(&mut self, value: f64) -> usize {
        let next_idx = self.constants.len();
        assert!(next_idx <= u8::MAX.into());

        self.constants.push(value);

        next_idx
    }
}
