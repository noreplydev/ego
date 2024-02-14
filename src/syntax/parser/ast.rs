use crate::error;
use std::vec;

use super::expressions::lookahead_expression;

use crate::{
    core::{error::ErrorType, types::RuntimeType},
    syntax::{
        tree::{BinaryOperator, Bool},
        AstNode, AstNodeType, AstTree,
        Expression::{self, Binary, Identifier, NumberLiteral, StringLiteral},
        LexerToken, LexerTokenType,
    },
};

pub fn parse(tokens: Vec<LexerToken>) -> AstTree {
    let _tree = tree(tokens);
    let ast = AstTree::new(_tree);
    return ast;
}

fn tree(tokens: Vec<LexerToken>) -> AstNode {
    let mut current = 0;
    let mut root = AstNode::root();

    while current < tokens.len() {
        let token = &tokens[current];

        match token.token_type {
            LexerTokenType::OpenCurlyBrace => {
                let (index_offset, block_node) = block(&tokens, current);
                root.add_child(block_node);
                current += index_offset;
            }
            LexerTokenType::OpenParenthesis => {
                let (index_offset, group_node) = group(&tokens, current);
                root.add_child(group_node);
                current += index_offset;
            }
            LexerTokenType::FunctionCall => {
                let (index_offset, print_node) = function_call(&tokens, current);
                root.add_child(print_node);
                current += index_offset;
            }
            LexerTokenType::IfKeyword => {
                let (index_offset, if_node) = if_statement(&tokens, current);
                root.add_child(if_node);
                current += index_offset;
            }
            LexerTokenType::LetKeyword => {
                let (index_offset, assignment_node) = assignment_statement(&tokens, current);
                root.add_child(assignment_node);
                current += index_offset;
            }
            LexerTokenType::TrueKeyword => {
                root.add_child(AstNode::new(
                    AstNodeType::Expression(Expression::Boolean(Bool::True)),
                    RuntimeType::string(token.value.clone()),
                    Vec::new(),
                ));
                current += 1;
            }
            LexerTokenType::FalseKeyword => {
                root.add_child(AstNode::new(
                    AstNodeType::Expression(Expression::Boolean(Bool::False)),
                    RuntimeType::string(token.value.clone()),
                    Vec::new(),
                ));
                current += 1;
            }
            LexerTokenType::StringLiteral => {
                root.add_child(AstNode::new(
                    AstNodeType::Expression(StringLiteral),
                    RuntimeType::string(token.value.clone()),
                    Vec::new(),
                ));
                current += 1;
            }
            LexerTokenType::Number => {
                root.add_child(AstNode::new(
                    AstNodeType::Expression(NumberLiteral),
                    RuntimeType::number(token.value.parse().unwrap()),
                    Vec::new(),
                ));
                current += 1;
            }
            LexerTokenType::AddOperator => {
                root.add_child(AstNode::new(
                    AstNodeType::Expression(Expression::Binary(BinaryOperator::AddOperator)),
                    RuntimeType::number(token.value.parse().unwrap()),
                    Vec::new(),
                ));
                current += 1;
            }
            _ => {
                current += 1;
            }
        }
    }

    root
}

// all parsing functions pattern variable takes
// in consideration that the function triggerer token is skipped.
// e.g: block() starts lookahead with ::Any instead of ::OpenCurlyBrace
fn block(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (vec![LexerTokenType::Any], "Expected expression before '}'"),
        (
            vec![LexerTokenType::CloseCurlyBrace],
            "Expected '}' to close a function call",
        ),
    ];

    lookahead(
        pattern,
        tokens,
        current + 1,
        AstNode::new(AstNodeType::Block, RuntimeType::nothing(), Vec::new()),
    )
}

fn group(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (vec![LexerTokenType::Any], "Expected expression before '}'"),
        (
            vec![LexerTokenType::CloseParenthesis],
            "Expected '}' to close a function call",
        ),
    ];

    lookahead(
        pattern,
        tokens,
        current + 1,
        AstNode::new(AstNodeType::Group, RuntimeType::nothing(), Vec::new()),
    )
}

fn function_call(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            vec![LexerTokenType::OpenParenthesis],
            "Expected '(' after function call",
        ),
        (
            vec![LexerTokenType::Any],
            "Something went wrong while parsing a function call",
        ),
        (
            vec![LexerTokenType::CloseParenthesis],
            "Expected ')' to close a function call",
        ),
        (
            vec![LexerTokenType::EndOfStatement],
            "Expected ';' to close a function call",
        ),
    ];

    let root_node_value = tokens[current].value.clone();
    lookahead(
        pattern,
        tokens,
        current + 1,
        AstNode::new(
            AstNodeType::FunctionCall,
            RuntimeType::identifier(root_node_value),
            Vec::new(),
        ),
    )
}

fn assignment_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            vec![LexerTokenType::Identifier],
            "Expected identifier after 'let'",
        ),
        (
            vec![LexerTokenType::AssignmentOperator],
            "Expected '=' after identifier",
        ),
        (
            vec![LexerTokenType::Expression],
            "Expected expression after '='",
        ),
        (
            vec![LexerTokenType::EndOfStatement],
            "Expected ';' after variable declaration",
        ),
    ];

    let root_node_value = tokens[current].value.clone();
    lookahead(
        pattern,
        tokens,
        current + 1,
        AstNode::new(
            AstNodeType::VariableDeclaration,
            RuntimeType::identifier(root_node_value),
            Vec::new(),
        ),
    )
}

