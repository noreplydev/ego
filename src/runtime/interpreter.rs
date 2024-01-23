use super::ScopesStack;
use crate::{
    ast::{AstTokenType, AstTree},
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
            match node.token_type {
                AstTokenType::FunctionCall => {
                    if node.value == "print" {
                        print(node.clone())
                    }
                }
                AstTokenType::VariableDeclaration => {
                    let mut current = 0;
                    let mut identifier = None;
                    let mut value = None;

                    while current < node.children.len() {
                        match node.children[current].token_type {
                            AstTokenType::Identifier => {
                                identifier = Some(node.children[current].value.clone())
                            }
                            AstTokenType::StringLiteral => {
                                value = Some(node.children[current].value.clone())
                            }
                            _ => {}
                        }
                        current += 1;
                    }

                    if identifier.is_some() && value.is_some() {
                        self.scopes.add_identifier(
                            identifier.unwrap().to_string(),
                            value.unwrap().to_string(),
                        );

                        println!("{:?}", self.scopes);
                    } else {
                        println!("[cei] Variable identifier or value is missing");
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
       match node.token_type {
           AstTokenType::Root => {
               for child in node.children {
                   self.execute_node(child);
               }
           }
           AstTokenType::FunctionCall => {
               if node.value == "print" {
                   Self::execute_print(&node);
               }
           }
           AstTokenType::VariableDeclaration => {
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

       match &string_node.token_type {
           AstTokenType::StringLiteral => {
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
           match node.children[current].token_type {
               AstTokenType::Identifier => identifier = Some(node.children[current].value.clone()),
               AstTokenType::StringLiteral => value = Some(node.children[current].value.clone()),
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
