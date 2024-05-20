use crate::{
    ast::{
        assignament_statement::{AssignamentNode, VarType}, binary_expression::BinaryExpression, block::Block, call_expression::CallExpression, if_statement::IfStatement, module::ModuleAst, while_statement::WhileStatement, AstNodeType, Expression
    },
    core::{
        error::{self, ErrorType},
        handlers::{print_handler::print, type_handler::type_of},
        runtypes::{traits::arithmetic::Arithmetic, RuntimeType},
    },
    runtime::scope::ScopeInvoker,
};

use super::ScopesStack;

pub struct Interpreter {
    ast: ModuleAst,
    scopes: ScopesStack,
}

impl Interpreter {
    pub fn new(ast: ModuleAst) -> Interpreter {
        Interpreter {
            ast,
            scopes: ScopesStack::new(ScopeInvoker::Module),
        }
    }

    pub fn exec(&mut self, debug: bool) {
        // hoisting
        let mut counter = 0;
        while counter < self.ast.children.len() {
            hoist_node(&self.ast.children[counter], &mut self.scopes, ScopeInvoker::Module);
            counter += 1;
        }
        if debug {
            println!("\nScopes: \n---------------\n{:#?}", self.scopes);
        }
    
        // execution
        let mut counter = 0;
        while counter < self.ast.children.len() {
            exec_node(&self.ast.children[counter], &mut self.scopes, ScopeInvoker::Module); 
            counter += 1;
        }
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
            // hoist current function
            let identifier = node.identifier.name.clone();
            let rn_function = RuntimeType::function(
                identifier.clone(),
                node.parameters.clone(),
                node.body.clone(),
                node.at,
                node.line,
            );
            scopes.add_identifier(identifier, rn_function);

            // hoist inside function body
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

fn exec_node(
    node: &AstNodeType,
    scopes: &mut ScopesStack,
    invoker: ScopeInvoker,
) -> Option<RuntimeType> {
    match node {
        AstNodeType::Block(node) => exec_block(node, scopes, invoker), 
        AstNodeType::FunctionDeclaration(_node) => None, 
        AstNodeType::IfStatement(node) => exec_if(node, scopes, invoker), 
        AstNodeType::WhileStatement(node) => exec_while(node, scopes, invoker), 
        AstNodeType::AssignamentStatement(node) => exec_assignament(node, scopes, invoker), 
        AstNodeType::Expression(expr) => calc_expression(expr, scopes),
        _ => None,
    }
}

fn exec_block(
    node: &Block,
    scopes: &mut ScopesStack,
    invoker: ScopeInvoker,
) -> Option<RuntimeType> {
    let mut counter = 0;
    let mut return_expr = None;
    while counter < node.children.len() {
        let children = &node.children[counter];
        counter += 1;

        match invoker {
            ScopeInvoker::WhileStatement => match children {
                AstNodeType::BreakStatement(_) => {
                    return_expr = Some(RuntimeType::nothing());
                }
                AstNodeType::ReturnStatement(_) => {
                    error::throw(
                        ErrorType::SyntaxError,
                        "Return statements are not valid inside while loop",
                        Some(children.line()),
                    );
                    std::process::exit(1);
                }
                _ => {
                    let exec_return = exec_node(children, scopes, invoker);
                    if exec_return.is_some() {
                        return_expr = exec_return;
                        break;
                    }
                }
            },
            _ => {
                if let AstNodeType::ReturnStatement(ret) = children {
                    return_expr = match &ret.value {
                        Expression::Number(v) => Some(RuntimeType::number(v.value)),
                        Expression::StringLiteral(v) => {
                            Some(RuntimeType::string(v.value.clone(), false))
                        }
                        Expression::Bool(v) => Some(RuntimeType::boolean(v.value)),
                        Expression::Identifier(v) => Some(RuntimeType::identifier(v.name.clone())),
                        Expression::Nothing(_) => Some(RuntimeType::nothing()),
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
                    };
                    break;
                } else {
                    let exec_return = exec_node(children, scopes, invoker);
                    if exec_return.is_some() {
                        return_expr = exec_return;
                        break;
                    }
                }
            }
        }
    }

    if let Some(return_value) = return_expr {
        Some(return_value)
    } else {
        None
    }
}

fn exec_assignament(
    node: &AssignamentNode,
    scopes: &mut ScopesStack,
    _invoker: ScopeInvoker,
) -> Option<RuntimeType> {
    let value_as_runtype = calc_expression(&node.init, scopes).unwrap_or_else(|| {
        error::throw(
            ErrorType::InterpretingError,
            "This is a known possible issue. Please report on https://github.com/noreplydev/ego with your code",
            Some(node.line),
        );
        std::process::exit(1);
    });
    match node.var_type {
        VarType::None => {
            scopes.set_indentifier(node.identifier.name.clone(), value_as_runtype);
        }
        VarType::Const | VarType::Let => {
            scopes.add_identifier(node.identifier.name.clone(), value_as_runtype);
        }
    }
    None
}

fn exec_if(
    node: &IfStatement,
    scopes: &mut ScopesStack,
    _invoker: ScopeInvoker,
) -> Option<RuntimeType> {
    let condition = calc_expression(&node.condition, scopes).unwrap_or_else(|| {
        error::throw(
            ErrorType::InterpretingError,
            "This is a known possible issue. Please report on https://github.com/noreplydev/ego with your code",
            Some(node.line),
        );
        std::process::exit(1);
    });
    scopes.push(ScopeInvoker::IfStatement);
    let mut return_expr = None;

    if condition.to_boolean() {
        return_expr = exec_node(
            &AstNodeType::Block(node.body.clone()),
            scopes,
            ScopeInvoker::IfStatement,
        )
    } else if let Some(else_body) = &node.else_node {
        return_expr = exec_node(
            &AstNodeType::Block(else_body.body.clone()),
            scopes,
            ScopeInvoker::IfStatement,
        )
    }
    scopes.pop();
    return_expr
}

fn exec_while(
    node: &WhileStatement,
    scopes: &mut ScopesStack,
    _invoker: ScopeInvoker,
) -> Option<RuntimeType> {
    let mut return_expr = None;
    while calc_expression(&node.condition, scopes)
        .unwrap_or_else(|| {
            error::throw(
                ErrorType::InterpretingError,
                "This is a known possible issue. Please report on https://github.com/noreplydev/ego with your code",
                Some(node.line),
            );
            std::process::exit(1);
        })
        .to_boolean() 
    {
        scopes.push(ScopeInvoker::WhileStatement);
        return_expr = exec_node(
            &AstNodeType::Block(node.body.clone()),
            scopes,
            ScopeInvoker::WhileStatement,
        );
        if return_expr.is_some() {
            scopes.pop();
            break
        }
        scopes.pop();
    }
    return_expr

}

fn calc_expression(node: &Expression, scopes: &mut ScopesStack) -> Option<RuntimeType> {
    match node {
        Expression::Bool(v) => Some(RuntimeType::boolean(v.value)),
        Expression::Number(v) => Some(RuntimeType::number(v.value)),
        Expression::StringLiteral(v) => Some(RuntimeType::string(v.value.clone(), false)),
        Expression::Nothing(_) => Some(RuntimeType::nothing()),
        Expression::Identifier(v) => {
            if let Some(val) = scopes.get_identifier_value(&v.name) {
                Some(val.clone()) // now we are cloning the value, so
                                  // it's not like passing the reference
            } else {
                None
            }
        }
        Expression::BinaryExpression(expr) => {
            let left = calc_expression(&expr.left, scopes);
            let right = calc_expression(&expr.right, scopes);
            match left {
                Some(_left) => match right {
                    Some(_right) => {
                        let result = _left.arithmetic(expr.operator.as_str(), _right);
                        match result {
                            Ok(val) => Some(val),
                            Err(err) => {
                                error::throw(
                                    err,
                                    expr.operator.to_string().as_str(),
                                    Some(expr.line),
                                );
                                std::process::exit(1);
                            }
                        }
                    }
                    None => None,
                },
                None => None,
            }
        }
        Expression::CallExpression(node) => {
            let runtime_arguments: Vec<RuntimeType> = node
                .arguments
                .children
                .iter()
                .map(|arg| -> RuntimeType {
                    if let Some(arg) = arg {
                        calc_expression(&arg, scopes).unwrap_or_else(|| {
                            error::throw(
                                ErrorType::InterpretingError,
                                "This is a known possible issue. Please report on https://github.com/noreplydev/ego with your code",
                                Some(node.line),
                            );
                            std::process::exit(1);
                        })
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
                        type_of(runtime_arguments[0].clone())
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
