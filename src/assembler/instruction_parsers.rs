use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use crate::assembler::Token;
use nom::multispace;
use nom::types::CompleteStr;

#[derive(Debug, PartialEq)]
pub struct AssemblerInstruction {
    opcode: Token,
    operand1: Option<Token>,
    operand2: Option<Token>,
    operand3: Option<Token>,
}

impl AssemblerInstruction {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut res: Vec<u8> = vec![];
        match self.opcode {
            Token::Op { code } => match code {
                _ => {
                    res.push(code as u8);
                }
            },
            _ => {
                print!("Non-opcode found in opcode field");
                std::process::exit(1);
            }
        };

        for operand in vec![&self.operand1, &self.operand2, &self.operand3] {
            if let Some(token) = operand {
                AssemblerInstruction::extract_operand(token, &mut res);
            }
        }
        return res;
    }

    fn extract_operand(t: &Token, res: &mut Vec<u8>) {
        match t {
            Token::Register { reg_num } => {
                res.push(*reg_num);
            }
            Token::IntegerOperand { value } => {
                let byte1 = *value as u16;
                let byte2 = byte1 >> 8;
                res.push(byte2 as u8);
                res.push(byte1 as u8);
            }
            _ => {
                print!("Invalid operand found");
                std::process::exit(1);
            }
        }
    }
}

// for instructions that take two operands, e.g. LOAD $0 #100
named!(pub instruction_two_int<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode_load >>
        r: register >>
        i: integer_operand >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1: Some(r),
                operand2: Some(i),
                operand3: None,
            }
        )
    )
);

// for instructions that take two register operands, e.g. EQ $0 $1
named!(pub instruction_two_reg<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r1: register >>
        r2: register >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: None,
            }
        )
    )
);

// for instructions that take three register operands, e.g. ADD $0 $1 $2
named!(pub instruction_three<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r1: register >>
        r2: register >>
        r3: register >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1: Some(r1),
                operand2: Some(r2),
                operand3: Some(r3),
            }
        )
    )
);

// for instructions that take one register operand, e.g. JMP $0
named!(pub instruction_one<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        r: register >>
        (
            AssemblerInstruction {
                opcode: o,
                operand1: Some(r),
                operand2: None,
                operand3: None,
            }
        )
    )
);

// for instructions that take no operands, e.g. HLTs
named!(instruction_zero<CompleteStr, AssemblerInstruction>,
    do_parse!(
        o: opcode >>
        opt!(multispace) >>
        (
            AssemblerInstruction{
                opcode: o,
                operand1: None,
                operand2: None,
                operand3: None,
            }
        )
    )
);

named!(pub instruction<CompleteStr, AssemblerInstruction>,
    do_parse!(
        ins: alt!(
            instruction_two_int |
            instruction_two_reg |
            instruction_three |
            instruction_one |
            instruction_zero
        ) >> (ins)
    )
);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Opcode;

    #[test]
    fn test_parse_instruction_form_two() {
        let result = instruction_two_int(CompleteStr("load $0 #100\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::LOAD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::IntegerOperand { value: 100 }),
                    operand3: None
                }
            ))
        );

        let result = instruction_two_reg(CompleteStr("eq $0 $1\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::EQ },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_zero() {
        let result = instruction_zero(CompleteStr("hlt\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::HLT },
                    operand1: None,
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_one() {
        let result = instruction_one(CompleteStr("jmp $0\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::JMP },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: None,
                    operand3: None
                }
            ))
        );
    }

    #[test]
    fn test_parse_instruction_form_three() {
        let result = instruction_three(CompleteStr("add $0 $1 $2\n"));
        assert_eq!(
            result,
            Ok((
                CompleteStr(""),
                AssemblerInstruction {
                    opcode: Token::Op { code: Opcode::ADD },
                    operand1: Some(Token::Register { reg_num: 0 }),
                    operand2: Some(Token::Register { reg_num: 1 }),
                    operand3: Some(Token::Register { reg_num: 2 })
                }
            ))
        );
    }
}
