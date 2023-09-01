use crate::{Element, Expr, Token};

pub fn parse_tokens(tokens: Vec<Token>) -> Vec<Token> {
    let mut cursor_pos = 0;

    let parse_number = |token: Token| -> Element {
        let num = match token {
            Token::Number(num) => Element::Number(num),
            _ => {
                panic!()
            }
        };
        num
    };
    let parse_mul_div = || {
        let mut expr = parse_number(tokens[cursor_pos]);
        cursor_pos += 1;
        while tokens[cursor_pos] == Token::Multiply || tokens[cursor_pos] == Token::Divide {
            cursor_pos += 1;
            let right = parse_number(tokens[cursor_pos]);
            cursor_pos += 1;
            expr = Element::Expr(Box::new(Expr {
                left: expr,
                operator: "+".to_string(),
                right: right,
            }))
        }
    };

    tokens
}
