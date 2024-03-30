use crate::{
    core::error::{self, ErrorType},
    syntax::{
        binary_expression::BinaryExpression, number::Number, AstNodeType, Expression, LexerToken,
        LexerTokenType,
    },
};

// 2 + 3 * 23
pub fn parse_expression(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    // will autoincrement current
    // and it will be the root or the left node
    // depending on the expression
    let (offset, mut node) = parse_term(tokens, current);
    let mut current = current + offset;

    while current < tokens.len() {
        match tokens[current].token_type {
            LexerTokenType::AddOperator | LexerTokenType::SubtractOperator => {
                let curr_token = &tokens[current];

                // consume the operator
                let operator = if let Some(op) = curr_token.value.chars().next() {
                    op
                } else {
                    error::throw(
                        ErrorType::ParsingError,
                        format!("Operator '{}' cannot be parsed as char", curr_token.value)
                            .as_str(),
                        Some(curr_token.line),
                    );
                    std::process::exit(1);
                };
                current += 1;

                // get right node
                let (offset, right) = parse_term(tokens, current);
                current += offset;

                node = Expression::BinaryExpression(BinaryExpression::new(
                    operator,
                    Box::new(node),
                    Box::new(right),
                    curr_token.at,
                    curr_token.line,
                ));
            }
            _ => {
                error::throw(
                    error::ErrorType::SyntaxError,
                    format!(
                        "Invalid token '{}' inside of a expression",
                        tokens[current].value
                    )
                    .as_str(),
                    Some(tokens[current].line),
                );
                std::process::exit(1);
            }
        }
    }

    (current, AstNodeType::Expression(node))
}

fn parse_term(tokens: &Vec<LexerToken>, current: usize) -> (usize, Expression) {
    // will autoincrement current
    // and it will be the root or the left node
    // depending on the expression
    let (offset, mut node) = parse_factor(tokens, current);
    let mut current = current + offset;

    while current < tokens.len() {
        match tokens[current].token_type {
            LexerTokenType::AddOperator | LexerTokenType::SubtractOperator => {
                let curr_token = &tokens[current];

                // consume the operator
                let operator = if let Some(op) = curr_token.value.chars().next() {
                    op
                } else {
                    error::throw(
                        ErrorType::ParsingError,
                        format!("Operator '{}' cannot be parsed as char", curr_token.value)
                            .as_str(),
                        Some(curr_token.line),
                    );
                    std::process::exit(1);
                };
                current += 1;

                // get right node
                let (offset, right) = parse_factor(tokens, current);
                current += offset;

                node = Expression::BinaryExpression(BinaryExpression::new(
                    operator,
                    Box::new(node),
                    Box::new(right),
                    curr_token.at,
                    curr_token.line,
                ));
            }
            _ => {
                error::throw(
                    error::ErrorType::SyntaxError,
                    format!(
                        "Invalid token '{}' inside of a expression",
                        tokens[current].value
                    )
                    .as_str(),
                    Some(tokens[current].line),
                );
                std::process::exit(1);
            }
        }
    }

    (current, node)
}

fn parse_factor(tokens: &Vec<LexerToken>, current: usize) -> (usize, Expression) {
    match tokens[current].token_type {
        LexerTokenType::Number => {
            let number_node = Number::from_string(
                tokens[current].value.clone(),
                tokens[current].at,
                tokens[current].line,
            );

            if let Some(node) = number_node {
                // 1 since it only consumes the number token.
                // in the future this will be inside a struct and
                // will mutate internal state
                (1, Expression::Number(node))
            } else {
                error::throw(
                    ErrorType::ParsingError,
                    format!(
                        "Invalid token '{}' inside of a expression",
                        tokens[current].value
                    )
                    .as_str(),
                    Some(tokens[current].line),
                );
                std::process::exit(1);
            }
        }
        _ => {
            error::throw(
                error::ErrorType::SyntaxError,
                format!(
                    "Invalid token '{}' inside of a expression",
                    tokens[current].value
                )
                .as_str(),
                Some(tokens[current].line),
            );
            std::process::exit(1);
        }
    }
}

/* (
    3,
    AstNodeType::Expression(Expression::BinaryExpression(BinaryExpression::new(
        '+',
        Box::new(Expression::Bool(Bool::new(true, 0, 0))),
        Box::new(Expression::Bool(Bool::new(true, 0, 0))),
        0,
        0,
    ))),
) */
