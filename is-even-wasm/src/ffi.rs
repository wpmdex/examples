use super::is_even_impl;

#[no_mangle]
pub extern "C" fn is_even(n: i32) -> bool {
    is_even_impl(n)
}

#[no_mangle]
pub extern "C" fn is_odd(n: i32) -> bool {
    !is_even_impl(n)
}
