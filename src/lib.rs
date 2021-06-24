use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::types::PyDict;
use pyo3::exceptions::PyTypeError;

mod fib_calcs;

use fib_calcs::fib_number::internal_fib_number;


#[pyfunction]
pub fn fib_number_two(number: i32) -> i32 {
    println!("The number is: {}", number);
    return 1
}


#[pyfunction]
pub fn fibonacci_reccursive(n: i32) -> u64 {
	if n < 0 {
		panic!("{} is negative!", n);
	}
	match n {
		0     => panic!("zero is not a right argument to fibonacci_reccursive()!"),
		1 | 2 => 1,
        3     => 2,
		_     => fibonacci_reccursive(n - 1) + fibonacci_reccursive(n - 2)
	}
}


#[pymodule]
fn flitton_fib_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_function(wrap_pyfunction!(fib_number, m)?)?;
    m.add_wrapped(wrap_pyfunction!(fib_number_two));
    m.add_wrapped(wrap_pyfunction!(fibonacci_reccursive));
    m.add_wrapped(wrap_pyfunction!(internal_fib_number));
    // m.add_function(wrap_pyfunction!(generate_message, m)?)?;
    // m.add_function(wrap_pyfunction!(send_message, m)?)?;
    // m.add_class::<Planet>()?;
    Ok(())
}