use crate::lexer::*;
pub mod lexer;
use crate::parser::*;
pub mod parser;

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
#[derive(Debug, Clone)]
struct Expr {
    left: Element,
    operator: String,
    right: Element,
}
#[derive(Debug, Clone)]
enum Element {
    Expr(Box<Expr>),
    Number(i32),
}
fn main() {
    let tokens = create_tokens("12 * 2 + 3 / (4 - 1)");
    println!("{:?}", tokens);

    parse_tokens(tokens);

    // let mut expr = Expr {
    //     left: Element::Number(12),
    //     operator: "+".to_string(),
    //     right: Element::Number(1),
    // };
    // let mut expr2 = Expr {
    //     left: Element::Number(12),
    //     operator: "+".to_string(),
    //     right: Element::Expr(Box::new(expr)),
    // };
    // println!("{:#?}", expr2.clone());
}
