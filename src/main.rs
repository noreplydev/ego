mod core;
mod runtime;
mod syntax;

use core::error;
use core::error::ErrorType;
//use runtime::Interpreter;
use runtime::exec;
use std::env;
use std::fs;
use syntax::lex;
use syntax::Module;

pub const KEYWORDS: [&str; 8] = [
    "fn", "let", "if", "while", "true", "false", "print", "import",
];

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        error::throw(ErrorType::EgoUsageError, "an ego file is required", None);
        std::process::exit(1); // to avoid types error
    };
    let debug = args.len() > 2 && args[2] == "-d";

    if !filename.ends_with(".ego") {
        error::throw(ErrorType::EgoUsageError, "This is not .e (ego) file", None);
    }

    let module_name = filename.split(".").collect::<Vec<&str>>()[0];
    let file_content = fs::read_to_string(filename).unwrap_or_else(|_| {
        error::throw(
            ErrorType::FatalError,
            "Something went wrong reading file",
            None,
        );
        std::process::exit(1); // to avoid types error
    });

    let tokens = lex(file_content);
    if debug {
        println!("\nLexer tokens: \n-------------");
        for (i, token) in tokens.iter().enumerate() {
            println!("{i}. {token}");
        }
    }

    let module = Module::new(module_name.to_string(), tokens);
    let ast = module.parse();
    if debug {
        println!("\nAst nodes: \n---------------\n{:#?}", ast);
    }

    exec(ast);
}
