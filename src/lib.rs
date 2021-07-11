use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
// use pyo3::types::PyDict;
// use pyo3::exceptions::PyTypeError;

mod fib_calcs;
mod interface;
mod class_module;

use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number;
use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;
use interface::config::__pyo3_get_function_run_config;
use interface::object::__pyo3_get_function_object_interface;
use class_module::fib_processor::FibProcessor;
// use interface::test::MyClass;


#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}


#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(fib_number, m)?)?;
    m.add_wrapped(wrap_pyfunction!(say_hello));
    m.add_wrapped(wrap_pyfunction!(fibonacci_number));
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers));
    m.add_wrapped(wrap_pyfunction!(run_config));
    m.add_wrapped(wrap_pyfunction!(object_interface));
    m.add_class::<FibProcessor>()?;
    // m.add_wrapped(wrap_pyfunction!(fibonacci_reccursive));
    // m.add_wrapped(wrap_pyfunction!(internal_fib_number));
    // m.add_function(wrap_pyfunction!(generate_message, m)?)?;
    // m.add_function(wrap_pyfunction!(send_message, m)?)?;
    // m.add_class::<MyClass>()?;
    Ok(())
}