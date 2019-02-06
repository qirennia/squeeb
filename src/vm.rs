extern crate byteorder;

use byteorder::{ByteOrder, LittleEndian};
use crate::instruction::Opcode;

pub struct VM {
    registers: [i32; 32],
    pc: usize,
    program: Vec<u8>,
    data: Vec<u8>
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            data: vec![],
            pc: 0,
        }
    }

    fn decode_opcode(&mut self) -> Opcode {
        return Opcode::from(self.next_u8());
    }

    fn next_u8(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;
        return result;
    }

    fn next_u16(&mut self) -> u16 {
        let result = LittleEndian::read_u16(&self.program[self.pc..=self.pc+1]);
        self.pc += 2;
        return result;
    }

    fn next_u32(&mut self) -> u32 {
        let result = LittleEndian::read_u32(&self.program[self.pc..=self.pc+3]);
        self.pc += 4;
        return result;
    }

    fn next_i32(&mut self) -> i32 {
        let result = LittleEndian::read_i32(&self.program[self.pc..=self.pc+3]);
        self.pc += 4;
        return result;
    }

    fn next_reg(&mut self) -> usize {
        return self.next_u8() as usize;
    }

    fn next_addr(&mut self) -> usize {
        return self.next_u32() as usize;
    }

    pub fn run(&mut self) {
        loop {
            // If our program counter has exceeded the length of the program itself, something has
            // gone awry
            if self.pc >= self.program.len() {
                break;
            }
            match self.decode_opcode() {
                Opcode::HLT => {
                    println!("HLT encountered");
                    return;
                },
                Opcode::IGL => {
                    println!("Unrecognized opcode found! Terminating!");
                    return;
                },
                Opcode::LOAD => {
                    let register = self.next_reg();
                    let addr = self.next_addr();
                    self.registers[register] = LittleEndian::read_i32(&self.data[addr..=addr+3]);
                    continue;
                },
                Opcode::LDIM => {
                    let reg = self.next_reg();
                    self.registers[reg] = self.next_i32();
                    continue;
                },
                Opcode::MOV => {
                    let source = self.next_reg();
                    let dest = self.next_reg();
                    self.registers[dest] = self.registers[source];
                    continue;
                },
                Opcode::STOR => {
                    let reg = self.next_reg();
                    let dest = self.next_addr();
                    LittleEndian::write_i32_into(&self.registers[reg..1], &mut self.data[dest..]);
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0)
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![Opcode::HLT as u8, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![Opcode::IGL as u8 ,0 ,0 ,0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = VM::new();
        test_vm.program = vec![Opcode::LOAD as u8, 0, 0, 0, 0, 0];
        test_vm.data = vec![244, 1, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_stor() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 1;
        test_vm.program = vec![Opcode::STOR as u8, 0, 0, 0, 0, 0];
        test_vm.data = vec![0, 0, 0, 0];
        test_vm.run();
        assert_eq!(test_vm.data[0], 1);
    }
}