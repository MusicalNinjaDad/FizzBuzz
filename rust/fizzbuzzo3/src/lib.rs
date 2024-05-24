use fizzbuzz::{FizzBuzz, MultiFizzBuzz};
use pyo3::prelude::*;

#[derive(FromPyObject)]
enum FizzBuzzable {
    Int(isize),
    Float(f64),
    Vec(Vec<isize>),
}

/// Returns the correct fizzbuzz answer for any number or list of numbers.
///
/// This is an optimised algorithm compiled in rust. Large lists will utilise multiple CPU cores for processing.
///
/// Arguments:
///     n: the number(s) to fizzbuzz
///
/// Returns:
///     A `str` with the correct fizzbuzz answer(s).
///     In the case of a list of inputs the answers will be separated by `, `
///
/// Examples:
///     a single `int`:
///     ```
///     >>> from fizzbuzz.fizzbuzzo3 import fizzbuzz
///     >>> fizzbuzz(1)
///     '1'
///     >>> fizzbuzz(3)
///     'fizz'
///     ```
///     a `list`:
///     ```
///     from fizzbuzz.fizzbuzzo3 import fizzbuzz
///     >>> fizzbuzz([1,3])
///     '1, fizz'
///     ```
#[pyfunction]
#[pyo3(name = "fizzbuzz", text_signature = "(n)")]
fn py_fizzbuzz(num: FizzBuzzable) -> String {
    match num {
        FizzBuzzable::Int(n) => n.fizzbuzz().into(),
        FizzBuzzable::Float(n) => n.fizzbuzz().into(),
        FizzBuzzable::Vec(v) => v.fizzbuzz().into(),
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
    use pyo3_testing::pyo3test;

    use super::*;

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz() {
        let result: String = fizzbuzz!(1i32);
        assert_eq!(result, "1");
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_float() {
        let result: String = fizzbuzz!(1f32);
        assert_eq!(result, "1");
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_vec() {
        let input = vec![1, 2, 3, 4, 5];
        let result: String = fizzbuzz!(input);
        assert_eq!(result, "1, 2, fizz, 4, buzz");
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
}
