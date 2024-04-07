use crate::{
    core::error::{self, ErrorType},
    syntax::{
        assignament_statement::{AssignamentNode, VarType},
        block::Block,
        bool::Bool,
        call_expression::CallExpressionNode,
        function_declaration::FunctionDeclaration,
        group::Group,
        identifier::Identifier,
        module::ModuleAst,
        number::Number,
        string_literal::StringLiteral,
        AstNodeType, Expression, LexerToken, LexerTokenType,
    },
};

use super::expressions::{expression, parse_expression};

pub fn parse(tokens: Vec<LexerToken>, module_name: &str) -> ModuleAst {
    let module = ModuleAst::new(module_name);
    tree(tokens, module)
}

fn tree(tokens: Vec<LexerToken>, mut module_ast: ModuleAst) -> ModuleAst {
    let mut current = 0;

    while current < tokens.len() {
        let token = &tokens[current];

        match token.token_type {
            LexerTokenType::FunctionCall => {
                let (index_offset, function_node) = call_expression(&tokens, current);
                module_ast.add_child(function_node);
                current += index_offset;
            }
            LexerTokenType::LetKeyword => {
                let (index_offset, assignment_node) = assignment_statement(&tokens, current);
                module_ast.add_child(assignment_node);
                current += index_offset;
            }
            LexerTokenType::FnKeyword => {
                let (index_offset, assignment_node) = function_declaration(&tokens, current);
                module_ast.add_child(assignment_node);
                current += index_offset;
            }
            LexerTokenType::Identifier => {
                let (index_offset, identifier_node) = identifier(&tokens, current);
                module_ast.add_child(identifier_node);
                current += index_offset;
            }
            LexerTokenType::Number => {
                let (index_offset, number_node) = expression(&tokens, current);
                module_ast.add_child(number_node);
                current += index_offset;
            }
            _ => {
                current += 1;
            }
        }
    }

    module_ast
}

// {}
fn block(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    let mut current = current;
    let mut offset = 0;
    let mut block_node = Block::new();

    // check '{'
    if tokens[current].token_type == LexerTokenType::OpenCurlyBrace {
        current += 1;
        offset += 1;
    } else {
        error::throw(
            ErrorType::SyntaxError,
            format!(
                "Unexpected token '{}' in block opening",
                tokens[current].value
            )
            .as_str(),
            Some(tokens[current].line),
        )
    }

    // get inside block ast nodes & check '}'
    let mut closed = false;

    while current < tokens.len() {
        let token = &tokens[current];

        // offset & current are incremented inside each type
        // to avoid "tokens[overflowed_index]"" if loops ends
        // before a '{'
        match token.token_type {
            LexerTokenType::CloseCurlyBrace => {
                current += 1;
                offset += 1;
                closed = true;
                break; // break block loop since it reaches the end
            }
            LexerTokenType::FunctionCall => {
                let (index_offset, function_node) = call_expression(&tokens, current);
                block_node.add_child(function_node);
                current += index_offset;
                offset += index_offset;
            }
            LexerTokenType::FnKeyword => {
                let (index_offset, assignment_node) = function_declaration(&tokens, current);
                block_node.add_child(assignment_node);
                current += index_offset;
                offset += index_offset;
            }
            LexerTokenType::LetKeyword => {
                let (index_offset, assignment_node) = assignment_statement(&tokens, current);
                block_node.add_child(assignment_node);
                current += index_offset;
                offset += index_offset;
            }
            LexerTokenType::Identifier => {
                let (index_offset, identifier_node) = identifier(&tokens, current);
                block_node.add_child(identifier_node);
                current += index_offset;
                offset += index_offset;
            }
            LexerTokenType::Number => {
                let (index_offset, number_node) = expression(&tokens, current);
                block_node.add_child(number_node);
                current += index_offset;
                offset += index_offset;
            }
            _ => {
                error::throw(
                    ErrorType::SyntaxError,
                    format!(
                        "Unexpected token '{}' inside block {{..}}", // generic error for unexpected codes for block parsing
                        token.value,
                    )
                    .as_str(),
                    Some(token.line),
                );
            }
        }
    }

    // non closed CallExpression
    if !closed {
        error::throw(
            ErrorType::SyntaxError,
            "Expected ')' after block openning",
            Some(tokens[current - 1].line),
        );
    }

    (offset, AstNodeType::Block(block_node))
}

