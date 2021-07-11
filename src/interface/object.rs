use pyo3::prelude::{pyfunction, PyResult, Python};
use pyo3::types::{PyAny, PyList};
use pyo3::exceptions::PyTypeError;

use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::fib_numbers::fibonacci_numbers;


#[pyfunction]
pub fn object_interface<'a>(input_object: &'a PyAny) -> PyResult<&'a PyAny> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    match input_object.getattr("number") {
        Ok(_) => {
            input_object.setattr("number", PyList::new(py,&[1, 2, 3, 4, 5])).unwrap();
        },
        Err(_) => println!("the object does not have number")
    }
    return Ok(input_object)
}
