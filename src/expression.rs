use crate::token_type::{Literal, Token, TokenType};

#[derive(Debug, Clone)]
pub enum Expression<'a> {
    Literal(Literal),
    Unary {
        modifier: Token,
        expression: &'a Expression<'a>,
    },
    Binary {
        operator: Token,
        l_expression: &'a Expression<'a>,
        r_expression: &'a Expression<'a>,
    },
    Grouping {
        expression: &'a Expression<'a>,
    },
}
