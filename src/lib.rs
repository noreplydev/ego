mod ast;
mod commands;
mod compiler;
mod core;
mod runtime;

use commands::run::run_ego;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exec_ego_code(code: String, vm: bool) {
    run_ego(code, vm);
}
