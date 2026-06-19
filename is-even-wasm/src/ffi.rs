//! C FFI bindings: `extern "C"` parity checks usable from any C ABI consumer.

use super::is_even_impl;

/// Returns `true` when `n` is even. C ABI export.
#[no_mangle]
pub const extern "C" fn is_even(n: i32) -> bool {
    is_even_impl(n)
}

/// Returns `true` when `n` is odd. C ABI export.
#[no_mangle]
pub const extern "C" fn is_odd(n: i32) -> bool {
    !is_even_impl(n)
}
