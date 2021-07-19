use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use std::time::{SystemTime, UNIX_EPOCH};


mod fib_calcs;
mod interface;
mod class_module;

use fib_calcs::fib_number::__pyo3_get_function_fibonacci_number;
use fib_calcs::fib_numbers::__pyo3_get_function_fibonacci_numbers;
use interface::config::__pyo3_get_function_run_config;
use interface::object::__pyo3_get_function_object_interface;
// use class_module::fib_processor::FibProcessor;
// use interface::test::MyClass;


#[pyfunction]
fn say_hello() {
    println!("saying hello from Rust!");
}

fn time_add_vectors(total_vector_size: i32) -> u128 {
    let start = SystemTime::now();
    let total_vector_size: i32 = 10;
    
    let mut buffer: Vec<i32> = Vec::new();
    let first_vector: Vec<i32> = (0..total_vector_size.clone()).map(|x| x).collect();
    let second_vector: Vec<i32> = (0..total_vector_size).map(|x| x).collect();
    
    for i in &first_vector {
        buffer.push(first_vector[**&i as usize] + second_vector[*i as usize]);
    }
   return start.duration_since(UNIX_EPOCH).expect("Time went backwards").as_millis()
}

#[pyfunction]
fn process_range(total: i32) -> Vec<u128> {
    let mut buffer: Vec<u128> = Vec::new();
    let total_jobs: Vec<i32> = (0..total).map(|x| x).collect();

    for i in total_jobs {
        buffer.push(time_add_vectors(i));
    }
    return buffer
}


#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(fib_number, m)?)?;
    m.add_wrapped(wrap_pyfunction!(say_hello));
    m.add_wrapped(wrap_pyfunction!(fibonacci_number));
    m.add_wrapped(wrap_pyfunction!(fibonacci_numbers));
    m.add_wrapped(wrap_pyfunction!(run_config));
    m.add_wrapped(wrap_pyfunction!(object_interface));
    m.add_wrapped(wrap_pyfunction!(process_range));
    // m.add_class::<FibProcessor>()?;
    // m.add_wrapped(wrap_pyfunction!(fibonacci_reccursive));
    // m.add_wrapped(wrap_pyfunction!(internal_fib_number));
    // m.add_function(wrap_pyfunction!(generate_message, m)?)?;
    // m.add_function(wrap_pyfunction!(send_message, m)?)?;
    // m.add_class::<MyClass>()?;
    Ok(())
}