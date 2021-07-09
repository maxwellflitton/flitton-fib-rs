use pyo3::prelude::{pyfunction, PyResult};
use pyo3::types::{PyDict, PyList};

use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::fib_numbers::fibonacci_numbers;


fn process_number(input_numbers: &Vec<i32>) -> Vec<u64> {
    let mut buffer: Vec<u64> = Vec::new();

    for i in input_numbers {
        buffer.push(fibonacci_number(i));
    }
    return buffer
}


fn process_numbers(input_numbers: Vec<Vec<i32>>) -> Vec<Vec<u64>> {
    let mut buffer: Vec<Vec<u64>> = Vec::new();
    for i in input_numbers {
        buffer.push(fibonacci_numbers(i));
    }
    return buffer
}


#[pyfunction]
pub fn run_config<'a>(config: &'a PyDict) -> PyResult<&'a PyDict> {
    match config.get_item("number") {
        Some(data) => {
            let data_buffer = data.to_string();
            
            match data_buffer.parse::<Vec<Vec<i32>>>() {
                Ok(raw_data) => println!("it's working"),
                Err(_) => println!("cannot be parsed") 
            }

            // match data.downcast::<Vec<i32>>() {
            //     Ok(unwrapped_data) => config.set_item("NUMBER RESULTS", 
            //     process_number(unwrapped_data)),
            //     Err => println!("number is not a list of integers")
            // }
        },
        None => println!("number is not in the config")
    }
    match config.get_item("numbers") {
        Some(data) => {
            config.set_item("NUMBERS RESULTS", data);
        },
        None => println!("number is not in the config")
    }
    return Ok(config)
}
