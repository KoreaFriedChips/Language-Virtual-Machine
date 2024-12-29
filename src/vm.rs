use crate::instruction::Opcode;

pub struct VM {
    pub registers: [i32; 32],
    // array of registers so we can have the location of each register at compile time
    pc: usize,            // program counter
    pub program: Vec<u8>, // program stored as byte code in a vector
    remainder: u32,       // remainder register for division instruction
    equal_flag: bool, // contains the result of the last comparison operation, usually mips uses another register
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            pc: 0,
            program: vec![],
            remainder: 0,
            equal_flag: false,
        }
    }

    pub fn run(&mut self) {
        let mut flag = false;
        while !flag {
            flag = self.execute_instruction();
        }
    }

    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        if self.pc >= self.program.len() {
            return true;
        }

        match self.decode_opcode() {
            Opcode::HLT => {
                println!("HLT encountered");
                return true;
            }
            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;
                self.registers[register] = number as i32;
            }
            Opcode::ADD => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = r1 + r2;
            }
            Opcode::SUB => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = r1 - r2;
            }
            Opcode::MUL => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = r1 * r2;
            }
            Opcode::DIV => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                self.registers[self.next_8_bits() as usize] = r1 / r2;
                self.remainder = (r1 % r2) as u32;
            }
            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            }
            Opcode::JMPF => {
                let val = self.registers[self.next_8_bits() as usize];
                self.pc += val as usize;
            }
            Opcode::JMPB => {
                let val = self.registers[self.next_8_bits() as usize];
                self.pc -= val as usize;
            }
            Opcode::EQ => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                if r1 == r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::NEQ => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                if r1 != r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::GT => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                if r1 > r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::LT => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                if r1 < r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::GTQ => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                if r1 >= r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::LTQ => {
                let r1 = self.registers[self.next_8_bits() as usize];
                let r2 = self.registers[self.next_8_bits() as usize];
                if r1 <= r2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }
                self.next_8_bits();
            }
            Opcode::JEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if self.equal_flag {
                    self.pc = target as usize;
                }
            }
            Opcode::JNEQ => {
                let target = self.registers[self.next_8_bits() as usize];
                if !self.equal_flag {
                    self.pc = target as usize;
                }
            }
            _ => {
                println!("Unrecognized opcode found! Terminating!");
                return true;
            }
        }
        return false;
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    fn next_8_bits(&mut self) -> u8 {
        let res = self.program[self.pc];
        self.pc += 1;
        return res;
    }

    fn next_16_bits(&mut self) -> u16 {
        let res = ((self.program[self.pc] as u16) << 8) | self.program[self.pc + 1] as u16;
        self.pc += 2;
        return res;
    }

    pub fn add_byte(&mut self, byte: u8) {
        self.program.push(byte);
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = VM::new();
        let test_bytes = vec![5, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = VM::new();
        let test_bytes = vec![200, 0, 0, 0];
        test_vm.program = test_bytes;
        test_vm.run();
        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_load_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_add_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244, 0, 1, 0, 244, 1, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
        assert_eq!(test_vm.registers[1], 244);
        assert_eq!(test_vm.registers[2], 744);
    }

    #[test]
    fn test_sub_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 1, 244, 0, 1, 0, 244, 2, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 500);
        assert_eq!(test_vm.registers[1], 244);
        assert_eq!(test_vm.registers[2], 256);
    }

    #[test]
    fn test_mul_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 0, 7, 0, 1, 0, 8, 3, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 7);
        assert_eq!(test_vm.registers[1], 8);
        assert_eq!(test_vm.registers[2], 56);
    }

    #[test]
    fn test_div_opcode() {
        let mut test_vm = VM::new();
        test_vm.program = vec![0, 0, 0, 8, 0, 1, 0, 5, 4, 0, 1, 2];
        test_vm.run();
        assert_eq!(test_vm.registers[0], 8);
        assert_eq!(test_vm.registers[1], 5);
        assert_eq!(test_vm.registers[2], 1);
        assert_eq!(test_vm.remainder, 3);
    }

    #[test]
    fn test_jmp_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 5;
        test_vm.program = vec![6, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 5);
    }

    #[test]
    fn test_jmpf_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_eq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.registers[1] = 20;
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_jeq_opcode() {
        let mut test_vm = VM::new();
        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;
        test_vm.registers[2] = 20;
        test_vm.program = vec![9, 0, 1, 0, 15, 2, 0, 0];
        test_vm.run_once();
        assert_eq!(test_vm.equal_flag, true);
        test_vm.run_once();
        assert_eq!(test_vm.pc, 20);
    }
}
