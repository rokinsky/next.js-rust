use wasm::euclid::{lcm, gcd};

#[test]
fn test_lcm() {
    assert_eq!(lcm(12, 18), 36);
    assert_eq!(lcm(2, 0), 0);
    assert_eq!(lcm(-6, 14), 42);
}

#[test]
fn test_gcd() {
    assert_eq!(gcd(12, 18), 6);
    assert_eq!(gcd(-4, 14), 2);
}
