use crate::{
    ast::{AstNode, AstTokenType},
    runtime::ScopesStack,
};

pub fn print(node: AstNode, scopes: &ScopesStack) {
    let mut values: Vec<String> = vec![];

    for child in node.children {
        match child.token_type {
            AstTokenType::Identifier => {
                if let Some(value) = scopes.get_identifier_value(&child.value) {
                    values.push(value.clone());
                }
            }
            AstTokenType::StringLiteral => {
                let a = child.value.clone();
                values.push(a);
            }
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
