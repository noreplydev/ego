use super::ScopesStack;
use crate::{
    ast::{AstNode, AstNodeType, AstTree, Boolean, Expression},
    core::handlers::print,
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
                                Expression::BinaryOperator => {
                                    value = Some(node.children[current].value.to_string())
                                }
                                Expression::Boolean(bool) => match bool {
                                    Boolean::True => {
                                        value = Some(node.children[current].value.to_string())
                                    }
                                    Boolean::False => {
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
                        println!(
                            "[cei] Cannot declare varible '{}'",
                            identifier.unwrap_or("unknown".to_string())
                        );
                        std::process::exit(1);
                    }
                }
                _ => {}
            }
        }
    }
}
