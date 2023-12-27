use std::fmt::Debug;
use std::string::ToString;

use anyhow::{bail, Error, Ok, Result};
use scanner_rust::generic_array::typenum::Minimum;
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
        let res = match c {
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
        };
        println!("Converted: {:?}, to: {:?}", c, res);
        return res;
    }
}

impl std::str::FromStr for TokenType {
    type Err = Error;
    fn from_str(string: &str) -> Result<Self> {
        match string {
            "and" => Ok(TokenType::AND),
            "class" => Ok(TokenType::CLASS),
            "else" => Ok(TokenType::ELSE),
            "false" => Ok(TokenType::FALSE),
            "for" => Ok(TokenType::FOR),
            "fun" => Ok(TokenType::FUN),
            "if" => Ok(TokenType::IF),
            "nil" => Ok(TokenType::NIL),
            "or" => Ok(TokenType::OR),
            "print" => Ok(TokenType::PRINT),
            "return" => Ok(TokenType::RETURN),
            "super" => Ok(TokenType::SUPER),
            "this" => Ok(TokenType::THIS),
            "true" => Ok(TokenType::TRUE),
            "var" => Ok(TokenType::VAR),
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
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
}
impl Literal {
    pub fn inner(&self) -> String {
        match self {
            Self::String(string) => format!("{}", string),
            Self::Number(num) => format!("{}", num),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Token {
    pub _type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize
}

impl Token {
    pub fn new(_type: TokenType, lexeme: &str, literal: Option<Literal>, line: usize) -> Self {
        let lexeme = lexeme.to_string();
        Self {
            _type,
            lexeme,
            literal,
            line
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
