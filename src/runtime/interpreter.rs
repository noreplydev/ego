use crate::ast::{AstNode, AstTokenType, AstTree};

pub struct Interpreter {
    ast: AstTree,
}

impl Interpreter {
    pub fn new(ast: AstTree) -> Interpreter {
        Interpreter { ast }
    }

    pub fn execute(&self) {
        self.execute_node(&self.ast.root);
    }

    fn execute_node(&self, node: &AstNode) {
        match node.token_type {
            AstTokenType::Root => {
                for child in &node.children {
                    self.execute_node(child);
                }
            }
            AstTokenType::FunctionCall => {
                if node.value == "print" {
                    self.execute_print(node);
                }
            }
            AstTokenType::VariableDeclaration => {}
            _ => {}
        }
    }

    fn execute_print(&self, node: &AstNode) {
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
}
