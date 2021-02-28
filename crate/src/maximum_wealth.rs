use wasm_bindgen::prelude::*;

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
