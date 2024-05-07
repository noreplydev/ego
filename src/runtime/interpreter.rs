use crate::{
    ast::{
        assignament_statement::VarType, binary_expression::BinaryExpression,
        call_expression::CallExpression, module::ModuleAst, AstNodeType, Expression,
    },
    core::{
        error::{self, ErrorType},
        handlers::{print_handler::print, type_handler::type_of},
        runtypes::{traits::arithmetic::Arithmetic, RuntimeType},
    },
    runtime::scope::ScopeInvoker,
};

use super::ScopesStack;

pub fn exec(ast: ModuleAst) {
    let mut scopes = ScopesStack::new(ScopeInvoker::Module);

    // hoisting
    let mut counter = 0;
    while counter < ast.children.len() {
        hoist_node(&ast.children[counter], &mut scopes, ScopeInvoker::Module);
        counter += 1;
    }

    // execution
    let mut counter = 0;
    while counter < ast.children.len() {
        exec_node(&ast.children[counter], &mut scopes, ScopeInvoker::Module);
        counter += 1;
    }
}

fn hoist_node(node: &AstNodeType, scopes: &mut ScopesStack, invoker: ScopeInvoker) {
    match node {
        AstNodeType::Block(node) => {
            // block level hoisting
            let mut counter = 0;
            while counter < node.children.len() {
                hoist_node(&node.children[counter], scopes, invoker);
                counter += 1;
            }
        }
        AstNodeType::IfStatement(node) => {
            // block level hoisting
            scopes.push(ScopeInvoker::IfStatement);
            hoist_node(
                &AstNodeType::Block(node.body.clone()),
                scopes,
                ScopeInvoker::IfStatement,
            );
            scopes.pop();
        }
        AstNodeType::WhileStatement(node) => {
            // block level hoisting
            scopes.push(ScopeInvoker::WhileStatement);
            hoist_node(
                &AstNodeType::Block(node.body.clone()),
                scopes,
                ScopeInvoker::WhileStatement,
            );
            scopes.pop();
        }
        AstNodeType::FunctionDeclaration(node) => {
            // add declaration to scopes
            let identifier = node.identifier.name.clone();
            let rn_function = RuntimeType::function(
                identifier.clone(),
                node.parameters.clone(),
                node.body.clone(),
                node.at,
                node.line,
            );
            scopes.add_identifier(identifier, rn_function);

            // hoisting function level declarations
            scopes.push(ScopeInvoker::Function);
            hoist_node(
                &AstNodeType::Block(node.body.clone()),
                scopes,
                ScopeInvoker::Function,
            );
            scopes.pop();
        }
        _ => {}
    }
}

fn exec_node(node: &AstNodeType, scopes: &mut ScopesStack, invoker: ScopeInvoker) -> RuntimeType {
    match node {
        AstNodeType::Block(node) => {
            let mut counter = 0;
            let mut return_expr = None;
            while counter < node.children.len() {
                let children = &node.children[counter];
                counter += 1;

                // executed only inside blocks of code
                if let AstNodeType::ReturnStatement(ret) = children {
                    return_expr = Some(match &ret.value {
                        Expression::Number(v) => RuntimeType::number(v.value),
                        Expression::StringLiteral(v) => RuntimeType::string(v.value.clone(), false),
                        Expression::Bool(v) => RuntimeType::boolean(v.value),
                        Expression::Identifier(v) => RuntimeType::identifier(v.name.clone()),
                        Expression::Nothing(_) => RuntimeType::nothing(),
                        Expression::BinaryExpression(v) => calc_expression(
                            &Expression::BinaryExpression(BinaryExpression::new(
                                v.operator.clone(),
                                v.left.clone(),
                                v.right.clone(),
                                v.at,
                                v.line,
                            )),
                            scopes,
                        ),
                        Expression::CallExpression(v) => calc_expression(
                            &Expression::CallExpression(CallExpression::new(
                                v.identifier.clone(),
                                v.arguments.clone(),
                                v.at,
                                v.line,
                            )),
                            scopes,
                        ),
                    })
                } else {
                    exec_node(children, scopes, invoker);
                }
            }

            scopes.pop();
            if let Some(return_value) = return_expr {
                return_value
            } else {
                RuntimeType::nothing()
            }
        }
        AstNodeType::FunctionDeclaration(_node) => {
            // hoisted before execution
            RuntimeType::nothing()
        }
        AstNodeType::AssignamentStatement(node) => {
            let value_as_runtype = calc_expression(&node.init, scopes);
            match node.var_type {
                VarType::None => {
                    scopes.set_indentifier(node.identifier.name.clone(), value_as_runtype);
                }
                VarType::Const | VarType::Let => {
                    scopes.add_identifier(node.identifier.name.clone(), value_as_runtype);
                }
            }
            RuntimeType::nothing()
        }
        AstNodeType::IfStatement(node) => {
            let condition = calc_expression(&node.condition, scopes);
            scopes.push(ScopeInvoker::IfStatement);
            if condition.to_boolean() {
                exec_node(
                    &AstNodeType::Block(node.body.clone()),
                    scopes,
                    ScopeInvoker::IfStatement,
                );
            } else if let Some(else_body) = &node.else_node {
                exec_node(
                    &AstNodeType::Block(else_body.body.clone()),
                    scopes,
                    ScopeInvoker::IfStatement,
                );
            }
            scopes.pop();
            RuntimeType::nothing()
        }
        AstNodeType::WhileStatement(node) => {
            while calc_expression(&node.condition, scopes).to_boolean() {
                scopes.push(ScopeInvoker::WhileStatement);
                exec_node(
                    &AstNodeType::Block(node.body.clone()),
                    scopes,
                    ScopeInvoker::WhileStatement,
                );
                scopes.pop();
            }
            RuntimeType::nothing()
        }
        AstNodeType::Expression(expr) => calc_expression(expr, scopes),
        _ => RuntimeType::nothing(),
    }
}

