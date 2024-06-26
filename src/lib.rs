mod ast;
mod commands;
mod compiler;
mod core;
mod runtime;
mod wasm;

use wasm::run_ego;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn exec_ego_code(code: String, vm: bool) {
    run_ego(code, vm);
}
