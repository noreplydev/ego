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
                    if let Some(value) = scopes.get_identifier_value(&child.value) {
                        values.push(value.clone());
                    }
                }
                Expression::StringLiteral => {
                    values.push(child.value.clone());
                }
                Expression::NumberLiteral => {
                    values.push(child.value.clone());
                }
            },
            _ => {}
        }
    }

    let formatted_values: Vec<String> = values
        .iter()
        .map(|val| {
            let mut val = val.chars();
            val.next();
            val.next_back();
            val.as_str().to_string()
        })
        .collect();

    println!("{}", formatted_values.join(""));
}
