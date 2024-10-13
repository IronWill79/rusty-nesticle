#[derive(Debug)]
pub enum Instruction {
    ADD(ArithmeticTarget),
    INC(IncDecTarget),
}

impl Instruction {
    pub fn from_byte(byte: u8) -> Option<Instruction> {
        match byte {
            0x02 => Some(Instruction::INC(IncDecTarget::BC)),
            0x13 => Some(Instruction::INC(IncDecTarget::BC)),
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum ArithmeticTarget {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
}

#[derive(Debug)]
pub enum IncDecTarget {
    BC,
    DE,
}
