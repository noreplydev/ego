use std::vec;

use super::{LexerToken, LexerTokenType};
use crate::{
    ast::{
        AstNode, AstNodeType, AstTree,
        Expression::{Identifier, NumberLiteral, StringLiteral},
    },
    core::types::RuntimeType,
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
                println!("repertirlo otra vez {index_offset}");
                current += index_offset;
            }
            LexerTokenType::FunctionCall => {
                println!("function call boy");
                let (index_offset, print_node) = function_call(&tokens, current);
                root.add_child(print_node);
                current += index_offset;
            }
            LexerTokenType::LetKeyword => {
                let (index_offset, assignment_node) = assignment_statement(&tokens, current);
                root.add_child(assignment_node);
                current += index_offset;
            }
            LexerTokenType::OpenParenthesis => {
                /*                 let (index_offset, assignment_node) = group_expression(&tokens, current);
                root.add_child(assignment_node);
                current += index_offset; */
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
        (
            vec![LexerTokenType::Any],
            "[cei] Expected expression before '}'",
        ),
        (
            vec![LexerTokenType::CloseCurlyBrace],
            "[cei] Expected '}' to close a function call",
        ),
    ];

    lookahead(
        pattern,
        tokens,
        current + 1,
        AstNode::new(AstNodeType::Block, RuntimeType::nothing(), Vec::new()),
    )
}

fn function_call(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            vec![LexerTokenType::OpenParenthesis],
            "[cei] Expected '(' after function call",
        ),
        (
            vec![LexerTokenType::Any],
            "[cei] Something went wrong while parsing a function call",
        ),
        (
            vec![LexerTokenType::CloseParenthesis],
            "[cei] Expected ')' to close a function call",
        ),
        (
            vec![LexerTokenType::EndOfStatement],
            "[cei] Expected ';' to close a function call",
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
            "[cei] Expected identifier after 'let'",
        ),
        (
            vec![LexerTokenType::AssignmentOperator],
            "[cei] Expected '=' after identifier",
        ),
        (
            vec![LexerTokenType::StringLiteral, LexerTokenType::Number],
            "[cei] Expected value after '='",
        ),
        (
            vec![LexerTokenType::EndOfStatement],
            "[cei] Expected ';' after variable declaration",
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

fn lookahead(
    types: Vec<(Vec<LexerTokenType>, &str)>,
    tokens: &Vec<LexerToken>,
    called_on: usize,
    mut root: AstNode,
) -> (usize, AstNode) {
    let mut current = called_on; // index of the
    let mut pattern_index = 0;

    while current < tokens.len() && pattern_index < types.len() {
        let token = &tokens[current];
        let (tokens_types, error_message) = &types[pattern_index];

        if tokens_types.contains(&LexerTokenType::Any)
            && types[pattern_index + 1].0.contains(&token.token_type)
        {
            pattern_index += 1; // go to the next pattern item after this iteration
            continue;
        }

        if tokens_types.contains(&token.token_type) || tokens_types.contains(&LexerTokenType::Any) {
            match token.token_type {
                LexerTokenType::OpenCurlyBrace => {
                    let (offset, block_node) = block(&tokens, current);
                    root.add_child(block_node);
                    current += offset;
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
                _ => {
                    current += 1;
                }
            }
        } else {
            println!("{}", error_message);
            std::process::exit(1);
        }

        // resolve LexerTokenType::Any type loop
        // check next token type with the next pattern token type
        if !tokens_types.contains(&LexerTokenType::Any) {
            pattern_index += 1;
        }
    }

    ((current - called_on), root) // +1 is for the node who called lookahead
}