fn calc_expression(node: &Expression, scopes: &mut ScopesStack) -> RuntimeType {
    match node {
        Expression::Bool(v) => RuntimeType::boolean(v.value),
        Expression::Number(v) => RuntimeType::number(v.value),
        Expression::StringLiteral(v) => RuntimeType::string(v.value.clone(), false),
        Expression::Nothing(_) => RuntimeType::nothing(),
        Expression::Identifier(v) => {
            if let Some(val) = scopes.get_identifier_value(&v.name) {
                val.clone() // now we are cloning the value, so
                            // it's not like passing the reference
            } else {
                RuntimeType::nothing()
            }
        }
        Expression::BinaryExpression(expr) => {
            let left = calc_expression(&expr.left, scopes);
            let right = calc_expression(&expr.right, scopes);
            let result = left.arithmetic(expr.operator.as_str(), right, scopes);

            match result {
                Ok(val) => val,
                Err(err) => {
                    error::throw(err, expr.operator.to_string().as_str(), Some(expr.line));
                    std::process::exit(1);
                }
            }
        }
        Expression::CallExpression(node) => {
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
            scopes.push(ScopeInvoker::Function);
            let call_expression_return = match node.identifier.name.as_str() {
                "print" => print(runtime_arguments, scopes),
                "type" => {
                    if runtime_arguments.len() > 0 {
                        type_of(runtime_arguments[0].clone(), scopes)
                    } else {
                        error::throw(
                            ErrorType::SyntaxError,
                            "type(...) requires one parameter of <any> type in it's call",
                            Some(node.line),
                        );
                        std::process::exit(1);
                    }
                }
                _ => {
                    let function = scopes
                        .get_identifier_value(&node.identifier.name)
                        .unwrap_or_else(|| {
                            error::throw(
                                ErrorType::ReferenceError,
                                format!("Function '{}' has not been defined", node.identifier.name)
                                    .as_str(),
                                Some(node.line),
                            );
                            std::process::exit(1);
                        });

                    let (parameters, body) = match function {
                        RuntimeType::RnFunction(func) => {
                            (func.parameters.clone(), func.body.clone())
                        }
                        _ => {
                            error::throw(
                                ErrorType::ReferenceError,
                                format!("Identifier '{}' is not callable", node.identifier.name)
                                    .as_str(),
                                Some(node.line),
                            );
                            std::process::exit(1);
                        }
                    };

                    for (i, parameter) in parameters.iter().enumerate() {
                        if i < runtime_arguments.len() {
                            scopes.add_identifier(
                                parameter.name.clone(),
                                runtime_arguments[i].clone(),
                            );
                        } else {
                            scopes.add_identifier(parameter.name.clone(), RuntimeType::nothing());
                        }
                    }

                    exec_node(&AstNodeType::Block(body), scopes, ScopeInvoker::Function)
                }
            };
            scopes.pop();
            call_expression_return
        }
    }
}
