use crate::{
    ast::{AstNode, AstTokenType},
    runtime::ScopesStack,
};

pub fn print(node: AstNode, scopes: &ScopesStack) {
    let mut values: Vec<&String> = vec![];

    for child in node.children {
        if child.token_type == AstTokenType::Identifier {
            if let Some(value) = scopes.get_identifier_value(&child.value) {
                values.push(value);
            } else {
                println!("[cei] '{}' is not defined", child.value);
                std::process::exit(1);
            }
        }
    }

    println!("{:?}", values);
    /*     let string_node = &node.children[0];
    let mut string_chars = string_node.value.chars();

    // remove first and last quote
    string_chars.next();
    string_chars.next_back();
    let string_literal = string_chars.as_str();

    println!("{}", string_literal) */
}
