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

use super::expressions::expression;

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
            /*
            LexerTokenType::IfKeyword => {
            let (index_offset, if_node) = if_statement(&tokens, current);
            root.add_child(if_node);
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
            _ => {
                error::throw(
                    ErrorType::SyntaxError,
                    format!(
                        "Unexpected token '{}'", // generic error for unexpected codes for block parsing
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
            LexerTokenType::StringLiteral => {
                last_token = Some(LexerTokenType::StringLiteral);
                group_node.add_child(Some(Expression::StringLiteral(StringLiteral::new(
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
                    group_node.add_child(Some(Expression::Number(Number::new(
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
                    group_node.add_child(Some(Expression::Bool(Bool::new(
                        bool_value, token.at, token.line,
                    ))));
                    current += 1;
                    offset += 1;
                }
            }
            LexerTokenType::Identifier => {
                last_token = Some(LexerTokenType::Identifier);
                group_node.add_child(Some(Expression::Identifier(Identifier::new(
                    token.value.clone(),
                    token.at,
                    token.line,
                ))));
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
                let context_msg = if let Some(context) = context {
                    format!(" as argument for {}", context)
                } else {
                    "".to_string()
                };
                error::throw(
                    ErrorType::SyntaxError,
                    format!("Unexpected token '{}'{}", token.value, context_msg).as_str(),
                    Some(token.line),
                );
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
        arguments_node
    } else {
        error::throw(
            ErrorType::ParsingError,
            "Unexpected node type in CallExpression, expected Group type node",
            Some(tokens[current].line),
        );
        std::process::exit(1);
    };

    current += arguments_offset;
    offset += arguments_offset;

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
    let token = &tokens[current];
    let node_init: Expression = match token.token_type {
        LexerTokenType::StringLiteral => Expression::StringLiteral(StringLiteral::new(
            token.value.clone(),
            token.at,
            token.line,
        )),
        LexerTokenType::Number => {
            let num_value: Result<i64, _> = token.value.parse();
            if let Ok(num_value) = num_value {
                Expression::Number(Number::new(num_value, token.at, token.line))
            } else {
                error::throw(
                    ErrorType::ParsingError,
                    format!("Cannot parse '{}' as Number", token.value).as_str(),
                    Some(token.line),
                );
                std::process::exit(1);
            }
        }
        LexerTokenType::TrueKeyword | LexerTokenType::FalseKeyword => {
            let bool_value: Result<bool, _> = token.value.parse();
            if let Ok(bool_value) = bool_value {
                Expression::Bool(Bool::new(bool_value, token.at, token.line))
            } else {
                error::throw(
                    ErrorType::ParsingError,
                    format!("Cannot parse '{}' as Boolean", token.value).as_str(),
                    Some(token.line),
                );
                std::process::exit(1);
            }
        }
        _ => {
            error::throw(
                ErrorType::SyntaxError,
                format!(
                    "Expected '<expression>' but got '{}'",
                    tokens[current].value
                )
                .as_str(),
                Some(tokens[current].line),
            );
            std::process::exit(1);
        }
    };

    current += 1;
    offset += 1;

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
            node_init,
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
    let mut arguments: Vec<Option<Expression>> = vec![];
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
                    arguments.push(None);
                }

                last_token = Some(LexerTokenType::Comma);
                current += 1;
                offset += 1;
            }
            LexerTokenType::StringLiteral => {
                last_token = Some(LexerTokenType::StringLiteral);
                arguments.push(Some(Expression::StringLiteral(StringLiteral::new(
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
                    arguments.push(Some(Expression::Number(Number::new(
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
                    arguments.push(Some(Expression::Bool(Bool::new(
                        bool_value, token.at, token.line,
                    ))));
                    current += 1;
                    offset += 1;
                }
            }
            LexerTokenType::Identifier => {
                last_token = Some(LexerTokenType::Identifier);
                arguments.push(Some(Expression::Identifier(Identifier::new(
                    token.value.clone(),
                    token.at,
                    token.line,
                ))));
                current += 1;
                offset += 1;
            }
            LexerTokenType::CloseParenthesis => {
                if last_token == Some(LexerTokenType::Comma) {
                    arguments.push(None);
                }

                current += 1;
                offset += 1;
                closed = true;
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

    // non closed FunctionDeclaration
    if !closed {
        error::throw(
            ErrorType::SyntaxError,
            "Expected ')' after function arguments",
            Some(tokens[current - 1].line),
        )
    }

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

    // check for block
    let (block_offset, block_node) = block(tokens, current);
    offset += block_offset;
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
            (
                group_offset,
                AstNodeType::CallExpression(call_expression_node),
            )
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
