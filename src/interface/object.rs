use pyo3::prelude::{pyfunction, PyResult, Python};
use pyo3::types::{PyAny, PyList, PyDict};
use pyo3::exceptions::PyTypeError;

use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::fib_numbers::fibonacci_numbers;
use super::config::run_config;


#[pyfunction]
pub fn object_interface<'a>(input_object: &'a PyAny) -> PyResult<&'a PyAny> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    let config_dict: &PyDict = PyDict::new(py);

    match input_object.getattr("number") {
        Ok(data) => {
            config_dict.set_item("number", data).unwrap();
        },
        Err(_) => println!("the object does not have number")
    }

    match input_object.getattr("numbers") {
        Ok(data) => {
            config_dict.set_item("numbers", data).unwrap();
        }
        Err(_) => println!("the object does not have numbers")
    }

    let output_dict: &PyDict = run_config(config_dict).unwrap();

    input_object.setattr("number_results", output_dict.get_item("NUMBER RESULT").unwrap()).unwrap();
    input_object.setattr("numbers_results", output_dict.get_item("NUMBERS RESULT").unwrap()).unwrap();

    return Ok(input_object)
}
