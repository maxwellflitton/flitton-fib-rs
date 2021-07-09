use pyo3::prelude::{pyfunction, PyResult};
use pyo3::types::{PyDict, PyList};

use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::{fib_number, fib_numbers};


fn process_number(input_numbers: Vec<i32>) -> Vec<u64> {
    let mut buffer: Vec<u64> = Vec::new();

    for i in input_numbers {
        buffer.push(fibonacci_number(i));
    }
    return buffer
}



#[pyfunction]
pub fn run_config<'a>(config: &'a PyDict) -> PyResult<&'a PyDict> {
    match config.get_item("number") {
        Some(data) => {
            config.set_item("NUMBER RESULTS", data);
        },
        None => println!("number is not in the config")
    }
    match config.get_item("numbers") {
        Some(data) => println!("here is the data {:?}", data),
        None => println!("number is not in the config")
    }
    return Ok(config)
}
