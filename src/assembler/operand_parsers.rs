use nom::digit;
use nom::types::CompleteStr;

use crate::assembler::register_parsers::register;
use crate::assembler::Token;

named!(pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            value: digit >>
            (
                Token::IntegerOperand {
                    value: value.parse::<i32>().unwrap()
                }
            )
        )
    )
);

named!(pub operand<CompleteStr, Token>,
    alt!(
        integer_operand |
        register
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#123"));
        assert_eq!(result.is_ok(), true);
        let (_, token) = result.unwrap();
        assert_eq!(token, Token::IntegerOperand { value: 123 });

        let result = integer_operand(CompleteStr("#a"));
        assert_eq!(result.is_ok(), false);

        let result = integer_operand(CompleteStr("123"));
        assert_eq!(result.is_ok(), false);
    }
}
