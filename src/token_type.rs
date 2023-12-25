use std::fmt::Debug;
use std::string::ToString;

use anyhow::{bail, Error, Ok, Result};
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens.
    LEFT_PAREN,
    RIGHT_PAREN,
    LEFT_BRACE,
    RIGHT_BRACE,
    COMMA,
    DOT,
    MINUS,
    PLUS,
    SEMICOLON,
    SLASH,
    STAR,

    // One or two character tokens.
    BANG,
    BANG_EQUAL,
    EQUAL,
    EQUAL_EQUAL,
    GREATER,
    GREATER_EQUAL,
    LESS,
    LESS_EQUAL,

    // Literals.
    IDENTIFIER,
    STRING,
    NUMBER,

    // Keywords.
    AND,
    CLASS,
    ELSE,
    FALSE,
    FUN,
    FOR,
    IF,
    NIL,
    OR,
    PRINT,
    RETURN,
    SUPER,
    THIS,
    TRUE,
    VAR,
    WHILE,
    IGNORE,
    WHITESPACE,
    EOF,
}
impl From<char> for TokenType {
    fn from(c: char) -> Self {
        match c {
            '(' => TokenType::LEFT_PAREN,
            ')' => TokenType::RIGHT_PAREN,
            '{' => TokenType::LEFT_BRACE,
            '}' => TokenType::RIGHT_BRACE,
            ',' => TokenType::COMMA,
            '.' => TokenType::DOT,
            '-' => TokenType::MINUS,
            '+' => TokenType::PLUS,
            ';' => TokenType::SEMICOLON,
            '*' => TokenType::STAR,
            '!' => TokenType::BANG_EQUAL,
            '=' => TokenType::EQUAL,
            '<' => TokenType::LESS,
            '>' => TokenType::GREATER,
            ' ' | '\r' | '\t' => TokenType::IGNORE,
            '\t' => TokenType::WHITESPACE,
            _ => unreachable!("Unknown token!"),
        }
    }
}
// case '!':
//   addToken(match('=') ? BANG_EQUAL : BANG);
//   break;
// case '=':
//   addToken(match('=') ? EQUAL_EQUAL : EQUAL);
//   break;
// case '<':
//   addToken(match('=') ? LESS_EQUAL : LESS);
//   break;
// case '>':
//   addToken(match('=') ? GREATER_EQUAL : GREATER);
//   break;

impl TokenType {
    pub fn match_with_eq(&self) -> TokenType {
        if !self.is_combinable_with_eq() {
            panic!("Not expected {:?} before =", self)
        }
        match self {
            Self::BANG => Self::BANG_EQUAL,
            Self::EQUAL => Self::EQUAL_EQUAL,
            Self::LESS => Self::LESS_EQUAL,
            Self::GREATER => Self::GREATER_EQUAL,
            _ => unreachable!(),
        }
    }
    pub fn is_combinable_with_div(&self) -> bool {
        matches!(&self, TokenType::SLASH)
    }
    pub fn is_combinable_with_eq(&self) -> bool {
        match self {
            TokenType::BANG => true,
            TokenType::EQUAL => true,
            TokenType::LESS => true,
            TokenType::GREATER => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
}
pub struct Token {
    _type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
    line: u64,
}

impl Token {
    pub fn new(_type: TokenType, lexeme: &str, literal: Option<Literal>, line: u64) -> Self {
        let lexeme = lexeme.to_string();
        Self {
            _type,
            lexeme,
            line,
            literal,
        }
    }
}
impl ToString for Token {
    fn to_string(&self) -> String {
        format!(
            "{:?}, {:?}, {:?}, {:?}, {:?}",
            self._type, " ", self.lexeme, " ", self.literal
        )
    }
}
