#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Constant,
    Return,
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
            1 => Self::Return,
            2 => Self::Add,
            3 => Self::Subtract,
            4 => Self::Multiply,
            5 => Self::Divide,
            6 => Self::Negate,
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

    pub fn constant(&mut self, idx: u8) -> f64 {
        self.constants[idx as usize]
    }
}

impl AsRef<[u8]> for Bytecode {
    fn as_ref(&self) -> &[u8] {
        self.code.as_ref()
    }
}
