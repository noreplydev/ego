use crate::{
    ast::{AstNode, AstTokenType, AstTree},
    lexer::{LexerToken, LexerTokenType},
};

pub fn parse(tokens: Vec<LexerToken>) -> AstTree {
    let tree = branch(tokens);
    let ast = AstTree::new(tree);
    return ast;
}

fn branch(branch_tokens: Vec<LexerToken>) -> AstNode {
    let mut current = 0;
    let mut root = AstNode::root();

    while current < branch_tokens.len() {
        let token = &branch_tokens[current];

        match token.token_type {
            LexerTokenType::ExpressionStatement => {
                if token.value == "print" {
                    let (index_offset, print_node) =
                        print_statement(branch_tokens.clone(), current);
                    root.add_child(print_node);
                    current += index_offset;
                }
            }
            LexerTokenType::StringLiteral => {
                root.add_child(AstNode::new(
                    AstTokenType::StringLiteral,
                    token.value.clone(),
                    Vec::new(),
                ));
                current += 1;
            }
            _ => {}
        }
    }

    root
}

fn print_statement(tokens: Vec<LexerToken>, current: usize) -> (usize, AstNode) {
    let mut index_offset = 1;
    let current_token = &tokens[current];
    let mut print_root_node = AstNode::new(
        AstTokenType::FunctionCall,
        current_token.value.clone(),
        Vec::new(),
    );

    match &tokens[current + 1].token_type {
        LexerTokenType::StringLiteral => {
            print_root_node.add_child(AstNode::new(
                AstTokenType::StringLiteral,
                tokens[current + 1].value.clone(),
                Vec::new(),
            ));
            index_offset += 1;
        }
        _ => {
            panic!("[goru] Expected a string literal after print");
        }
    }

    (index_offset, print_root_node)
}
