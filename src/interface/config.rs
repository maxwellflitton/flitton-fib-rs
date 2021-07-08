use pyo3::prelude::{pyfunction, PyResult, PyDict};

extern crate fib_calcs;

use fib_calcs::{fib_number, fib_numbers};


#[pyfunction]
pub fn run_config<'a>(config: &'a PyDict) -> PyResult<&'a PyDict> {
    match config.get_item("number") {
        Some(data) => println!("here is the data {:?}", data),
        None => println!("number is not in the config")
    }
    match config.get_item("numbers") {
        Some(data) => println!("here is the data {:?}", data),
        None => println!("number is not in the config")
    }
    return config
}
