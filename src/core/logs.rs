use std::sync::Mutex;

use lazy_static::lazy_static;
use wasm_bindgen::prelude::*;
use web_sys::console;

lazy_static! {
    static ref LOG_HISTORY: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub fn log(str: &str) {
    if cfg!(target_arch = "wasm32") {
        console::log_1(&JsValue::from_str(str));
    } else {
        println!("{str}");
    }

    LOG_HISTORY.lock().unwrap().push(str.to_string());
}

pub fn get_log_history() -> Vec<String> {
    LOG_HISTORY.lock().unwrap().clone()
}

// Macro to simplify logging usage
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::core::logs::log(&format!($($arg)*));
    };
}
