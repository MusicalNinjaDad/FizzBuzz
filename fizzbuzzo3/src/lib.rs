use fizzbuzz::FizzBuzz;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "fizzbuzz")]
fn py_fizzbuzz(num: i32) -> String {
    num.fizzbuzz()
}

#[pymodule]
#[pyo3(name = "fizzbuzzo3")]
fn py_fizzbuzzo3(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(py_fizzbuzz, module)?)?;
    Ok(())
}