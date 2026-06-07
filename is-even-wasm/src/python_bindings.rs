use pyo3::prelude::*;
use super::is_even_impl;

#[pyfunction]
fn is_even(n: i32) -> bool {
    is_even_impl(n)
}

#[pyfunction]
fn is_odd(n: i32) -> bool {
    !is_even_impl(n)
}

#[pymodule]
pub fn is_even_wasm(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_even, m)?)?;
    m.add_function(wrap_pyfunction!(is_odd, m)?)?;
    Ok(())
}
