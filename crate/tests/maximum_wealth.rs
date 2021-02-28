use wasm::maximum_wealth::{maximum_wealth, calculate_maximum_wealth};

#[test]
fn test_maximum_wealth() {
    assert_eq!(maximum_wealth(vec![vec![1, 2, 3], vec![3, 2, 1]]), 6);
    assert_eq!(maximum_wealth(vec![vec![1, 5], vec![7, 3], vec![3, 5]]), 10);
    assert_eq!(maximum_wealth(vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]]), 17);
}

#[test]
fn test_calculate_maximum_wealth() {
    assert_eq!(calculate_maximum_wealth(1), 0);
    assert_eq!(calculate_maximum_wealth(2), 1);
    assert_eq!(calculate_maximum_wealth(1000), 499500);
}
