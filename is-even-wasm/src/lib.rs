fn is_even_impl(n: i32) -> bool {
    n % 2 == 0
}

// WASM / JavaScript bindings — compiled only on wasm32
#[cfg(target_arch = "wasm32")]
mod wasm_bindings;

// C FFI — excluded on wasm32 to avoid export name collision with wasm_bindgen
#[cfg(not(target_arch = "wasm32"))]
pub mod ffi;

// Python bindings — opt-in via --features python
#[cfg(feature = "python")]
mod python_bindings;

#[cfg(test)]
mod tests {
    use super::is_even_impl;

    #[test]
    fn even() {
        assert!(is_even_impl(0));
        assert!(is_even_impl(2));
        assert!(is_even_impl(-4));
        assert!(!is_even_impl(1));
        assert!(!is_even_impl(7));
    }
}
