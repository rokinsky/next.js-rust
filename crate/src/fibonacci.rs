use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci(n: u64) -> u64 {
    fn go(n: u64, a: u64, b: u64) -> u64 {
        match n {
            0 => a,
            1 => b,
            _ => go(n - 1, b, a + b)
        }
    }

    go(n, 0, 1)
}
