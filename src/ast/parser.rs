use std::cell::Cell;

use crate::{
    ast::{
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
    core::error::{self, ErrorType},
};

use super::{binary_expression::BinaryExpression, if_statement::IfStatement};

pub struct Module {
    module_name: String,
    tokens: Vec<LexerToken>,
    current: Cell<usize>,
}

impl Module {
    pub fn new(module_name: String, tokens: Vec<LexerToken>) -> Module {
        Module {
            module_name,
            tokens,
            current: 0.into(),
        }
    }

    pub fn parse(&mut self) -> ModuleAst {
        let module = ModuleAst::new(&self.module_name);
        self.tree(module)
    }

    // Index handlers:
    // returns directly the node since only next() method
    // changes the current index and it checks if it's overflowed
    fn peek(&self) -> &LexerToken {
        &self.tokens[self.current.get()]
    }

    fn peek_next(&self) -> Option<&LexerToken> {
        if (self.current() + 1) < self.tokens.len() {
            Some(&self.tokens[self.current() + 1])
        } else {
            None
        }
    }

    fn is_peekable(&self) -> bool {
        if self.current() < self.tokens.len() {
            true
        } else {
            false
        }
    }

    fn next(&self) {
        self.current.set(self.current.get() + 1);
    }

    fn current(&self) -> usize {
        self.current.get()
    }

    fn tree(&mut self, mut module_ast: ModuleAst) -> ModuleAst {
        while self.is_peekable() {
            let token = self.peek();

            match token.token_type {
                LexerTokenType::FunctionCall => {
                    let function_node = self.call_expression();
                    module_ast.add_child(function_node);
                }
                LexerTokenType::LetKeyword => {
                    let assignment_node = self.assignment_statement();
                    module_ast.add_child(assignment_node);
                }
                LexerTokenType::FnKeyword => {
                    let function_node = self.function_declaration();
                    module_ast.add_child(function_node);
                }
                LexerTokenType::Identifier => {
                    let identifier_node = self.identifier();
                    module_ast.add_child(identifier_node);
                }
                LexerTokenType::OpenCurlyBrace => {
                    let block_node = self.block();
                    module_ast.add_child(block_node);
                }
                LexerTokenType::IfKeyword => {
                    let block_node = self.if_statement();
                    module_ast.add_child(block_node);
                }
                _ => {
                    self.next();
                }
            }
        }

        module_ast
    }

    // {}
    fn block(&self) -> AstNodeType {
        let mut block_node = Block::new();

        // check '{'
        let token = self.peek();
        if token.token_type == LexerTokenType::OpenCurlyBrace {
            self.next();
        } else {
            error::throw(
                ErrorType::SyntaxError,
                format!("Unexpected token '{}' in block opening", token.value).as_str(),
                Some(token.line),
            )
        }

        // get inside block ast nodes & check '}'
        let mut closed = false;

        while self.is_peekable() {
            let token = self.peek();

            // offset & current are incremented inside each type
            // to avoid "tokens[overflowed_index]"" if loops ends
            // before a '{'
            match token.token_type {
                LexerTokenType::CloseCurlyBrace => {
                    // consume '}'
                    self.next();
                    closed = true;
                    break; // break block loop since it reaches the end
                }
                LexerTokenType::FunctionCall => {
                    let call_node = self.call_expression();
                    block_node.add_child(call_node);
                }
                LexerTokenType::FnKeyword => {
                    let function_node = self.function_declaration();
                    block_node.add_child(function_node);
                }
                LexerTokenType::LetKeyword => {
                    let assignment_node = self.assignment_statement();
                    block_node.add_child(assignment_node);
                }
                LexerTokenType::Identifier => {
                    let identifier_node = self.identifier();
                    block_node.add_child(identifier_node);
                }
                LexerTokenType::Number => {
                    let number_node = self.expression();
                    block_node.add_child(number_node);
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

        // non closed Block
        if !closed {
            error::throw(
                ErrorType::SyntaxError,
                "Expected ')' after block openning",
                Some(token.line),
            );
        }

        AstNodeType::Block(block_node)
    }

    // ()
    fn group(&self, context: Option<&str>) -> AstNodeType {
        // where am i
        let context_msg = match context {
            Some(str) => format!(" in {}", str),
            _ => "".to_string(),
        };

        let group_token = self.peek();
        let mut group_node = Group::new(group_token.at, group_token.line);

        // check '('
        if group_token.token_type == LexerTokenType::OpenParenthesis {
            self.next()
        } else {
            error::throw(
                ErrorType::SyntaxError,
                format!("Unexpected token '{}'{}", group_token.value, context_msg).as_str(),
                Some(group_token.line),
            )
        }

        // get arguments & check ')'
        let mut last_token = None;
        let mut closed = false;

        while self.is_peekable() {
            let token = self.peek();

            // offset & current are incremented inside each type
            // to avoid "tokens[overflowed_index]"" if loops ends
            // before a '{'
            match token.token_type {
                LexerTokenType::Comma => {
                    if last_token == Some(LexerTokenType::Comma) {
                        group_node.add_child(None);
                    }

                    last_token = Some(LexerTokenType::Comma);
                    self.next();
                }
                LexerTokenType::CloseParenthesis => {
                    if last_token == Some(LexerTokenType::Comma) {
                        group_node.add_child(None);
                    }

                    closed = true;
                    break;
                }
                _ => {
                    let node = self.parse_expression();
                    match node {
                        Expression::Identifier(_) => last_token = Some(LexerTokenType::Identifier),
                        Expression::Bool(_) => last_token = Some(LexerTokenType::TrueKeyword),
                        Expression::Number(_) => last_token = Some(LexerTokenType::Number),
                        Expression::StringLiteral(_) => {
                            last_token = Some(LexerTokenType::StringLiteral)
                        }
                        Expression::BinaryExpression(_) => {
                            last_token = Some(LexerTokenType::Number)
                        }
                    }
                    group_node.add_child(Some(node));
                }
            }
        }

        // non closed CallExpression
        if !closed {
            error::throw(
                ErrorType::SyntaxError,
                format!("Expected ')' {}", context_msg).as_str(),
                Some(group_node.line),
            )
        };

        // consume ')'
        self.next();
        AstNodeType::Group(group_node)
    }

    // print(a, b, c)
    fn call_expression(&self) -> AstNodeType {
        // get the identifier
        let identifier_token = self.peek();
        let identifier_node = Identifier::new(
            identifier_token.value.clone(),
            identifier_token.at,
            identifier_token.line,
        );

        // consume identifier
        self.next();

        let arguments_node = self.group(Some(format!("{}()", identifier_node.name).as_str()));

        let arguments_node = if let AstNodeType::Group(arguments_node) = arguments_node {
            arguments_node
        } else {
            error::throw(
                ErrorType::ParsingError,
                "Unexpected node type in CallExpression, expected Group type node",
                Some(identifier_token.line),
            );
            std::process::exit(1);
        };

        // avoid early end of file (idk if it's needed)
        if !self.is_peekable() {
            error::throw(
                ErrorType::SyntaxError,
                format!(
                    "Expected ';' but got '{}' as end of statement",
                    identifier_token.value
                )
                .as_str(),
                Some(identifier_token.line),
            )
        }

        let token = self.peek();
        // check for final semicolon
        let (at, line) = if token.token_type == LexerTokenType::EndOfStatement {
            let call_expression_properties = (token.at, token.line);
            // consume ';'
            self.next();
            call_expression_properties
        } else {
            error::throw(
                ErrorType::SyntaxError,
                format!("Expected ';' but got '{}' as end of statement", token.value).as_str(),
                Some(token.line),
            );
            std::process::exit(1); // for type checking
        };

        AstNodeType::CallExpression(CallExpressionNode::new(
            identifier_node,
            arguments_node,
            at,
            line,
        ))
    }

    // let a = 20
    fn assignment_statement(&self) -> AstNodeType {
        let token = self.peek();
        // get assignment type: mutable or constant
        let var_type = if token.value == "let" {
            VarType::Let
        } else {
            VarType::Const
        };

        // consume var type
        self.next();

        // get the identifier
        let token = self.peek();
        let identifier_node = Identifier::new(token.value.clone(), token.at, token.line);
        // consume identifier
        self.next();

        let token = self.peek();
        // check next token is '='
        if token.token_type != LexerTokenType::AssignmentOperator {
            error::throw(
                ErrorType::SyntaxError,
                format!("Expected '=' but got '{}'", token.value).as_str(),
                Some(token.line),
            )
        };
        // consume '='
        self.next();

        // get variable value
        let expr = self.parse_expression();

        // check for final semicolon
        let token = self.peek();
        let (at, line) = if token.token_type == LexerTokenType::EndOfStatement {
            let assignament_statement_properties = (token.at, token.line);
            // consume ';'
            self.next();
            assignament_statement_properties
        } else {
            error::throw(
                ErrorType::SyntaxError,
                format!("Expected ';' but got '{}' as end of statement", token.value).as_str(),
                Some(token.line),
            );
            std::process::exit(1); // for type checking
        };

        AstNodeType::AssignamentStatement(AssignamentNode::new(
            identifier_node,
            expr,
            var_type,
            at,
            line,
        ))
    }

    // fn a() {...}
    fn function_declaration(&self) -> AstNodeType {
        // consume 'fn' keyword
        self.next();

        // get the identifier
        let token = self.peek();
        let identifier_node = Identifier::new(token.value.clone(), token.at, token.line);
        // consume identifier
        self.next();

        // check for group
        let arguments = self.group(Some("function declaration"));
        let arguments = match arguments {
            AstNodeType::Group(grp) => grp,
            _ => {
                error::throw(
                    ErrorType::ParsingError,
                    "Expected (...) as function parameters",
                    Some(self.peek().at),
                );
                std::process::exit(1);
            }
        };

        // check for block
        let token = self.peek();
        let block_node = self.block();
        let function_body = match block_node {
            AstNodeType::Block(b) => b,
            _ => {
                error::throw(
                    ErrorType::ParsingError,
                    "Expected blockNode as function body",
                    Some(token.line),
                );
                std::process::exit(1);
            }
        };

        AstNodeType::FunctionDeclaration(FunctionDeclaration::new(
            identifier_node,
            arguments,
            function_body,
            token.at,
            token.line,
        ))
    }

    // if (true) {...}
    fn if_statement(&self) -> AstNodeType {
        // consume 'if' keyword
        let at = self.peek().at;
        let line = self.peek().line;

        // check '('
        self.next();
        let token = self.peek();
        if token.token_type != LexerTokenType::OpenParenthesis {
            error::throw(
                ErrorType::SyntaxError,
                format!("Unexpected token '{}'", token.value).as_str(),
                Some(token.line),
            );
        }

        // consume expression
        self.next();
        let expr = self.expression();
        let expr_node = match expr {
            AstNodeType::Expression(b) => b,
            _ => {
                error::throw(
                    ErrorType::ParsingError,
                    "Expected blockNode as if arm",
                    Some(token.line),
                );
                std::process::exit(1);
            }
        };

        // consume ')'
        self.next();
        let token = self.peek();
        if token.token_type == LexerTokenType::CloseParenthesis {
            error::throw(
                ErrorType::SyntaxError,
                format!("Unexpected token '{}' in if statement", token.value).as_str(),
                Some(token.line),
            )
        }

        // consume '{'
        let token = self.peek();
        if token.token_type != LexerTokenType::OpenCurlyBrace {
            error::throw(
                ErrorType::SyntaxError,
                format!("Unexpected token '{}' in if statement", token.value).as_str(),
                Some(token.line),
            )
        }

        let block = self.block();
        let block_node = match block {
            AstNodeType::Block(b) => b,
            _ => {
                error::throw(
                    ErrorType::ParsingError,
                    "Expected blockNode as if arm",
                    Some(token.line),
                );
                std::process::exit(1);
            }
        };

        AstNodeType::IfStatement(IfStatement::new(expr_node, block_node, at, line))
    }

    // a | a() | a.value | a = 20 + a
    fn identifier(&self) -> AstNodeType {
        let token = self.peek();
        // get the identifier
        let identifier_node = Identifier::new(token.value.clone(), token.at, token.line);
        // check next token of the identifier without consuming
        let token = self.peek_next();

        let node = match token {
            Some(next) => match next.token_type {
                LexerTokenType::OpenParenthesis => {
                    // a();
                    self.call_expression()
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
                            next.value, identifier_node.name
                        )
                        .as_str(),
                        Some(next.line),
                    );
                    std::process::exit(1);
                }
            },
            None => AstNodeType::Expression(Expression::Identifier(identifier_node)),
        };

        node
    }

    // (2 * 2) + 3
    fn expression(&self) -> AstNodeType {
        let expr = self.parse_expression();
        AstNodeType::Expression(expr)
    }

    // 2 + 3 * 23
    fn parse_expression(&self) -> Expression {
        // will autoincrement current
        // and it will be the root or the left node
        // depending on the expression
        let mut node = self.parse_term();
        while self.is_peekable() {
            let token = self.peek();
            match token.token_type {
                LexerTokenType::AddOperator | LexerTokenType::SubtractOperator => {
                    let operator = if let Some(op) = token.value.chars().next() {
                        op
                    } else {
                        error::throw(
                            ErrorType::ParsingError,
                            format!("Operator '{}' cannot be parsed as char", token.value).as_str(),
                            Some(token.line),
                        );
                        std::process::exit(1);
                    };

                    // consume the operator
                    self.next();

                    // get right node
                    let right = self.parse_term();
                    node = Expression::BinaryExpression(BinaryExpression::new(
                        operator,
                        Box::new(node),
                        Box::new(right),
                        token.at,
                        token.line,
                    ));
                }
                _ => break,
            }
        }

        node
    }

    fn parse_term(&self) -> Expression {
        // will autoincrement current
        // and it will be the root or the left node
        // depending on the expression
        let mut node = self.parse_factor();

        while self.is_peekable() {
            let token = self.peek();
            match token.token_type {
                LexerTokenType::MultiplyOperator | LexerTokenType::DivideOperator => {
                    let operator = if let Some(op) = token.value.chars().next() {
                        op
                    } else {
                        error::throw(
                            ErrorType::ParsingError,
                            format!("Operator '{}' cannot be parsed as char", token.value).as_str(),
                            Some(token.line),
                        );
                        std::process::exit(1);
                    };

                    // consume the operator
                    self.next();

                    // get right node
                    let right = self.parse_factor();
                    node = Expression::BinaryExpression(BinaryExpression::new(
                        operator,
                        Box::new(node),
                        Box::new(right),
                        token.at,
                        token.line,
                    ));
                }
                _ => break,
            }
        }

        node
    }

    fn parse_factor(&self) -> Expression {
        let token = self.peek();
        match token.token_type {
            LexerTokenType::OpenParenthesis => {
                self.next(); // to consume the '('
                let expr = self.parse_expression();

                if token.token_type == LexerTokenType::CloseParenthesis {
                    self.next(); // to consume the ')'
                    expr
                } else {
                    error::throw(
                        ErrorType::ParsingError,
                        format!("Unexpected token '{}', expected ')'", token.value).as_str(),
                        Some(token.line),
                    );
                    std::process::exit(1);
                }
            }
            LexerTokenType::Number => {
                let number_node = Number::from_string(token.value.clone(), token.at, token.line);

                if let Some(node) = number_node {
                    self.next(); // consume number itself
                    Expression::Number(node)
                } else {
                    error::throw(
                        ErrorType::ParsingError,
                        format!("Invalid token '{}' inside of a expression", token.value).as_str(),
                        Some(token.line),
                    );
                    std::process::exit(1);
                }
            }
            LexerTokenType::TrueKeyword | LexerTokenType::FalseKeyword => {
                let token = self.peek();
                let node = if let Ok(bool_value) = token.value.parse::<bool>() {
                    Bool::new(bool_value, token.at, token.line)
                } else {
                    error::throw(
                        ErrorType::ParsingError,
                        format!("Invalid token '{}' inside of a expression", token.value).as_str(),
                        Some(token.line),
                    );
                    std::process::exit(1);
                };

                self.next(); // consume keyword
                Expression::Bool(node)
            }
            LexerTokenType::StringLiteral => {
                self.next(); // consume string literal
                Expression::StringLiteral(StringLiteral::new(
                    token.value.clone(),
                    token.at,
                    token.line,
                ))
            }
            LexerTokenType::Identifier => {
                self.next(); // consume identifier
                Expression::Identifier(Identifier::new(token.value.clone(), token.at, token.line))
            }
            _ => {
                error::throw(
                    error::ErrorType::SyntaxError,
                    format!("Invalid token '{}' inside of a expression", token.value).as_str(),
                    Some(token.line),
                );
                std::process::exit(1);
            }
        }
    }
}
