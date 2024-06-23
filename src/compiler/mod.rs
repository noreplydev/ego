mod bytecode;
mod handlers;

use crate::ast::{module::ModuleAst, AstNodeType, Expression};

pub struct Compiler {}

impl Compiler {
    pub fn gen_bytecode(ast: ModuleAst) -> Vec<u8> {
        let mut counter = 0;
        let mut bytecode: Vec<u8> = vec![];

        while counter < ast.children.len() {
            match &ast.children[counter] {
                AstNodeType::Expression(expr) => match expr {
                    Expression::CallExpression(v) => {
                        let call_expression_bytecode = match v.identifier.name.as_str() {
                            "print" => handlers::print_as_bytecode(v),
                            _ => {
                                // todo: handle custom defined callable members
                                vec![]
                            }
                        };

                        bytecode.extend_from_slice(&call_expression_bytecode);
                    }
                    _ => {}
                },
                _ => {}
            }
            counter += 1;
        }

        bytecode
    }
}
