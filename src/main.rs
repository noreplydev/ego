mod lexer;
mod parser;

use lexer::lex;
use std::env;
use std::fs;

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
    //let ast = parse(tokens);
    for (i, token) in tokens.iter().enumerate() {
        println!("{i}. {token}");
    }
}
