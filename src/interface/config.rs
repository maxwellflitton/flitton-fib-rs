use pyo3::prelude::{pyfunction, PyResult};
use pyo3::types::{PyDict, PyList};

use crate::fib_calcs::fib_number::fibonacci_number;
use crate::fib_calcs::fib_numbers::fibonacci_numbers;


fn process_number(input_numbers: Vec<i32>) -> Vec<u64> {

    let test: Vec<u64> = input_numbers.iter().map(|x| fibonacci_number(*x)).collect();

    // let mut buffer: Vec<u64> = Vec::new();

    // for i in input_numbers {
    //     buffer.push(fibonacci_number(i));
    // }
    return test
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

            match data.downcast::<PyList>() {
                Ok(raw_data) => {
                    let processed_results: Vec<i32> = raw_data.extract::<Vec<i32>>().unwrap();
                    config.set_item("NUMBER RESULT", process_number(processed_results));
                },
                Err(_) => println!("parameter number is not a list of integers") 
            }
        },
        None => println!("parameter number is not in the config")
    }

    match config.get_item("numbers") {
        Some(data) => {
            
            match data.downcast::<PyList>() {
                Ok(raw_data) => {
                    let processed_results: Vec<Vec<i32>> = raw_data.extract::<Vec<Vec<i32>>>().unwrap();
                    config.set_item("NUMBERS RESULT", process_numbers(processed_results));
                },
                Err(_) => println!("parameter number is not a list of integers")
            }

        },
        None => println!("parameter numbers is not in the config")
    }
    return Ok(config)
}
