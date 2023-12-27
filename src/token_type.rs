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
            '\n' => TokenType::WHITESPACE,
            '"' => TokenType::STRING,
            c if c.is_ascii_digit() => TokenType::NUMBER,
            c if c.is_alphabetic() => TokenType::IDENTIFIER,
            _ => unreachable!("Unknown token!"),
        }
    }
}

impl std::str::FromStr for TokenType {
    type Err = Error;
    fn from_str(string: &str) -> Result<Self> {
        match string {
            "and" => Ok(TokenType::WHILE),
            "class" => Ok(TokenType::WHILE),
            "else" => Ok(TokenType::WHILE),
            "false" => Ok(TokenType::WHILE),
            "for" => Ok(TokenType::WHILE),
            "fun" => Ok(TokenType::WHILE),
            "if" => Ok(TokenType::WHILE),
            "nil" => Ok(TokenType::WHILE),
            "or" => Ok(TokenType::WHILE),
            "print" => Ok(TokenType::WHILE),
            "return" => Ok(TokenType::WHILE),
            "super" => Ok(TokenType::WHILE),
            "this" => Ok(TokenType::WHILE),
            "true" => Ok(TokenType::WHILE),
            "var" => Ok(TokenType::WHILE),
            "while" => Ok(TokenType::WHILE),
            _ => bail!("Can only be used ofr identifiers"),
        }
    }
}

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
        matches!(
            self,
            TokenType::BANG | TokenType::EQUAL | TokenType::LESS | TokenType::GREATER
        )
    }
}
#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(f64),
}
#[derive(Debug)]
pub struct Token {
    _type: TokenType,
    lexeme: String,
    literal: Option<Literal>,
}

impl Token {
    pub fn new(_type: TokenType, lexeme: &str, literal: Option<Literal>) -> Self {
        let lexeme = lexeme.to_string();
        Self {
            _type,
            lexeme,
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
