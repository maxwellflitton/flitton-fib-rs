use pyo3::prelude::{pyfunction, PyResult};
use pyo3::types::{PyAny, PyList};
use pyo3::exceptions::PyTypeError;

use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::fib_numbers::fibonacci_numbers;


pub fn object_interface<'a>(input_object: &'a PyAny) -> PyResult<&'a PyAny> {
    match input_object.getattr("number") {
        Some(_) => println!("the object has number"),
        None => println!("the object does not have number")
    }
    return PyResult(input_object)
}
