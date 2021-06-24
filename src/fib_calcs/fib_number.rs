use pyo3::prelude::*;
// use pyo3::wrap_pyfunction;


#[pyfunction]
pub fn internal_fib_number(number: i32) -> i32 {
    println!("The number is: {}", number);
    return 1
}