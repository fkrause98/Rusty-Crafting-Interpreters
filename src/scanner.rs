use crate::token_type::{
    Literal, Token,
    TokenType::{self, BANG, EOF, EQUAL, GREATER, LESS},
};
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
        while (!self.is_at_end()) {
            self.start = self.current;
            self.scan_token();
        }
        self.tokens.push(Token::new(EOF, "", None, self.line));
        return &self.tokens;
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len() as u64;
    }

    fn scan_token(&mut self) {
        let c = self.advance().expect("No characters left!");
        match TokenType::from(c) {
            _type if _type.is_combinable_with_eq() && self.can_match_with(&_type, '=') => {
                _type.match_with_eq();
                self.add_token(_type, None)
            }
            _type if _type.is_combinable_with_div() && self.can_match_with(&_type, '/') => {
                while (self.peek() != '\n' && !self.is_at_end()) {
                    self.advance();
                }
            }
            TokenType::IGNORE => {}
            TokenType::WHITESPACE => {
                self.line+= 1;
            }
            _type => self.add_token(_type, None),
        };
    }

    fn peek(&mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn can_match_with(&mut self, token: &TokenType, expected: char) -> bool {
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
        self.current += 1;
        self.source.chars().nth(self.current as usize)
    }
    fn add_token(&mut self, _type: TokenType, literal: Option<Literal>) {
        let source = &self.source;
        let start = self.start as usize;
        let end = self.current as usize;
        let text = source.get(start..end).expect("Empty token!");
        self.tokens
            .push(Token::new(_type, text, literal, self.line))
    }
}
