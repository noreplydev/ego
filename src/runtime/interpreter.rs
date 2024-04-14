use crate::{
    ast::{identifier, module::ModuleAst, AstNodeType, Expression},
    core::{
        error::{self, ErrorType},
        handlers::print::print,
        runtypes::{traits::arithmetic::Arithmetic, RuntimeType},
    },
};

use super::{scope, ScopesStack};

pub fn exec(ast: ModuleAst) {
    let mut scopes = ScopesStack::new();

    // hoisting
    let mut counter = 0;
    while counter < ast.children.len() {
        hoist_node(&ast.children[counter], &mut scopes);
        counter += 1;
    }

    // execution
    let mut counter = 0;
    while counter < ast.children.len() {
        exec_node(&ast.children[counter], &mut scopes);
        counter += 1;
    }
}

fn hoist_node(node: &AstNodeType, scopes: &mut ScopesStack) {
    match node {
        AstNodeType::Block(node) => {
            let mut counter = 0;
            while counter < node.children.len() {
                hoist_node(&node.children[counter], scopes);
                counter += 1;
            }
        }
        AstNodeType::FunctionDeclaration(node) => {
            let identifier = node.identifier.name.clone();
            let rn_function = RuntimeType::function(
                identifier.clone(),
                node.parameters.clone(),
                node.body.clone(),
                node.at,
                node.line,
            );
            scopes.add_identifier(identifier, rn_function);
        }
        _ => {}
    }
}

fn exec_node(node: &AstNodeType, scopes: &mut ScopesStack) {
    match node {
        AstNodeType::Block(node) => {
            scopes.push();
            let mut counter = 0;
            while counter < node.children.len() {
                exec_node(&node.children[counter], scopes);
                counter += 1;
            }
            scopes.pop();
        }
        AstNodeType::FunctionDeclaration(node) => {
            // hoisted before execution
        }
        AstNodeType::CallExpression(node) => {
            let runtime_arguments: Vec<RuntimeType> = node
                .arguments
                .children
                .iter()
                .map(|arg| -> RuntimeType {
                    if let Some(arg) = arg {
                        calc_expression(&arg, scopes)
                    } else {
                        RuntimeType::nothing()
                    }
                })
                .collect();

            // push new hashmap onto the stack
            // for function scope
            scopes.push();
            match node.identifier.name.as_str() {
                "print" => print(runtime_arguments, &scopes),
                _ => {
                    // runtime declared functions
                    let function = match scopes.get_identifier_value(&node.identifier.name) {
                        Some(func) => func,
                        None => {
                            error::throw(
                                ErrorType::ReferenceError,
                                format!("Function '{}' has not been defined", node.identifier.name)
                                    .as_str(),
                                Some(node.line),
                            );
                            std::process::exit(1);
                        }
                    };

                    let function = match function {
                        RuntimeType::RnFunction(func) => func,
                        _ => unreachable!(),
                    };

                    exec_node(&AstNodeType::Block(function.body.clone()), scopes);
                }
            }
            scopes.pop();
        }
        AstNodeType::AssignamentStatement(node) => {
            let value_as_runtype = calc_expression(&node.init, scopes);
            scopes.add_identifier(node.identifier.name.clone(), value_as_runtype);
        }
        _ => {}
    }
}

fn calc_expression(node: &Expression, scopes: &mut ScopesStack) -> RuntimeType {
    match node {
        Expression::Bool(b) => RuntimeType::boolean(b.value),
        Expression::Number(n) => RuntimeType::number(n.value),
        Expression::StringLiteral(s) => RuntimeType::string(s.value.clone()),
        Expression::BinaryExpression(expr) => {
            let left = calc_expression(&expr.left, scopes);
            let right = calc_expression(&expr.right, scopes);
            let result = left.arithmetic(expr.operator, right, scopes);

            match result {
                Ok(val) => val,
                Err(err) => {
                    error::throw(err, expr.operator.to_string().as_str(), Some(expr.line));
                    std::process::exit(1);
                }
            }
        }
        Expression::Identifier(i) => {
            if let Some(val) = scopes.get_identifier_value(&i.name) {
                val.clone() // now we are cloning the value, so
                            // it's not like passing the reference
            } else {
                RuntimeType::nothing()
            }
        }
    }
}
