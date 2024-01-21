use crate::{
    ast::{AstNode, AstTokenType, AstTree},
    lexer::{LexerToken, LexerTokenType},
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
            LexerTokenType::PrintKeyword => {
                let (index_offset, print_node) = print_statement(tokens.clone(), current);
                root.add_child(print_node);
                current += index_offset;
            }
            LexerTokenType::LetKeyword => {
                let (index_offset, assignment_node) = assignment_statement(tokens.clone(), current);
                root.add_child(assignment_node);
                current += index_offset;
            }
            LexerTokenType::StringLiteral => {
                root.add_child(AstNode::new(
                    AstTokenType::StringLiteral,
                    token.value.clone(),
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

fn print_statement(tokens: Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            LexerTokenType::StringLiteral,
            "[cei] Expected expression after print",
        ),
        (
            LexerTokenType::EndOfStatement,
            "[cei] Expected ';' to close print statement",
        ),
    ];

    let root_node_value = tokens[current].value.clone();
    lookhead(
        pattern,
        tokens,
        current + 1,
        AstNode::new(AstTokenType::FunctionCall, root_node_value, Vec::new()),
    )
}

fn assignment_statement(tokens: Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let pattern = vec![
        (
            LexerTokenType::Identifier,
            "[cei] Expected identifier after 'let'",
        ),
        (
            LexerTokenType::AssignmentOperator,
            "[cei] Expected '=' after identifier",
        ),
        (
            LexerTokenType::StringLiteral,
            "[cei] Expected string literal after '='",
        ),
        (
            LexerTokenType::EndOfStatement,
            "[cei] Expected ';' after variable declaration",
        ),
    ];

    let root_node_value = tokens[current].value.clone();
    lookhead(
        pattern,
        tokens,
        current + 1,
        AstNode::new(
            AstTokenType::VariableDeclaration,
            root_node_value,
            Vec::new(),
        ),
    )
}

fn lookhead(
    types: Vec<(LexerTokenType, &str)>,
    tokens: Vec<LexerToken>,
    mut current: usize,
    mut root_node: AstNode,
) -> (usize, AstNode) {
    let mut index_offset = 0; // then we will rest

    while current < tokens.len() && index_offset < types.len() {
        let token = &tokens[current];
        let (token_type, error_message) = &types[index_offset];

        current += 1;
        index_offset += 1;

        if token.token_type.to_string() == token_type.to_string() {
            match token.token_type {
                LexerTokenType::Identifier => {
                    root_node.add_child(AstNode::new(
                        AstTokenType::Identifier,
                        token.value.clone(),
                        Vec::new(),
                    ));
                }
                LexerTokenType::StringLiteral => {
                    root_node.add_child(AstNode::new(
                        AstTokenType::StringLiteral,
                        token.value.clone(),
                        Vec::new(),
                    ));
                }
                LexerTokenType::AssignmentOperator => {}
                LexerTokenType::EndOfStatement => {
                    return (index_offset + 1, root_node); // +1 is for the node who called lookahead
                }
                _ => {
                    println!("unexpected token type for recursion")
                    // here goes recursion
                }
            }
        } else {
            println!("{}", error_message);
            std::process::exit(1);
        }
    }

    (index_offset + 1, root_node) // +1 is for the node who called lookahead
}
