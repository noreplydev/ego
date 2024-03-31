use crate::{
    core::runtypes::{traits::print::Print, RuntimeType},
    runtime::ScopesStack,
};

pub fn print(args: Vec<RuntimeType>, scopes: &ScopesStack) {
    let mut raw_values: Vec<String> = vec![];
    for arg in args {
        raw_values.push(arg.print(scopes));
    }

    let string = raw_values.join(" ");
    println!("{string}");
}

/* use crate::{
    runtime::ScopesStack,
    syntax::{AstNode, AstNodeType, Bool, Expression},
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
                        Expression::Binary(bin) => match bin {
                            _ => {
                                values.push(child.value.to_string());
                            }
                        },
                        Expression::Boolean(bool) => match bool {
                            Bool::True => {
                                values.push(child.value.to_string());
                            }
                            Bool::False => {
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
 */
