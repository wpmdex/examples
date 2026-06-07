use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

#[wasm_bindgen]
pub fn is_odd(n: i32) -> bool {
    !is_even(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_even() {
        assert!(is_even(0));
        assert!(is_even(2));
        assert!(is_even(-4));
        assert!(!is_even(1));
        assert!(!is_even(7));
    }

    #[test]
    fn test_is_odd() {
        assert!(is_odd(1));
        assert!(is_odd(3));
        assert!(!is_odd(2));
    }
}
