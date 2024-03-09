use std::{os::unix::process, process::exit};

use crate::{
    core::error::{self, ErrorType},
    syntax::{
        bool::Bool, call_expression::CallExpressionNode, identifier::IdentifierNode,
        module::ModuleAst, number::Number, string_literal::StringLiteral, AstNodeType, LexerToken,
        LexerTokenType,
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
                let (index_offset, function_node) = call_expression(&tokens, current);
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

// print(a, b, c)
fn call_expression(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    let mut current = current;
    let mut offset = 0;

    // get the identifier
    let identifier_node = IdentifierNode::new(
        tokens[current].value.clone(),
        tokens[current].at,
        tokens[current].line,
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

    // get arguments & check ')'
    let mut arguments: Vec<Option<AstNodeType>> = vec![];
    let mut last_token = None;
    while current < tokens.len() {
        let token = &tokens[current];

        // offset & current are incremented inside each type
        // to avoid "tokens[overflowed_index]"" if loops ends
        // before a '{'
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
            LexerTokenType::Number => {
                last_token = Some(LexerTokenType::Number);
                let number: Result<i64, _> = token.value.parse();

                if let Ok(number) = number {
                    arguments.push(Some(AstNodeType::Number(Number::new(
                        number, token.at, token.line,
                    ))));
                    current += 1;
                    offset += 1;
                } else {
                    error::throw(
                        ErrorType::ParsingError,
                        format!("Types inferece error for '{}'", token.value).as_str(),
                        Some(token.line),
                    )
                }
            }
            LexerTokenType::TrueKeyword | LexerTokenType::FalseKeyword => {
                last_token = Some(LexerTokenType::TrueKeyword); // let's say always true, but doesn't matter at all
                let bool_value: Result<bool, _> = token.value.parse();
                if let Ok(bool_value) = bool_value {
                    arguments.push(Some(AstNodeType::Bool(Bool::new(
                        bool_value, token.at, token.line,
                    ))));
                    current += 1;
                    offset += 1;
                }
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
    let (at, line) = if tokens[current].token_type == LexerTokenType::EndOfStatement {
        let call_expression_properties = (tokens[current].at, tokens[current].line);
        current += 1;
        offset += 1;
        call_expression_properties
    } else {
        error::throw(
            ErrorType::SyntaxError,
            format!(
                "Expected ';' but got '{}' as end of statement",
                tokens[current].value
            )
            .as_str(),
            Some(tokens[current].line),
        );
        std::process::exit(1); // for type checking
    };

    (
        offset,
        AstNodeType::FunctionCall(CallExpressionNode::new(
            identifier_node,
            arguments,
            at,
            line,
        )),
    )
}

/*
fn assignment_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {}

fn if_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {} */
