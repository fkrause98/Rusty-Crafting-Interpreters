use anyhow::Result;
use std::{env::args, process::exit};
use token_type::{Literal::Number, Token};
pub mod ast_printer;
pub mod expression;
pub mod lox;
pub mod scanner;
pub mod token_type;
use crate::lox::Lox;
#[allow(clippy::print_literal)]
#[allow(clippy::comparison_chain)]
fn main() -> Result<()> {
    let mut args = args();
    args.next();
    let argument_amount = args.len();
    let mut lox = Lox::new();
    if argument_amount > 2_usize {
        println!("Usage: jlox [script]");
        exit(64);
    } else if argument_amount == 2_usize {
        lox.run_file(args.next().unwrap().as_ref());
    } else {
        let unary = expression::Expression::Unary {
            modifier: Token {
                _type: token_type::TokenType::MINUS,
                lexeme: "-".to_string(),
                literal: None,
                line: 1usize,
            },
            expression: &expression::Expression::Literal(Number(123.0_f64)),
        };
        let mul = Token::new(token_type::TokenType::STAR, "*", None, 1usize);
        let num = expression::Expression::Literal(Number(45.67_f64));
        let grouping = expression::Expression::Grouping { expression: &num };
        let expression = expression::Expression::Binary {
            operator: mul,
            l_expression: &unary,
            r_expression: &grouping,
        };
        let ast = ast_printer::print(&expression);
        println!("{}", ast);
        // let expression::Expression = expression::Expression::Binary { operator: (), l_expression: (), r_expression: () };
        // lox.run_prompt()?;
    }
    Ok(())
}
