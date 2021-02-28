use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn factorial(n: u64) -> u64 {
    fn go(n: u64, acc: u64) -> u64 {
        match n {
            0 => acc,
            _ => go(n - 1, n * acc),
        }
    }

    go(n, 1)
}
