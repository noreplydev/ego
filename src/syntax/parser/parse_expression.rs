use crate::{
    core::{
        error::{self, ErrorType},
        types::RuntimeType,
    },
    syntax::{AstNode, AstNodeType, BinaryOperator, Expression, LexerToken, LexerTokenType},
};

// returns: offset of tokens index, expression node, error?
pub fn lookahead_expression(
    tokens: &Vec<LexerToken>,
    mut current: usize,
) -> (usize, AstNode, bool) {
    let expression_types = vec![
        LexerTokenType::StringLiteral,
        LexerTokenType::Number,
        LexerTokenType::AddOperator,
        LexerTokenType::TrueKeyword,
        LexerTokenType::FalseKeyword,
        LexerTokenType::FunctionCall,
        LexerTokenType::Identifier,
    ];

    let mut expressions_counter = 0;
    let mut expression_stack: Vec<AstNode> = vec![];

    while current < tokens.len() {
        let token = &tokens[current];

        if expression_types.contains(&token.token_type) {
            match token.token_type {
                LexerTokenType::Number => {
                    if expression_stack.len() < 1 {
                        if let Ok(number) = token.value.parse() {
                            expression_stack.push(AstNode::new(
                                AstNodeType::Expression(Expression::NumberLiteral),
                                RuntimeType::number(number),
                                vec![],
                            ));
                        } else {
                            error::throw(
                                ErrorType::ExpressionError,
                                format!(
                                    "Error: Expected number expression found: '{}'",
                                    token.value.to_string()
                                )
                                .as_str(),
                                -1,
                            );
                        }
                    } else {
                        error::throw(
                            ErrorType::ExpressionError,
                            format!(
                                "Expected binary operator after '{}' to use as against '{}'",
                                expression_stack[0].value.to_string(),
                                token.value.to_string()
                            )
                            .as_str(),
                            -1,
                        );
                        std::process::exit(1);
                    }
                }
                LexerTokenType::AddOperator => {
                    if let Some(previous_expression) = expression_stack.get(0) {
                        let binary_op_node = match token.value.as_str() {
                            "+" => AstNode::new(
                                AstNodeType::Expression(Expression::Binary(
                                    BinaryOperator::AddOperator,
                                )),
                                RuntimeType::identifier(token.value.clone()),
                                vec![],
                            ),
                            _ => {
                                error::throw(
                                    ErrorType::ExpressionError,
                                    format!(
                                        "Expected binary operator and found '{}'",
                                        token.value.to_string()
                                    )
                                    .as_str(),
                                    -1,
                                );

                                std::process::exit(1);
                            }
                        };
                        expression_stack.pop(); // clear stack
                    } else {
                        error::throw(
                            ErrorType::ExpressionError,
                            format!(
                                "Expected expression before '{}' binary operator",
                                token.value.to_string()
                            )
                            .as_str(),
                            -1,
                        );
                        std::process::exit(1);
                    }
                }
                _ => {}
            }
        } else if expressions_counter == 0 {
            // means no expression at all
            return (
                expressions_counter,
                AstNode::new(AstNodeType::Empty, RuntimeType::nothing(), vec![]),
                true,
            );
        }

        current += 1;
        expressions_counter += 1;
    }

    (
        expressions_counter,
        AstNode::new(
            AstNodeType::Expression(Expression::StringLiteral),
            RuntimeType::string("test".to_string()),
            vec![],
        ),
        false,
    )
}
