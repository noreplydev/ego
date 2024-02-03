use crate::{
    ast::{AstNode, AstNodeType, Boolean, Expression},
    runtime::ScopesStack,
};

pub fn print(node: AstNode, scopes: &ScopesStack) {
    let mut values: Vec<String> = vec![];

    for (i, child) in node.children.iter().enumerate() {
        if child.node_type == AstNodeType::Group && i == 0 {
            for child in &child.children {
                match child.node_type {
                    AstNodeType::Expression(exp) => match exp {
                        Expression::Identifier => {
                            if let Some(value) =
                                scopes.get_identifier_value(&child.value.to_string())
                            {
                                values.push(value.clone());
                            }
                        }
                        Expression::StringLiteral => {
                            values.push(child.value.to_string());
                        }
                        Expression::NumberLiteral => {
                            values.push(child.value.to_string());
                        }
                        Expression::BinaryOperator => {
                            values.push(child.value.to_string());
                        }
                        Expression::Boolean(bool) => match bool {
                            Boolean::True => {
                                values.push(child.value.to_string());
                            }
                            Boolean::False => {
                                values.push(child.value.to_string());
                            }
                        },
                    },
                    _ => {}
                }
            }
        }
    }

    println!("{}", values.join(" "));
}
