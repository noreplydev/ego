use crate::ast::AstNode;

pub fn print(node: AstNode) {
    let string_node = &node.children[0];
    let mut string_chars = string_node.value.chars();

    // remove first and last quote
    string_chars.next();
    string_chars.next_back();
    let string_literal = string_chars.as_str();

    println!("{}", string_literal)
}
