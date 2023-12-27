use crate::token_type::{
    Literal, Token,
    TokenType::{EOF, self},
};
use anyhow::Context;
use anyhow::Result;
#[derive(Debug)]
pub struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: u64,
    current: u64,
    line: u64,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        let tokens = vec![];
        let source = source.to_string();
        Self {
            source,
            tokens,
            start: 0,
            current: 0,
            line: 1,
        }
    }
    pub fn scan_tokens<'a>(&'a mut self) -> &'a Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token().unwrap();
        }
        self.tokens.push(Token::new(EOF, "", None));
        return &self.tokens;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as u64;
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self
            .advance()
            .context("Scanner Error: No characters left!")?;
        match TokenType::from(c) {
            _type if _type.is_combinable_with_eq() && self.can_match_with(&_type, '=') => {
                _type.match_with_eq();
                self.add_token(_type, None)
            }
            _type if _type.is_combinable_with_div() && self.can_match_with(&_type, '/') => {
                while self.peek()? != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }
            TokenType::IGNORE => {}
            TokenType::WHITESPACE => self.string()?,
            TokenType::NUMBER => self.number()?,
            TokenType::IDENTIFIER => self.identifier()?,
            _type => self.add_token(_type, None),
        };
        Ok(())
    }

    fn identifier(&mut self) -> Result<()>{
        while self.peek()?.is_alphanumeric() {
            self.advance();
        }
        let start = self.start as usize;
        let current = self.current as usize;
        let text = self.source.get(start..current).context("Expected identifier")?;
        let _type: Result<TokenType> = text.parse::<TokenType>();
        match _type {
            Ok(keyword) => self.add_token(keyword, None),
            Err(_) => self.add_token(TokenType::IDENTIFIER, None),
        }
        Ok(())
    }

    fn number(&mut self) -> Result<()> {
        while self.peek()?.is_digit(10) {
            self.advance();
        }
        if self.peek()? == '.' && self.peek_next().is_digit(10) {
            self.advance();

            while self.peek_next().is_digit(10) {
                self.advance();
            }
        }
        let start = self.start as usize;
        let current = self.current as usize;
        let source = &self.source;
        let literal_number = source.get(start..current).context("Expected a number")?;
        let number = Literal::Number(literal_number.parse::<f64>()?);
        self.add_token(TokenType::NUMBER, Some(number));
        Ok(())
    }

    fn peek_next(&self) -> char {
        let next_char_index = (self.current + 1) as usize;
        // TODO: Return Option<Char> and turn this into a map.
        if let Some(next_char) = self.source.chars().nth(next_char_index) {
            return next_char;
        } else {
            return '\0';
        }
    }

    fn string(&mut self) -> Result<()> {
        while self.peek()? != '\n' && self.is_at_end() {
            if self.peek()? == '\n' {
                self.line += 1;
            }
            self.advance();
        }
        if self.is_at_end() {
            panic!("Unterminated string at: {}", self.line)
        }
        self.advance();
        let start = (self.start + 1) as usize;
        let end = (self.current - 1) as usize;
        let value = self.source.get(start..end).context("Expected end of string")?;
        self.add_token(TokenType::STRING, Some(Literal::String(value.to_string())));
        Ok(())
    }
    fn peek(&mut self) -> Result<char> {
        if self.is_at_end() {
            return Ok('\0');
        }
        return self.source.chars().nth(self.current as usize).context("Reached EOF");
    }

    fn can_match_with(&mut self, token: &TokenType, expected: char) -> bool {
        let _ = token;
        let at_eof = self.is_at_end();
        let next_is_not_expected =
            expected != self.source.chars().nth(self.current as usize).unwrap();
        if at_eof || next_is_not_expected {
            return false;
        }
        self.current += 1;
        return true;
    }
    fn advance(&mut self) -> Option<char> {
        let next = self.source.chars().nth(self.current as usize);
        self.current += 1;
        return next;
    }
    fn add_token(&mut self, _type: TokenType, literal: Option<Literal>) {
        let source = &self.source;
        let start = self.start as usize;
        let end = self.current as usize;
        let text = source.get(start..end).expect("Empty token!");
        self.tokens
            .push(Token::new(_type, text, literal))
    }
}
