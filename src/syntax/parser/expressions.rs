use crate::{
    core::error::{self, ErrorType},
    syntax::{
        binary_expression::BinaryExpression, bool::Bool, identifier::Identifier, number::Number,
        string_literal::StringLiteral, AstNodeType, Expression, LexerToken, LexerTokenType,
    },
};

pub fn expression(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    let (offset, expr) = parse_expression(tokens, current);
    (offset, AstNodeType::Expression(expr))
}

// 2 + 3 * 23
pub fn parse_expression(tokens: &Vec<LexerToken>, current: usize) -> (usize, Expression) {
    // will autoincrement current
    // and it will be the root or the left node
    // depending on the expression
    let (node_offset, mut node) = parse_term(tokens, current);
    let mut offset = 0 + node_offset;
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
                offset += 1;

                // get right node
                let (right_offset, right) = parse_term(tokens, current);
                current += right_offset;
                offset += right_offset;

                node = Expression::BinaryExpression(BinaryExpression::new(
                    operator,
                    Box::new(node),
                    Box::new(right),
                    curr_token.at,
                    curr_token.line,
                ));
            }
            _ => break,
        }
    }

    (offset, node)
}

fn parse_term(tokens: &Vec<LexerToken>, current: usize) -> (usize, Expression) {
    // will autoincrement current
    // and it will be the root or the left node
    // depending on the expression
    let (node_offset, mut node) = parse_factor(tokens, current);
    let mut offset = 0 + node_offset;
    let mut current = current + offset;

    while current < tokens.len() {
        match tokens[current].token_type {
            LexerTokenType::MultiplyOperator | LexerTokenType::DivideOperator => {
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
                offset += 1;

                // get right node
                let (right_offset, right) = parse_factor(tokens, current);
                current += right_offset;
                offset += right_offset;

                node = Expression::BinaryExpression(BinaryExpression::new(
                    operator,
                    Box::new(node),
                    Box::new(right),
                    curr_token.at,
                    curr_token.line,
                ));
            }
            _ => break,
        }
    }

    (offset, node)
}

fn parse_factor(tokens: &Vec<LexerToken>, current: usize) -> (usize, Expression) {
    match tokens[current].token_type {
        LexerTokenType::OpenParenthesis => {
            let mut offset = 1; // to consume open parenthesis
            let mut current = current + offset;

            let (expr_offset, expr) = parse_expression(tokens, current);
            current += expr_offset;
            offset += expr_offset;

            if tokens[current].token_type == LexerTokenType::CloseParenthesis {
                offset += 1; // to consume close parenthesis
                (offset, expr)
            } else {
                error::throw(
                    ErrorType::ParsingError,
                    format!("Unexpected token '{}', expected ')'", tokens[current].value).as_str(),
                    Some(tokens[current].line),
                );
                std::process::exit(1);
            }
        }
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
        LexerTokenType::TrueKeyword | LexerTokenType::FalseKeyword => {
            let node = if let Ok(bool_value) = tokens[current].value.parse::<bool>() {
                Bool::new(bool_value, tokens[current].at, tokens[current].line)
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
            };

            (1, Expression::Bool(node))
        }
        LexerTokenType::StringLiteral => (
            1,
            Expression::StringLiteral(StringLiteral::new(
                tokens[current].value.clone(),
                tokens[current].at,
                tokens[current].line,
            )),
        ),
        LexerTokenType::Identifier => (
            1,
            Expression::Identifier(Identifier::new(
                tokens[current].value.clone(),
                tokens[current].at,
                tokens[current].line,
            )),
        ),
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
