mod ast;
mod lexer;
mod parser;
mod runtime;

use lexer::lex;
use parser::parse;
use runtime::Interpreter;
use std::env;
use std::fs;

pub const KEYWORDS: [&str; 2] = ["print", "let"]; // "if", "else", "while", "for", "fn"

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        panic!("[cei] A file is required");
    };

    if !filename.ends_with(".e") {
        panic!("[cei] This is not .e (ego) file");
    }

    let file_content = fs::read_to_string(filename).expect("[cei] This is not .e (ego) file");

    let tokens = lex(file_content);
    if args.len() > 2 && args[2] == "-d" {
        println!("\nLexer tokens: \n-------------");
        for (i, token) in tokens.iter().enumerate() {
            println!("{i}. {token}");
        }
    }

    let ast = parse(tokens.clone());
    if args.len() > 2 && args[2] == "-d" {
        println!("\nAST:\n----\n{:?}\n", ast);
    }

    let interpreter = Interpreter::new(ast);
    interpreter.execute();

    println!("")
}
