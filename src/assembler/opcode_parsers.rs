use crate::assembler::Token;
use crate::instruction::Opcode;
use nom::types::CompleteStr;

named!(pub opcode_load<CompleteStr, Token>,
  do_parse!(
      tag_no_case!("load") >> (Token::Op{code: Opcode::LOAD})
  )
);

mod tests {

    use super::*;

    #[test]
    fn test_opcode_load() {
        // First tests that the opcode is detected and parsed correctly
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op { code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        // Tests that an invalid opcode isn't recognized
        let result = opcode_load(CompleteStr("aold"));
        assert_eq!(result.is_ok(), false);
    }
}
