use pyo3::prelude::pyclass;


#[pyclass]
pub struct MyClass {
    num: i32,
    debug: bool,
}
