use super::ScopesStack;
use crate::{
    ast::{AstNodeType, AstTree, Expression},
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
        for node in &mut self.ast.root.children {
            match node.node_type {
                AstNodeType::FunctionCall => {
                    if node.value == "print" {
                        print(node.clone(), &self.scopes);
                    }
                }
                AstNodeType::VariableDeclaration => {
                    let mut current = 0;
                    let mut identifier = None;
                    let mut value = None;

                    while current < node.children.len() {
                        match node.children[current].node_type {
                            AstNodeType::Identifier => {
                                identifier = Some(node.children[current].value.clone())
                            }
                            AstNodeType::Expression(exp) => match exp {
                                Expression::StringLiteral => {
                                    value = Some(node.children[current].value.clone())
                                }
                                Expression::NumberLiteral => {
                                    value = Some(node.children[current].value.clone())
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

    pub fn child_exec(&mut self) {}
}

/*

pub fn new(scopes: ScopesStack, ast: AstTree) -> Interpreter {
       Interpreter { scopes, ast }
   }

   pub fn execute(&mut self) {
       let root = self.execute_node(self.ast.root);
       self.ast.root = root
   }

   fn execute_node(&mut self, node: AstNode) -> AstNode {
       match node.node_type {
           AstNodeType::Root => {
               for child in node.children {
                   self.execute_node(child);
               }
           }
           AstNodeType::FunctionCall => {
               if node.value == "print" {
                   Self::execute_print(&node);
               }
           }
           AstNodeType::VariableDeclaration => {
               if let Some((identifier, value)) = Self::variable_declaration(&node) {
                   self.scopes.add_identifier(identifier, value);
               }
               println!("{:?}", self.scopes);
           }
           _ => {}
       }

       node
   }

   fn execute_print(node: &AstNode) {
       let string_node = &node.children[0];
       let mut string_chars = string_node.value.chars();
       string_chars.next();
       string_chars.next_back();
       let string_literal = string_chars.as_str();

       match &string_node.node_type {
           AstNodeType::StringLiteral => {
               println!("{}", string_literal);
           }
           _ => {}
       }
   }

   fn variable_declaration(node: &AstNode) -> Option(String, String) {
       let mut current = 0;
       let mut identifier = None;
       let mut value = None;

       while current < node.children.len() {
           match node.children[current].node_type {
               AstNodeType::Identifier => identifier = Some(node.children[current].value.clone()),
               AstNodeType::StringLiteral => value = Some(node.children[current].value.clone()),
               _ => {}
           }
           current += 1;
       }

       if identifier.is_some() && value.is_some() {
           Some((identifier.unwrap(), value.unwrap()))
       } else {
           None
       }
   }
*/
