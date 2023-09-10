pub mod lexer;
pub mod parser;

use crate::lexer::*;
use crate::parser::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Token {
    Add,
    Subtract,
    Multiply,
    Divide,
    OpenBracket,
    CloseBracket,
    Number(i32),
}

fn main() {
    let input_data = "12 * 2 + 9 / (4 - 1)";
    println!("{}", input_data);

    match create_tokens(input_data) {
        Ok(tokens) => {
            println!("Tokens: {:?}", tokens);

            let mut parser = Parser::new(tokens);
            match parser.parse() {
                Ok(result) => println!("Result: {}", result),
                Err(err) => println!("Parsing Error: {}", err),
            }
        }
        Err(err) => println!("Lexer Error: {}", err),
    }
}
