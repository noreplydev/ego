use std::collections::HashMap;

use crate::core::{
    error::{self, ErrorType},
    runtypes::RuntimeType,
};

#[derive(Debug, Clone)]
pub struct ScopesStack {
    scopes: Vec<Scope>,
}

impl ScopesStack {
    pub fn new(invoker: ScopeInvoker) -> ScopesStack {
        ScopesStack {
            scopes: vec![Scope::new(invoker)],
        }
    }

    pub fn add_identifier(&mut self, identifier: String, value: RuntimeType) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.add(identifier, value);
        }
    }

    // this function must to be recursive since the target variable can be defined in another scope
    pub fn set_indentifier(&mut self, identifier: String, value: RuntimeType) {
        let mut counter = self.scopes.len() - 1;

        while counter >= 0 {
            if let Some(_) = self.scopes[counter].get(&identifier) {
                self.scopes[counter].set(identifier.clone(), value);
                break;
            } else if counter == 0 {
                error::throw(
                    ErrorType::ReferenceError,
                    format!("identifier '{identifier}' was not declared").as_str(),
                    None,
                );
            }
            counter -= 1;
        }
    }

    pub fn get_identifier_value(&self, identifier: &String) -> Option<&RuntimeType> {
        let mut counter = self.scopes.len() - 1;

        while counter >= 0 {
            if let Some(identifier_value) = self.scopes[counter].get(identifier) {
                return Some(identifier_value);
            } else if counter == 0 {
                error::throw(
                    ErrorType::ReferenceError,
                    format!("identifier '{identifier}' was not declared").as_str(),
                    None,
                );
            }
            counter -= 1;
        }

        error::throw(
            ErrorType::ReferenceError,
            format!("identifier '{identifier}' was not declared").as_str(),
            None,
        );
        std::process::exit(1);
    }

    pub fn push(&mut self, invoker: ScopeInvoker) {
        self.scopes.push(Scope::new(invoker));
    }

    pub fn pop(&mut self) {
        let pop_status = self.scopes.pop();
        if pop_status.is_none() {
            error::throw(
                ErrorType::StackUnderflowError,
                "Error: Stack Underflow Detected\nThe program attempted to exit a scope when none are active. This usually indicates a mismatch in the creation and destruction of scopes, such as exiting more blocks or functions than were entered. Please review your code for any discrepancies in scope management, ensuring that each entered scope or function block is properly exited.", 
                None
            );
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ScopeInvoker {
    Module,
    IfStatement,
    WhileStatement,
    Function,
}
#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, RuntimeType>,
    pub invoker: ScopeInvoker,
}

impl Scope {
    fn new(invoker: ScopeInvoker) -> Scope {
        Scope {
            vars: HashMap::new(),
            invoker,
        }
    }

    fn add(&mut self, identifier: String, value: RuntimeType) -> bool {
        if self.vars.contains_key(&identifier) {
            error::throw(
                ErrorType::ReferenceError,
                format!("Cannot redeclare '{identifier}' in the scope").as_str(),
                None,
            );
        }

        match self.vars.insert(identifier.clone(), value) {
            Some(_) => {
                error::throw(
                    ErrorType::ReferenceError,
                    format!("Cannot redeclare '{identifier}' in the scope").as_str(),
                    None,
                );
                std::process::exit(1)
            }
            _ => true,
        }
    }

    fn set(&mut self, identifier: String, value: RuntimeType) -> bool {
        match self.vars.insert(identifier, value) {
            Some(_) => true,
            None => false,
        }
    }

    fn get(&self, identifier: &String) -> Option<&RuntimeType> {
        self.vars.get(identifier)
    }
}
