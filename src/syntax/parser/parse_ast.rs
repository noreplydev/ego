use crate::{
    core::error::{self, ErrorType},
    syntax::{
        call_expression::CallExpressionNode, identifier::IdentifierNode, module::ModuleAst,
        string_literal::StringLiteral, AstNodeType, LexerToken, LexerTokenType,
    },
};

pub fn parse(tokens: Vec<LexerToken>, module_name: &str) -> ModuleAst {
    let module = ModuleAst::new(module_name);
    tree(tokens, module)
}

fn tree(tokens: Vec<LexerToken>, mut module_ast: ModuleAst) -> ModuleAst {
    let mut current = 0;

    while current < tokens.len() {
        let token = &tokens[current];

        match token.token_type {
            /*             LexerTokenType::OpenCurlyBrace => {
                let (index_offset, block_node) = block(&tokens, current);
                root.add_child(block_node);
                current += index_offset;
            }
            LexerTokenType::OpenParenthesis => {
                let (index_offset, group_node) = group(&tokens, current);
                root.add_child(group_node);
                current += index_offset;
            } */
            LexerTokenType::FunctionCall => {
                let (index_offset, function_node) = function_call(&tokens, current);
                module_ast.add_child(function_node);
                current += index_offset;
            } /*
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
            } */
            _ => {
                current += 1;
            }
        }
    }

    module_ast
}

// all parsing functions pattern variable takes
// in consideration that the function triggerer token is skipped.
// e.g: block() starts lookahead with ::Any instead of ::OpenCurlyBrace
/* fn block(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {}

fn group(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {} */

fn function_call(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    let mut current = current;
    let mut offset = 0;

    // get the identifier
    let identifier_node = IdentifierNode::new(
        tokens[current].value.clone(),
        tokens[current].line,
        tokens[current].at,
    );
    current += 1;
    offset += 1;

    // check '('
    if tokens[current].token_type == LexerTokenType::OpenParenthesis {
        current += 1;
        offset += 1;
    } else {
        error::throw(
            ErrorType::SyntaxError,
            format!(
                "Unexpected token '{}' in function call",
                tokens[current].value
            )
            .as_str(),
            Some(tokens[current].line),
        )
    }

    // get arguments
    let mut arguments: Vec<Option<AstNodeType>> = vec![];
    let mut last_token = None;
    while current < tokens.len() {
        let token = &tokens[current];

        // offset & current are incremented inside each type
        // to avoid "tokens[overflowed_index]"" if loops ends
        // before a '{'
        println!("{:#?}", last_token);
        match token.token_type {
            LexerTokenType::Comma => {
                if last_token == Some(LexerTokenType::Comma) {
                    arguments.push(None);
                }

                last_token = Some(LexerTokenType::Comma);
                current += 1;
                offset += 1;
            }
            LexerTokenType::StringLiteral => {
                last_token = Some(LexerTokenType::StringLiteral);
                arguments.push(Some(AstNodeType::StringLiteral(StringLiteral::new(
                    token.value.clone(),
                    token.at,
                    token.line,
                ))));
                current += 1;
                offset += 1;
            }
            LexerTokenType::CloseParenthesis => {
                current += 1;
                offset += 1;
                break;
            }
            _ => {
                error::throw(
                    ErrorType::SyntaxError,
                    format!(
                        "Unexpected token '{}' as argument for {}(..)",
                        token.value, identifier_node.name
                    )
                    .as_str(),
                    Some(token.line),
                );
            }
        }
    }

    // avoid early end of file
    if current >= tokens.len() {
        error::throw(
            ErrorType::SyntaxError,
            format!(
                "Expected ';' but got '{}' as end of statement",
                tokens[current - 1].value
            )
            .as_str(),
            Some(tokens[current - 1].line),
        )
    }

    // get final ;
    if tokens[current].token_type == LexerTokenType::EndOfStatement {
        current += 1;
        offset += 1;
    } else {
        error::throw(
            ErrorType::SyntaxError,
            format!(
                "Expected ';' but got '{}' as end of statement",
                tokens[current].value
            )
            .as_str(),
            Some(tokens[current].line),
        )
    }

    (
        offset,
        AstNodeType::FunctionCall(CallExpressionNode::new(identifier_node, arguments, 0, 0)),
    )
}
/*
fn assignment_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {}

fn if_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {} */
