mod ast;
mod interpreter;
mod lexer;
mod parser;

use lexer::lex;
use parser::parse;
use std::env;
use std::fs;

use crate::interpreter::Interpreter;

pub const KEYWORDS: [&str; 2] = ["print", "let"]; // "if", "else", "while", "for", "fn"

fn main() {
    let args: Vec<String> = env::args().collect();

    let filename = if args.len() > 1 {
        &args[1]
    } else {
        panic!("[goru] A file is required");
    };

    if !filename.ends_with(".e") {
        panic!("[goru] This is not .e (ego) file");
    }

    let file_content = fs::read_to_string(filename).expect("[goru] This is not .e (ego) file");

    let tokens = lex(file_content);
    let ast = parse(tokens.clone());

    if args.len() > 2 && args[2] == "-d" {
        println!("\nLexer tokens: \n-------------");
        for (i, token) in tokens.iter().enumerate() {
            println!("{i}. {token}");
        }
        println!("\nAST:\n----\n{:?}\n", ast);
    }

    let interpreter = Interpreter::new(ast);
    interpreter.execute();

    println!("")
}
