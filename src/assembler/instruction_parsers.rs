use crate::assembler::opcode_parsers::*;
use crate::assembler::operand_parsers::integer_operand;
use crate::assembler::register_parsers::register;
use crate::assembler::Token;
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
            match operand {
                Some(t) => AssemblerInstruction::extract_operand(t, &mut res),
                None => {}
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

named!(pub instr_zero<CompleteStr, AssemblerInstruction>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assembler::Opcode;

    #[test]
    fn test_parse_instruction_form_zero() {
        let result = instr_zero(CompleteStr("load $0 #100\n"));
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
    }
}