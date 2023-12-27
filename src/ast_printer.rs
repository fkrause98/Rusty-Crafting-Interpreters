use crate::expression::Expression::{self, Binary, Grouping, Literal, Unary};
fn parenthesize(name: &str, exprs: &[&Expression]) -> String {
    let mut strings: Vec<String> = vec![];
    strings.push("(".to_string());
    strings.push(name.to_string());
    for expression in exprs {
        strings.push(" ".to_string());
        strings.push(print(expression));
    }
    strings.push(")".to_string());
    strings.into_iter().collect()
}

pub fn print(expr: &Expression) -> String {
    match expr {
        Literal(literal) => literal.inner(),
        Unary {
            modifier,
            expression,
        } => parenthesize(&modifier.lexeme, &[expression]),
        Binary {
            operator,
            l_expression,
            r_expression,
        } => parenthesize(&operator.lexeme, &[l_expression, r_expression]),
        Grouping { expression } => parenthesize("group", &[expression]),
    }
}
