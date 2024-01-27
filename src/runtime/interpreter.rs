use super::ScopesStack;
use crate::{
    ast::{AstNodeType, AstTree, Expression},
    core::{handlers::print, types::interpolate},
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
        for node in &mut self.ast.root.children {
            match node.node_type {
                AstNodeType::FunctionCall => {
                    if node.value.to_string() == "print" {
                        print(node.clone(), &self.scopes);
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
                                    value =
                                        Some(interpolate(node.children[current].value.to_string()))
                                }
                                Expression::NumberLiteral => {
                                    value = Some(node.children[current].value.to_string())
                                }
                            },
                            _ => {}
                        }
                        current += 1;
                    }

                    if identifier.is_some() && value.is_some() {
                        self.scopes.add_identifier(
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