// ()
fn group(tokens: &Vec<LexerToken>, current: usize, context: Option<&str>) -> (usize, AstNodeType) {
    let mut current = current;
    let mut offset = 0;
    let mut group_node = Group::new(tokens[current].at, tokens[current].line);

    // check '('
    if tokens[current].token_type == LexerTokenType::OpenParenthesis {
        current += 1;
        offset += 1;
    } else {
        let context_msg = if let Some(_context) = context {
            format!(" in {}", _context)
        } else {
            "".to_string()
        };
        error::throw(
            ErrorType::SyntaxError,
            format!(
                "Unexpected token '{}'{}",
                tokens[current].value, context_msg
            )
            .as_str(),
            Some(tokens[current].line),
        )
    }

    // get arguments & check ')'
    let mut last_token = None;
    let mut closed = false;

    while current < tokens.len() {
        let token = &tokens[current];

        // offset & current are incremented inside each type
        // to avoid "tokens[overflowed_index]"" if loops ends
        // before a '{'
        match token.token_type {
            LexerTokenType::Comma => {
                if last_token == Some(LexerTokenType::Comma) {
                    group_node.add_child(None);
                }

                last_token = Some(LexerTokenType::Comma);
                current += 1;
                offset += 1;
            }
            LexerTokenType::CloseParenthesis => {
                if last_token == Some(LexerTokenType::Comma) {
                    group_node.add_child(None);
                }

                current += 1;
                offset += 1;
                closed = true;
                break;
            }
            _ => {
                let (node_offset, node) = parse_expression(tokens, current);
                match node {
                    Expression::Identifier(_) => last_token = Some(LexerTokenType::Identifier),
                    Expression::Bool(_) => last_token = Some(LexerTokenType::TrueKeyword),
                    Expression::Number(_) => last_token = Some(LexerTokenType::Number),
                    Expression::StringLiteral(_) => {
                        last_token = Some(LexerTokenType::StringLiteral)
                    }
                    Expression::BinaryExpression(_) => last_token = Some(LexerTokenType::Number),
                }
                current += node_offset;
                offset += node_offset;
                group_node.add_child(Some(node));
            }
        }
    }

    // non closed CallExpression
    if !closed {
        let context_msg = if let Some(context) = context {
            format!(" after {}", context)
        } else {
            "".to_string()
        };
        error::throw(
            ErrorType::SyntaxError,
            format!("Expected ')' {}", context_msg).as_str(),
            Some(tokens[current - 1].line),
        )
    };

    (offset, AstNodeType::Group(group_node))
}

// print(a, b, c)
fn call_expression(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    let mut current = current;
    let mut offset = 0;

    // get the identifier
    let identifier_node = Identifier::new(
        tokens[current].value.clone(),
        tokens[current].at,
        tokens[current].line,
    );
    current += 1;
    offset += 1;

    let (arguments_offset, arguments_node) = group(
        tokens,
        current,
        Some(format!("{}()", identifier_node.name).as_str()),
    );

    let arguments_node = if let AstNodeType::Group(arguments_node) = arguments_node {
        current += arguments_offset;
        offset += arguments_offset;
        arguments_node
    } else {
        error::throw(
            ErrorType::ParsingError,
            "Unexpected node type in CallExpression, expected Group type node",
            Some(tokens[current].line),
        );
        std::process::exit(1);
    };

    // avoid early end of file (idk if it's needed)
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

    // check for final semicolon
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
        AstNodeType::CallExpression(CallExpressionNode::new(
            identifier_node,
            arguments_node,
            at,
            line,
        )),
    )
}

