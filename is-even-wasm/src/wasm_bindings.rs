//! WebAssembly bindings: parity checks exported to JavaScript via wasm-bindgen.

use wasm_bindgen::prelude::*;
use super::is_even_impl;

/// Returns `true` when `n` is even. Exported to JS as `is_even`.
#[wasm_bindgen]
pub fn is_even(n: i32) -> bool {
    is_even_impl(n)
}

/// Returns `true` when `n` is odd. Exported to JS as `is_odd`.
#[wasm_bindgen]
pub fn is_odd(n: i32) -> bool {
    !is_even_impl(n)
}
