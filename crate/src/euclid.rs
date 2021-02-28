use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn lcm(a: i64, b: i64) -> i64 {
    match (a, b) {
        (_, 0) => 0,
        (_, _) => a.abs() / gcd(a, b) * b.abs()
    }
}

#[wasm_bindgen]
pub fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (_, 0) => a.abs(),
        (_, _) => gcd(b, a % b)
    }
}
