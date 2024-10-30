mod ast;
mod commands;
mod compiler;
mod core;
mod runtime;
mod wasm;

use ast::{lex, Module};
use compiler::Compiler;
use wasm::run_ego;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exec_ego_code(code: String, vm: bool) -> Vec<String> {
    run_ego(code, vm)
}

pub fn gen_bytecode(code: String) -> Vec<u8> {
    let tokens = lex(code);
    let mut module = Module::new("unknown".to_string(), tokens);
    let ast = module.parse();
    Compiler::gen_bytecode(ast)
}
