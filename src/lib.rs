use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::{PyAny, PyDict, PyList};


mod fib_calcs;
mod interface;
mod class_module;

use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number;
use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;
use interface::config::__pyo3_get_function_run_config;
use interface::object::__pyo3_get_function_object_interface;
use class_module::fib_processor::FibProcessor;


#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}

#[pyfunction]
fn time_add_vectors(total_vector_size: i32) -> Vec<i32> {
    let total_vector_size: i32 = 10;
    
    let mut buffer: Vec<i32> = Vec::new();
    let first_vector: Vec<i32> = (0..total_vector_size.clone()).map(|x| x).collect();
    let second_vector: Vec<i32> = (0..total_vector_size).map(|x| x).collect();
    
    for i in &first_vector {
        buffer.push(first_vector[**&i as usize] + second_vector[*i as usize]);
    }
   return buffer
}

#[pyfunction]
fn test_numpy() {
    let gil = Python::acquire_gil();
    let py = gil.python();
    let locals = [("numpy", py.import("numpy")?)].into_py_dict(py);
    let code = "numpy.array([[3, 2], [1, 4]])";
    let weights_matrix = py.eval(code, None, Some(&locals))?.extract()?;
    return weights_matrix
}


#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(say_hello));
    m.add_wrapped(wrap_pyfunction!(fibonacci_number));
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers));
    m.add_wrapped(wrap_pyfunction!(run_config));
    m.add_wrapped(wrap_pyfunction!(object_interface));
    m.add_wrapped(wrap_pyfunction!(time_add_vectors));
    m.add_wrapped(wrap_pyfunction!(time_add_vectors));
    m.add_class::<FibProcessor>()?;
    Ok(())
}