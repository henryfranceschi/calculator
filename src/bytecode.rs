#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Opcode {
    Constant,
    Pop,
    Return,
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Negate,
}

impl TryFrom<u8> for Opcode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        let opcode = match value {
            0 => Self::Constant,
            1 => Self::Pop,
            2 => Self::Return,
            3 => Self::Add,
            4 => Self::Subtract,
            5 => Self::Multiply,
            6 => Self::Divide,
            7 => Self::Remainder,
            8 => Self::Negate,
            _ => return Err(()),
        };

        Ok(opcode)
    }
}

#[derive(Debug, Default, Clone)]
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
        self.constants.push(value);
        next_idx
    }

    pub fn constant<T: Into<usize>>(&self, idx: T) -> f64 {
        self.constants[idx.into()]
    }
}

impl Display for Bytecode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut it = self.code.iter();
        while let Some(&byte) = it.next() {
            let op: Opcode = byte.try_into().expect("invalid opcode");
            write!(f, "{:?}", op)?;

            // Print operands
            match op {
                Opcode::Constant => {
                    writeln!(f, " {:02X}", it.next().expect("expected operand"))?;
                }
                _ => {
                    writeln!(f)?;
                }
            }
        }
        writeln!(f, "{:?}", self.constants)?;

        Ok(())
    }
}

impl AsRef<[u8]> for Bytecode {
    fn as_ref(&self) -> &[u8] {
        self.code.as_ref()
    }
}
