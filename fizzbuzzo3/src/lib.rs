use fizzbuzz::FizzBuzz;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "fizzbuzz")]
fn py_fizzbuzz(num: i32) -> String {
    num.fizzbuzz()
}

#[pymodule]
#[pyo3(name = "fizzbuzzo3")]
fn py_fizzbuzzo3(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(py_fizzbuzz, module)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use pyo3::exceptions::PyTypeError;

    use super::*;

    #[test]
    fn test_fizzbuzz() {
        pyo3::append_to_inittab!(py_fizzbuzzo3);
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let fizzbuzzo3 = py.import_bound("fizzbuzzo3").expect("Failed to import fizzbuzzo3");
            let fizzbuzz = fizzbuzzo3.getattr("fizzbuzz").expect("Failed to get fizzbuzz function");
            let result: PyResult<String> = match fizzbuzz.call1((1i32,)) {
                Ok(r) => r.extract(),
                Err(e) => Err(e),
            };
            let result = result.unwrap();
            let expected_result = "1";
            assert_eq!(result, expected_result);
        });
    }
    #[test]
    fn test_fizzbuzz_string() {
        pyo3::append_to_inittab!(py_fizzbuzzo3);
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let fizzbuzzo3 = py.import_bound("fizzbuzzo3").expect("Failed to import fizzbuzzo3");
            let fizzbuzz = fizzbuzzo3.getattr("fizzbuzz").expect("Failed to get fizzbuzz function");
            let result: PyResult<bool> = match fizzbuzz.call1(("one",)) {
                Ok(_) => Ok(false),
                Err(error)if error.is_instance_of::<PyTypeError>(py) => Ok(true),
                Err(e) => Err(e),
            };
            assert!(result.unwrap());
        });
    }
}
