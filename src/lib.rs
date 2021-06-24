use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;
use pyo3::exceptions::PyTypeError;

mod fib_calcs;

use fib_calcs::fib_number;


#[pyfunction]
pub fn fib_number_two(number: i32) -> i32 {
    println!("The number is: {}", number);
    return 1
}


#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(fib_number, m)?)?;
    m.add_wrapped(wrap_pyfunction!(fib_number_two));
    // m.add_function(wrap_pyfunction!(generate_message, m)?)?;
    // m.add_function(wrap_pyfunction!(send_message, m)?)?;
    // m.add_class::<Planet>()?;
    Ok(())
}