use pyo3::prelude::{pyclass, pymethods};


#[pyclass]
pub struct FibProcessor {
    pub number: Vec<i32>,
    pub numbers: Vec<Vec<i32>>
}

#[pymethods]
impl FibProcessor {

    #[new]
    fn new(number: Vec<i32>, numbers: Vec<Vec<i32>>) -> Self {
        return FibProcessor { number, numbers }
    }

}