use crate::{
    ast::{AstNode, AstNodeType, Expression},
    runtime::ScopesStack,
};

pub fn print(node: AstNode, scopes: &ScopesStack) {
    let mut values: Vec<String> = vec![];

    for child in node.children {
        match child.node_type {
            AstNodeType::Expression(exp) => match exp {
                Expression::Identifier => {
                    if let Some(value) = scopes.get_identifier_value(&child.value.to_string()) {
                        values.push(value.clone());
                    }
                }
                Expression::StringLiteral => {
                    values.push(child.value.to_string());
                }
                Expression::NumberLiteral => {
                    values.push(child.value.to_string());
                }
            },
            _ => {}
        }
    }

    println!("{}", values.join(" "));
}
