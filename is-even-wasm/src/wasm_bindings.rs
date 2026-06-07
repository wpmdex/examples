use wasm_bindgen::prelude::*;
use super::is_even_impl;

#[wasm_bindgen]
pub fn is_even(n: i32) -> bool {
    is_even_impl(n)
}

#[wasm_bindgen]
pub fn is_odd(n: i32) -> bool {
    !is_even_impl(n)
}
