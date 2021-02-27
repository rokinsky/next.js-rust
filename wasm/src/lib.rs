mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
    accounts.iter()
        .map(|x| x.iter().sum())
        .max()
        .unwrap()
}

#[wasm_bindgen]
pub fn calculate_maximum_wealth(matrix_size: i32) -> i32 {
    let accounts = (0..matrix_size).map(|_| (0..matrix_size).collect()).collect();
    maximum_wealth(accounts)
}
