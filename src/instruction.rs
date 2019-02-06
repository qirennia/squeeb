#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LOAD,
    MOV,
    STOR,
    LDIM,
    IGL
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0x00 => Opcode::HLT,
            0x01 => Opcode::LOAD,
            0x02 => Opcode::MOV,
            0x03 => Opcode::STOR,
            0x04 => Opcode::LDIM,
            _ => Opcode::IGL
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}