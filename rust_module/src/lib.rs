use pyo3::prelude::*;
use numpy::{PyReadonlyArray1, IntoPyArray};

/// Formats the sum of two numbers as a string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn sub_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a - b).to_string())
}

#[pyfunction]
fn mult_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a * b).to_string())
}

// Sum an array of numbers as a string.
#[pyfunction]
fn sum_array_as_string(arr: Vec<usize>) -> PyResult<String> {
    Ok(arr.iter().sum::<usize>().to_string())
}

#[pyfunction]
fn sum_array_fast(arr: PyReadonlyArray1<'_, i64>) -> PyResult<i64> {
    let slice = arr.as_slice()?;
    Ok(slice.iter().copied().sum())
}


/// A Python module implemented in Rust.
#[pymodule]
fn rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sub_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(mult_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_array_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_array_fast, m)?)?;
    Ok(())
}