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

// Sorting functions
#[pyfunction]
fn bubble_sort(arr: Vec<usize>) -> PyResult<Vec<usize>> {
    let mut arr = arr.clone();
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
    Ok(arr)
}

#[pyfunction]
fn quick_sort(arr: Vec<usize>) -> PyResult<Vec<usize>> {
    let mut arr = arr.clone();
    quick_sort_helper(&mut arr);
    Ok(arr)
}

fn quick_sort_helper(arr: &mut [usize]) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = arr[arr.len() / 2];
    let (left, right): (Vec<usize>, Vec<usize>) = arr.iter().partition(|&&x| x < pivot);
    let mut sorted = left;
    sorted.push(pivot);
    sorted.extend(right);
    arr.copy_from_slice(&sorted);
}

#[pyfunction]
fn merge_sort(arr: Vec<usize>) -> PyResult<Vec<usize>> {
    let mut arr = arr.clone();
    merge_sort_helper(&mut arr);
    Ok(arr)
}

fn merge_sort_helper(arr: &mut [usize]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    let mut left = arr[..mid].to_vec();
    let mut right = arr[mid..].to_vec();
    merge_sort_helper(&mut left);
    merge_sort_helper(&mut right);
    merge(arr, left, right);
}

fn merge(arr: &mut [usize], left: Vec<usize>, right: Vec<usize>) {
    let mut i = 0;
    let mut j = 0;
    for k in 0..arr.len() {
        if i < left.len() && (j >= right.len() || left[i] <= right[j]) {
            arr[k] = left[i];
            i += 1;
        } else {
            arr[k] = right[j];
            j += 1;
        }
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sub_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(mult_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_array_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_array_fast, m)?)?;
    m.add_function(wrap_pyfunction!(bubble_sort, m)?)?;
    m.add_function(wrap_pyfunction!(quick_sort, m)?)?;
    m.add_function(wrap_pyfunction!(merge_sort, m)?)?;
    Ok(())
}