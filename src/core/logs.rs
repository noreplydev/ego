use wasm_bindgen::prelude::*;
use web_sys::console;

pub fn log(str: &str) {
    if cfg!(target_arch = "wasm32") {
        console::log_1(&JsValue::from_str(str));
    } else {
        println!("{str}");
    }
}

// Macro to simplify logging usage
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        $crate::core::logs::log(&format!($($arg)*));
    };
}
