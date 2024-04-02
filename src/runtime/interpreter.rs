use crate::{
    core::{
        error::{self, ErrorType},
        handlers::print::print,
        runtypes::{traits::arithmetic::Arithmetic, RuntimeType},
    },
    syntax::{identifier, module::ModuleAst, AstNodeType, Expression},
};

use super::{scope, ScopesStack};

pub fn exec(ast: ModuleAst) {
    let mut scopes = ScopesStack::new();

    let mut counter = 0;
    while counter < ast.children.len() {
        exec_node(&ast.children[counter], &mut scopes);
        counter += 1;
    }
}

fn exec_node(node: &AstNodeType, scopes: &mut ScopesStack) {
    match node {
        AstNodeType::Block(node) => {
            let mut counter = 0;
            while counter < node.children.len() {
                exec_node(&node.children[counter], scopes);
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
/* use super::ScopesStack;
use crate::{
    core::{
        error::{self, ErrorType},
        handlers::print,
    },
    syntax::{AstNode, AstNodeType, AstTree, BinaryOperator, Bool, Expression},
};

pub struct Interpreter {
    scopes: ScopesStack,
    ast: AstTree,
}

impl Interpreter {
    pub fn new(scopes: ScopesStack, ast: AstTree) -> Interpreter {
        Interpreter { scopes, ast }
    }

    pub fn exec(&mut self) {
        Self::exec_block(&mut self.ast.root, &mut self.scopes);
    }

    fn exec_block(node: &mut AstNode, scopes: &mut ScopesStack) {
        for node in &mut node.children {
            match node.node_type {
                AstNodeType::Block => {
                    scopes.push();
                    Self::exec_block(node, scopes);
                    scopes.pop();
                }
                AstNodeType::IfStatement => {
                    if node.children.len() < 1 {
                        error::throw(ErrorType::SyntaxError, "'if' statement AST node was provided with no children. This occurs if an 'if' statement has no group and block nodes inside of it", None);
                    }

                    if Self::resolve_group(&node.children[0]) {
                        scopes.push();
                        Self::exec_block(&mut node.children[1], scopes);
                        scopes.pop();
                    }
                }
                AstNodeType::Group => {
                    // todo
                }
                AstNodeType::FunctionCall => {
                    if node.value.to_string() == "print" {
                        print(node.clone(), scopes);
                    }
                }
                AstNodeType::VariableDeclaration => {
                    let mut current = 0;
                    let mut identifier = None;
                    let mut value = None;

                    while current < node.children.len() {
                        match node.children[current].node_type {
                            AstNodeType::Expression(exp) => match exp {
                                Expression::Identifier => {
                                    identifier = Some(node.children[current].value.to_string())
                                }
                                Expression::StringLiteral => {
                                    value = Some(node.children[current].value.to_string())
                                }
                                Expression::NumberLiteral => {
                                    value = Some(node.children[current].value.to_string())
                                }
                                Expression::Binary(bin) => match bin {
                                    // TODO: catch cases well instead of catch all
                                    _ => value = Some(node.children[current].value.to_string()),
                                },
                                Expression::Boolean(bool) => match bool {
                                    Bool::True => {
                                        value = Some(node.children[current].value.to_string())
                                    }
                                    Bool::False => {
                                        value = Some(node.children[current].value.to_string())
                                    }
                                },
                            },
                            _ => {}
                        }
                        current += 1;
                    }

                    if identifier.is_some() && value.is_some() {
                        scopes.add_identifier(
                            identifier.unwrap().to_string(),
                            value.unwrap().to_string(),
                        );
                    } else {
                        let error_message = format!(
                            "Cannot declare varible '{}'",
                            identifier.unwrap_or("unknown".to_string())
                        );
                        error::throw(ErrorType::SyntaxError, &error_message, None);
                    }
                }
                _ => {}
            }
        }
    }

    // this is a use-case fitted solution, but i need
    // more conceptual development of the expressions
    // to know how to handle them well
    fn resolve_group(node: &AstNode) -> bool {
        true
    }
}
 */
