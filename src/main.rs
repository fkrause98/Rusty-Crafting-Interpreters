use anyhow::Result;
use rustyline;
use std::{env::args, process::exit};
pub mod lox;
pub mod scanner;
pub mod token_type;
use crate::lox::Lox;
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
        lox.run_prompt()?;
    }
    Ok(())
}