fn if_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            vec![LexerTokenType::OpenParenthesis],
            "Expected '(' after if statement",
        ),
        (vec![LexerTokenType::Any], "Bad 'if' structure after '('"),
        (
            vec![LexerTokenType::CloseParenthesis],
            "Expected ')' to close expression on 'if' statement",
        ),
        (
            vec![LexerTokenType::OpenCurlyBrace],
            "Expected '{' after parentheses on 'if' statement",
        ),
        (
            vec![LexerTokenType::Any],
            "Bad 'if' block structure after '{'",
        ),
        (
            vec![LexerTokenType::CloseCurlyBrace],
            "Expected '}' to close 'if' statement block",
        ),
    ];

    let root_node_value = tokens[current].value.clone();
    lookahead(
        pattern,
        tokens,
        current + 1,
        AstNode::new(
            AstNodeType::IfStatement,
            RuntimeType::identifier(root_node_value),
            Vec::new(),
        ),
    )
}

fn lookahead(
    types: Vec<(Vec<LexerTokenType>, &str)>,
    tokens: &Vec<LexerToken>,
    called_on: usize,
    mut root: AstNode,
) -> (usize, AstNode) {
    let mut current = called_on; // first token index
    let mut pattern_index = 0; // pattern type index

    while current < tokens.len() && pattern_index < types.len() {
        let token = &tokens[current];
        let (tokens_types, error_message) = &types[pattern_index];

        //println!("{}----{:?}", token.token_type, tokens_types);

        if tokens_types.contains(&LexerTokenType::Expression) {
            let (tokens_offset, node, error) = lookahead_expression(tokens, current);

            if error {
                error::throw(ErrorType::ExpressionError, error_message, token.line);
            } else {
                current += tokens_offset;
                root.add_child(node);
                pattern_index += 1;
                continue;
            }
        }

        // stop many and manyAny loop
        if tokens_types.contains(&LexerTokenType::Any)
            && types[pattern_index + 1]
                .0
                .contains(&tokens[current].token_type)
        {
            pattern_index += 1;
            continue;
        }

        if tokens_types.contains(&token.token_type) || tokens_types.contains(&LexerTokenType::Any) {
            match token.token_type {
                LexerTokenType::OpenCurlyBrace => {
                    let (offset, block_node) = block(&tokens, current);
                    root.add_child(block_node);
                    current += offset;
                }
                LexerTokenType::OpenParenthesis => {
                    let (index_offset, group_node) = group(&tokens, current);
                    root.add_child(group_node);
                    current += index_offset;
                }
                LexerTokenType::FunctionCall => {
                    let (offset, function_node) = function_call(&tokens, current);
                    root.add_child(function_node);
                    current += offset;
                }
                LexerTokenType::LetKeyword => {
                    let (offset, assignment_node) = assignment_statement(&tokens, current);
                    root.add_child(assignment_node);
                    current += offset;
                }
                LexerTokenType::TrueKeyword => {
                    let boolean = if token.value == "true" { true } else { false };
                    root.add_child(AstNode::new(
                        AstNodeType::Expression(Expression::Boolean(Bool::True)),
                        RuntimeType::boolean(boolean),
                        Vec::new(),
                    ));
                    current += 1;
                }
                LexerTokenType::FalseKeyword => {
                    let boolean = if token.value == "true" { true } else { false };
                    root.add_child(AstNode::new(
                        AstNodeType::Expression(Expression::Boolean(Bool::False)),
                        RuntimeType::boolean(boolean),
                        Vec::new(),
                    ));
                    current += 1;
                }
                LexerTokenType::Identifier => {
                    root.add_child(AstNode::new(
                        AstNodeType::Expression(Identifier),
                        RuntimeType::identifier(token.value.clone()),
                        Vec::new(),
                    ));
                    current += 1;
                }
                LexerTokenType::StringLiteral => {
                    root.add_child(AstNode::new(
                        AstNodeType::Expression(StringLiteral),
                        RuntimeType::string(token.value.clone()),
                        Vec::new(),
                    ));
                    current += 1;
                }
                LexerTokenType::Number => {
                    root.add_child(AstNode::new(
                        AstNodeType::Expression(NumberLiteral),
                        RuntimeType::number(token.value.parse().unwrap()),
                        Vec::new(),
                    ));
                    current += 1;
                }
                LexerTokenType::AddOperator => {
                    root.add_child(AstNode::new(
                        AstNodeType::Expression(Expression::Binary(BinaryOperator::AddOperator)),
                        RuntimeType::string(token.value.clone()),
                        Vec::new(),
                    ));
                    current += 1;
                }
                _ => {
                    current += 1;
                }
            }
        } else {
            error::throw(ErrorType::SyntaxError, error_message, token.line);
        }

        // if Many or ManyAny present, start loop
        if !tokens_types.contains(&LexerTokenType::Any) {
            pattern_index += 1;
        }
    }

    // if we've look less tokens than required minimun by
    // the lookahead function pattern, return error
    if (current - called_on) < types.len() {
        let token = &tokens[current - 1];
        let error_message = types[pattern_index].1;
        error::throw(ErrorType::SyntaxError, error_message, token.line);
        std::process::exit(1);
    }

    ((current - called_on), root) // +1 is for the node who called lookahead
}
