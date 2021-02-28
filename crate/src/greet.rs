use wasm_bindgen::prelude::*;
use crate::utils::set_panic_hook;
use crate::alert;

#[wasm_bindgen]
pub fn greet(name: &str) {
    set_panic_hook();
    alert(&format!("Hello, {}!", name));
}
