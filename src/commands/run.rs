use std::fs;

use crate::ast::lex;
use crate::ast::Module;
use crate::core::error;
use crate::core::error::ErrorType;
use crate::runtime::{exec, Interpreter};

pub struct Run {
    args: Vec<String>,
}

impl Run {
    pub fn new(args: Vec<String>) -> Run {
        Run { args }
    }
    pub fn debug(&self) -> bool {
        self.args.contains(&"-d".to_string())
    }
    pub fn exec(&self) {
        if self.args.len() < 1 {}

        let module_name = self.args[0].clone();
        let file_content = fs::read_to_string(&module_name).unwrap_or_else(|_| {
            error::throw(
                ErrorType::FatalError,
                format!("Cannot read {}", self.args[0]).as_str(),
                None,
            );
            std::process::exit(1); // to avoid types error
        });

        let tokens = lex(file_content);
        if self.debug() {
            println!("\nLexer tokens: \n-------------");
            for (i, token) in tokens.iter().enumerate() {
                println!("{i}. {token}");
            }
        }

        let mut module = Module::new(module_name, tokens);
        let ast = module.parse();
        if self.debug() {
            println!("\nAst nodes: \n---------------\n{:#?}", ast);
        }

        let interpreter = Interpreter::new(ast.clone());
        exec(ast);
    }
}
