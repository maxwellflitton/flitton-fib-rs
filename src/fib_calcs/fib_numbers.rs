use std::vec::Vec;

use pyo3::prelude::pyfunction;
use pyo3::types::PyList;

use super::fib_number::fibonacci_number;


#[pyfunction]
pub fn fibonacci_numbers(numbers: Vec<u64>) -> u64 {
    let mut vec: Vec<u64> = Vec::new();
	
    for n in numbers.iter() {
        println!("{:?}", n);
    }
    return 24
}
