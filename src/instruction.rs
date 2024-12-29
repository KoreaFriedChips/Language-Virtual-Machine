use nom::types::CompleteStr;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    LOAD, // load $0 #10
    ADD,  // add $0 $1 $2
    SUB,  // sub $0 $1 $2
    MUL,  // mul $0 $1 $2
    DIV,  // div $0 $1 $2
    HLT,  // hlt
    JMP,  // jmp $0
    JMPF, // jmpf $0
    JMPB, // jmpb $0
    EQ,   // eq $0 $1
    NEQ,  // neq $0 $1
    GT,   // gt $0 $1
    LT,   // lt $0 $1
    GTQ,  // gtq $0 $1
    LTQ,  // ltq $0 $1
    JEQ,  // jeq $0
    JNEQ, // jneq $0
    ALOC, // aloc $0
    INC,  // inc $0
    DEC,  // dec $0
    IGL,  // illegal
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction { opcode: opcode }
    }
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            0 => Opcode::LOAD,
            1 => Opcode::ADD,
            2 => Opcode::SUB,
            3 => Opcode::MUL,
            4 => Opcode::DIV,
            5 => Opcode::HLT,
            6 => Opcode::JMP,
            7 => Opcode::JMPF,
            8 => Opcode::JMPB,
            9 => Opcode::EQ,
            10 => Opcode::NEQ,
            11 => Opcode::GT,
            12 => Opcode::LT,
            13 => Opcode::GTQ,
            14 => Opcode::LTQ,
            15 => Opcode::JEQ,
            16 => Opcode::JNEQ,
            17 => Opcode::ALOC,
            18 => Opcode::INC,
            19 => Opcode::DEC,
            _ => Opcode::IGL,
        }
    }
}

impl<'a> From<CompleteStr<'a>> for Opcode {
    fn from(v: CompleteStr<'a>) -> Self {
        let s = v.to_uppercase();

        match s.as_str() {
            "LOAD" => Opcode::LOAD,
            "ADD" => Opcode::ADD,
            "SUB" => Opcode::SUB,
            "MUL" => Opcode::MUL,
            "DIV" => Opcode::DIV,
            "HLT" => Opcode::HLT,
            "JMP" => Opcode::JMP,
            "JMPF" => Opcode::JMPF,
            "JMPB" => Opcode::JMPB,
            "EQ" => Opcode::EQ,
            "NEQ" => Opcode::NEQ,
            "GT" => Opcode::GT,
            "LT" => Opcode::LT,
            "GTQ" => Opcode::GTQ,
            "LTQ" => Opcode::LTQ,
            "JEQ" => Opcode::JEQ,
            "JNEQ" => Opcode::JNEQ,
            "ALOC" => Opcode::ALOC,
            "INC" => Opcode::INC,
            "DEC" => Opcode::DEC,
            _ => Opcode::IGL,
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

    #[test]
    fn test_str_to_opcode() {
        let opcode = Opcode::from(CompleteStr("load"));
        assert_eq!(opcode, Opcode::LOAD);
        let opcode = Opcode::from(CompleteStr("illegal"));
        assert_eq!(opcode, Opcode::IGL);
    }
}
