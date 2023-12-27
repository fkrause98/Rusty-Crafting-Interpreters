use std::process::exit;

use crate::scanner::Scanner;
use anyhow::Result;

pub struct Lox {
    had_error: bool,
}
impl Lox {
    pub fn new() -> Self {
        return Self {
            had_error: false,
        };
    }
    pub fn run_prompt(&mut self) -> Result<()> {
        let mut rl = rustyline::DefaultEditor::new()?;
        loop {
            let readline = rl.readline("> ");
            match readline {
                Ok(line) => {
                    self.run(&line);
                    self.had_error = false
                }
                Err(_) => break,
            }
        }
        Ok(())
    }

    pub fn run_file(&mut self, path: &str) {
        let bytes = std::fs::read(path).expect("Aborting, file not found");
        self.run(std::str::from_utf8(&bytes).unwrap())
    }

    pub fn run(&mut self, source: &str) {
        let mut scanner = Scanner::new(source);
        let tokens = scanner.scan_tokens();
        for token in tokens {
            println!("{:?}", token);
        }
        if self.had_error {
            exit(65);
        }
    }

    pub fn error(&mut self, line: usize, message: &str) {
        self.report(line, "", message)
    }
    pub fn report(&mut self, line: usize, place: &str, message: &str) {
        eprintln!(
            "{}, {}, {}, {}, {}, {}",
            "[line ", line, "] Error", place, ": ", message
        );
    }
}
