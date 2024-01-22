use std::{collections::HashMap, process};

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

    fn __new(scopes: Vec<Scope>) -> ScopesStack {
        ScopesStack { scopes }
    }

    pub fn add(&mut self) -> ScopesStack {
        let new_scope = Scope::new();
        let mut scopes = self.scopes.clone();
        scopes.push(new_scope);

        ScopesStack::__new(scopes)
    }

    pub fn add_identifier(&mut self, identifier: String, value: String) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.add(identifier, value);
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
    pub fn remove(&self) -> bool {}
    pub fn remove_identifier(&self) -> bool {} */
}

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, String>,
}

impl Scope {
    fn new() -> Scope {
        Scope {
            vars: HashMap::new(),
        }
    }

    fn add(&mut self, identifier: String, value: String) -> bool {
        if self.vars.contains_key(&identifier) {
            println!("[cei] Cannot redeclare '{identifier}' in the scope");
            process::exit(1);
        }

        let mut vars = self.vars.clone();
        match vars.insert(identifier.clone(), value) {
            Some(_) => {
                println!("[cei] Cannot redeclare '{identifier}' in the scope");
                process::exit(1);
            }
            None => {
                self.vars = vars;
                true
            }
        }
    }

    fn set(&mut self, identifier: String, value: String) -> bool {
        match self.vars.insert(identifier, value) {
            Some(_) => true,
            None => false,
        }
    }

    /*     fn get(&self, name: &str) -> Option<String> {
        // here get local value
    }
    fn set(&mut self, name: String, value: String) -> bool {
        true
    } */
}
