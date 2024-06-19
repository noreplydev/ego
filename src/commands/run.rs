use std::fs;

use crate::ast::lex;
use crate::ast::Module;
use crate::compiler::Compiler;
use crate::core::error;
use crate::core::error::ErrorType;
use crate::runtime::Interpreter;

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
        let module_name = if self.args.len() > 0 {
            self.args[0].clone()
        } else {
            "main.ego".to_string() // default lookup on a ego project
        };

        let file_content = fs::read_to_string(&module_name).unwrap_or_else(|_| {
            error::throw(
                ErrorType::FatalError,
                format!("Cannot read {}\n", module_name).as_str(),
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

        if self.args.contains(&"-vm".to_string()) {
            let bytecode = Compiler::gen_bytecode(ast);
            let mut vm = self_vm::vm::Vm::new(bytecode);
            vm.run();
        } else {
            let mut interpreter = Interpreter::new(ast.clone());
            interpreter.exec(self.debug());
        }
    }
}
