
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// Write to browser console
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn debug_log<T: std::fmt::Debug>(obj: &T, message: &str) {
    log(&format!("{0} :{1:?}", message, obj));
}