// let a = 20
fn assignment_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    let mut current = current;
    let mut offset = 0;

    // get assignment type: mutable or constant
    let var_type = if tokens[current].value == "let" {
        VarType::Let
    } else {
        VarType::Const
    };

    current += 1;
    offset += 1;

    // get the identifier
    let identifier_node = Identifier::new(
        tokens[current].value.clone(),
        tokens[current].at,
        tokens[current].line,
    );
    current += 1;
    offset += 1;

    // check next token is '='
    if tokens[current].token_type != LexerTokenType::AssignmentOperator {
        error::throw(
            ErrorType::SyntaxError,
            format!("Expected '=' but got '{}'", tokens[current].value).as_str(),
            Some(tokens[current].line),
        )
    };
    current += 1;
    offset += 1;

    // get variable value
    let (expr_offset, expr) = parse_expression(tokens, current);
    current += expr_offset;
    offset += expr_offset;

    // check for final semicolon
    let (at, line) = if tokens[current].token_type == LexerTokenType::EndOfStatement {
        let assignament_statement_properties = (tokens[current].at, tokens[current].line);
        current += 1;
        offset += 1;
        assignament_statement_properties
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
        AstNodeType::AssignamentStatement(AssignamentNode::new(
            identifier_node,
            expr,
            var_type,
            at,
            line,
        )),
    )
}

// fn a() {}
fn function_declaration(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    let mut current = current + 1; // start after "fn" token
    let mut offset = 0;

    // get the identifier
    let identifier_node = Identifier::new(
        tokens[current].value.clone(),
        tokens[current].at,
        tokens[current].line,
    );
    current += 1;
    offset += 1;

    // check for group
    let (arguments_offset, arguments) = group(tokens, current, Some("function declaration"));
    current += arguments_offset;
    offset += arguments_offset;
    let arguments = match arguments {
        AstNodeType::Group(grp) => grp,
        _ => {
            error::throw(
                ErrorType::ParsingError,
                "Expected (...) as function parameters",
                Some(tokens[current].at),
            );
            std::process::exit(1);
        }
    };

    // check for block
    let (block_offset, block_node) = block(tokens, current);
    offset += block_offset;
    // not increment current to avoid invex overflow
    // on the function return
    let function_body = match block_node {
        AstNodeType::Block(b) => b,
        _ => {
            error::throw(
                ErrorType::ParsingError,
                "Expected blockNode as function body",
                Some(tokens[current].at),
            );
            std::process::exit(1);
        }
    };

    (
        offset,
        AstNodeType::FunctionDeclaration(FunctionDeclaration::new(
            identifier_node,
            arguments,
            function_body,
            tokens[current].at,
            tokens[current].line,
        )),
    )
}

// a | a() | a.value | a = 20 + a
fn identifier(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNodeType) {
    let mut current = current;
    let mut offset = 0;

    // get the identifier
    let identifier_node = Identifier::new(
        tokens[current].value.clone(),
        tokens[current].at,
        tokens[current].line,
    );
    current += 1;
    offset += 1;

    // check next token of the identifier
    let (node_offset, node) = match tokens[current].token_type {
        LexerTokenType::OpenParenthesis => {
            // a()
            let (group_offset, group_node) = group(
                tokens,
                current,
                Some(format!("{}()", identifier_node.name).as_str()),
            );

            let group_node = if let AstNodeType::Group(group_node) = group_node {
                current += group_offset;
                offset += group_offset;
                group_node
            } else {
                error::throw(
                    ErrorType::ParsingError,
                    "Unexpected node type in identifier, expected Group type node",
                    Some(tokens[current].line),
                );
                std::process::exit(1);
            };

            let at = group_node.line;
            let line = group_node.line;

            let call_expression_node =
                CallExpressionNode::new(identifier_node, group_node, at, line);
            (offset, AstNodeType::CallExpression(call_expression_node))
        }
        // [Property acess] should handle <a.value> here
        //LexerTokenType::Dot => {}
        // [Variable mutation] should handle <a = ...> here
        //LexerTokenType::AssignamentOperator => {}
        _ => {
            error::throw(
                ErrorType::SyntaxError,
                format!(
                    "Unexpected token '{}' after '{}' identifier",
                    tokens[current].value, identifier_node.name
                )
                .as_str(),
                Some(tokens[current].line),
            );
            std::process::exit(1);
        }
    };

    current += offset;
    offset += node_offset;

    (offset, node)
}

//fn if_statement(tokens: &Vec<LexerToken>, current: usize) -> (usize, AstNode) {}
