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
    pub fn new() -> ScopesStack {
        ScopesStack {
            scopes: vec![Scope::new()],
        }
    }

    pub fn add_identifier(&mut self, identifier: String, value: RuntimeType) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.add(identifier, value);
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

    pub fn push(&mut self) {
        self.scopes.push(Scope::new());
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

    // this function must to be recursive since the target variable can be defined in another scope
    /*     pub fn set_indentifier(&self, indentifier: String, value: String) -> bool {
        let current_scope = match self.scopes.last() {
            Some(last) => last,
            None => {
                println!("[cei] ScopesStack it's empty");
                std::process::exit(1);
            }
        };
        let status = current_scope.set(indentifier, value);

        if status {
            println!("inserted");
        } else {
            println!("not inserted");
        }

        status
        /* println!("[cei] '{identifier}' is not declared");
        process::exit(1); */
    } */
    /*
    pub fn remove_identifier(&self) -> bool {} */
}

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, RuntimeType>,
}

impl Scope {
    fn new() -> Scope {
        Scope {
            vars: HashMap::new(),
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

    /*     fn get(&self, name: &str) -> Option<String> {
        // here get local value
    }
    fn set(&mut self, name: String, value: String) -> bool {
        true
    } */
}
