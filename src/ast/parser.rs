use std::vec;

use super::{tree::Boolean, Expression, LexerToken, LexerTokenType};
use crate::{
    ast::{
        AstNode, AstNodeType, AstTree,
        Expression::{BinaryOperator, Identifier, NumberLiteral, StringLiteral},
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
                    AstNodeType::Expression(Expression::Boolean(Boolean::True)),
                    RuntimeType::string(token.value.clone()),
                    Vec::new(),
                ));
                current += 1;
            }
            LexerTokenType::FalseKeyword => {
                root.add_child(AstNode::new(
                    AstNodeType::Expression(Expression::Boolean(Boolean::False)),
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
                    AstNodeType::Expression(BinaryOperator),
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
// e.g: block() starts lookahead with ::ManyAny instead of ::OpenCurlyBrace
fn block(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            vec![LexerTokenType::ManyAny],
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

fn group(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            vec![LexerTokenType::ManyAny],
            "[cei] Expected expression before '}'",
        ),
        (
            vec![LexerTokenType::CloseParenthesis],
            "[cei] Expected '}' to close a function call",
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
            "[cei] Expected '(' after function call",
        ),
        (
            vec![LexerTokenType::ManyAny],
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
    let valid_expressions = vec![
        LexerTokenType::StringLiteral,
        LexerTokenType::Number,
        LexerTokenType::AddOperator,
        LexerTokenType::Identifier,
        LexerTokenType::TrueKeyword,
        LexerTokenType::FalseKeyword,
    ];

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
            vector_of_many(valid_expressions),
            "[cei] Expected expression after '='",
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

fn if_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            vec![LexerTokenType::OpenParenthesis],
            "[cei] Expected '(' after if statement",
        ),
        (
            vec![LexerTokenType::ManyAny],
            "[cei] Bad 'if' structure after '('",
        ),
        (
            vec![LexerTokenType::CloseParenthesis],
            "[cei] Expected ')' to close expression on 'if' statement",
        ),
        (
            vec![LexerTokenType::OpenCurlyBrace],
            "[cei] Expected '{' after parentheses on 'if' statement",
        ),
        (
            vec![LexerTokenType::ManyAny],
            "[cei] Bad 'if' block structure after '{'",
        ),
        (
            vec![LexerTokenType::CloseCurlyBrace],
            "[cei] Expected '}' to close 'if' statement block",
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
    let mut current = called_on; // index of the
    let mut pattern_index = 0;

    while current < tokens.len() && pattern_index < types.len() {
        let token = &tokens[current];
        let (tokens_types, error_message) = &types[pattern_index];

        if tokens_types.contains(&LexerTokenType::ManyAny)
            && types[pattern_index + 1].0.contains(&token.token_type)
        {
            pattern_index += 1; // go to the next pattern item after this iteration
            continue;
        }

        if tokens_types.contains(&token.token_type)
            || tokens_types.contains(&LexerTokenType::ManyAny)
        {
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
                        AstNodeType::Expression(Expression::Boolean(Boolean::True)),
                        RuntimeType::boolean(boolean),
                        Vec::new(),
                    ));
                    current += 1;
                }
                LexerTokenType::FalseKeyword => {
                    let boolean = if token.value == "true" { true } else { false };
                    root.add_child(AstNode::new(
                        AstNodeType::Expression(Expression::Boolean(Boolean::False)),
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
                        AstNodeType::Expression(BinaryOperator),
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
            println!("{}", error_message);
            std::process::exit(1);
        }

        // resolve LexerTokenType::ManyAny type loop
        // check next token type with the next pattern token type
        if !tokens_types.contains(&LexerTokenType::ManyAny) {
            pattern_index += 1;
        }
    }

    ((current - called_on), root) // +1 is for the node who called lookahead
}

fn vector_of_many(mut types: Vec<LexerTokenType>) -> Vec<LexerTokenType> {
    types.push(LexerTokenType::Many);
    types
}
