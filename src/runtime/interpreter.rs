use crate::{
    core::{handlers::print::print, runtypes::RuntimeType},
    syntax::{module::ModuleAst, AstNodeType, Expression},
};

use super::ScopesStack;

pub fn exec(ast: ModuleAst) {
    let mut scopes = ScopesStack::new();

    let mut counter = 0;
    while counter < ast.children.len() {
        exec_node(&ast.children[counter], &mut scopes);
        counter += 1;
    }
}

fn exec_node(node: &AstNodeType, scopes: &mut ScopesStack) {
    match node {
        AstNodeType::CallExpression(node) => {
            let runtime_arguments: Vec<RuntimeType> = node
                .arguments
                .children
                .iter()
                .map(|arg| -> RuntimeType {
                    if let Some(arg) = arg {
                        match arg {
                            Expression::StringLiteral(string) => {
                                RuntimeType::string(string.value.clone())
                            }
                            Expression::Number(number) => RuntimeType::number(number.value),
                            Expression::Bool(bool) => RuntimeType::boolean(bool.value),
                            Expression::Identifier(ident) => {
                                RuntimeType::identifier(ident.name.to_string())
                            }
                        }
                    } else {
                        RuntimeType::nothing()
                    }
                })
                .collect();

            match node.identifier.name.as_str() {
                "print" => print(runtime_arguments, &scopes),
                _ => {
                    // runtime declared functions
                }
            }
        }
        AstNodeType::AssignamentStatement(node) => {
            let value_as_runtype = match &node.init {
                Expression::Bool(b) => RuntimeType::boolean(b.value),
                Expression::Number(n) => RuntimeType::number(n.value),
                Expression::StringLiteral(s) => RuntimeType::string(s.value.clone()),
                // here will go identifier when assigning one variable to another
                _ => RuntimeType::nothing(),
            };
            scopes.add_identifier(node.identifier.name.clone(), value_as_runtype);

            println!("{:#?}", scopes)
        }
        _ => {}
    }
}

/* use super::ScopesStack;
use crate::{
    core::{
        error::{self, ErrorType},
        handlers::print,
    },
    syntax::{AstNode, AstNodeType, AstTree, BinaryOperator, Bool, Expression},
};

pub struct Interpreter {
    scopes: ScopesStack,
    ast: AstTree,
}

impl Interpreter {
    pub fn new(scopes: ScopesStack, ast: AstTree) -> Interpreter {
        Interpreter { scopes, ast }
    }

    pub fn exec(&mut self) {
        Self::exec_block(&mut self.ast.root, &mut self.scopes);
    }

    fn exec_block(node: &mut AstNode, scopes: &mut ScopesStack) {
        for node in &mut node.children {
            match node.node_type {
                AstNodeType::Block => {
                    scopes.push();
                    Self::exec_block(node, scopes);
                    scopes.pop();
                }
                AstNodeType::IfStatement => {
                    if node.children.len() < 1 {
                        error::throw(ErrorType::SyntaxError, "'if' statement AST node was provided with no children. This occurs if an 'if' statement has no group and block nodes inside of it", None);
                    }

                    if Self::resolve_group(&node.children[0]) {
                        scopes.push();
                        Self::exec_block(&mut node.children[1], scopes);
                        scopes.pop();
                    }
                }
                AstNodeType::Group => {
                    // todo
                }
                AstNodeType::FunctionCall => {
                    if node.value.to_string() == "print" {
                        print(node.clone(), scopes);
                    }
                }
                AstNodeType::VariableDeclaration => {
                    let mut current = 0;
                    let mut identifier = None;
                    let mut value = None;

                    while current < node.children.len() {
                        match node.children[current].node_type {
                            AstNodeType::Expression(exp) => match exp {
                                Expression::Identifier => {
                                    identifier = Some(node.children[current].value.to_string())
                                }
                                Expression::StringLiteral => {
                                    value = Some(node.children[current].value.to_string())
                                }
                                Expression::NumberLiteral => {
                                    value = Some(node.children[current].value.to_string())
                                }
                                Expression::Binary(bin) => match bin {
                                    // TODO: catch cases well instead of catch all
                                    _ => value = Some(node.children[current].value.to_string()),
                                },
                                Expression::Boolean(bool) => match bool {
                                    Bool::True => {
                                        value = Some(node.children[current].value.to_string())
                                    }
                                    Bool::False => {
                                        value = Some(node.children[current].value.to_string())
                                    }
                                },
                            },
                            _ => {}
                        }
                        current += 1;
                    }

                    if identifier.is_some() && value.is_some() {
                        scopes.add_identifier(
                            identifier.unwrap().to_string(),
                            value.unwrap().to_string(),
                        );
                    } else {
                        let error_message = format!(
                            "Cannot declare varible '{}'",
                            identifier.unwrap_or("unknown".to_string())
                        );
                        error::throw(ErrorType::SyntaxError, &error_message, None);
                    }
                }
                _ => {}
            }
        }
    }

    // this is a use-case fitted solution, but i need
    // more conceptual development of the expressions
    // to know how to handle them well
    fn resolve_group(node: &AstNode) -> bool {
        true
    }
}
 */
