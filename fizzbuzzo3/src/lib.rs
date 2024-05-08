use fizzbuzz::FizzBuzz;
use pyo3::prelude::*;

#[derive(FromPyObject)]
enum PyNum {
    Int(isize),
    Float(f64),
}

#[pyfunction]
#[pyo3(name = "fizzbuzz")]
fn py_fizzbuzz(num: PyNum) -> String {
    match num {
        PyNum::Int(n) => n.fizzbuzz(),
        PyNum::Float(n) => n.fizzbuzz(),
    }
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
    use pyo3::types::PyDict;

    use super::*;

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz() {
        let result: PyResult<String> = match fizzbuzz.call1((1i32,)) {
            Ok(r) => r.extract(),
            Err(e) => Err(e),
        };
        let result = result.unwrap();
        let expected_result = "1";
        assert_eq!(result, expected_result);
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_string() {
        let result: PyResult<bool> = match fizzbuzz.call1(("one",)) {
            Ok(_) => Ok(false),
            Err(error) if error.is_instance_of::<PyTypeError>(py) => Ok(true),
            Err(e) => Err(e),
        };
        assert!(result.unwrap());
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_float() {
        let result: PyResult<String> = match fizzbuzz.call1((1f32,)) {
            Ok(r) => r.extract(),
            Err(e) => Err(e),
        };
        let result = result.unwrap();
        let expected_result = "1";
        assert_eq!(result, expected_result);
    }
}
