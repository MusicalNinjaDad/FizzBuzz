use std::{borrow::Cow, ops::Neg};

use fizzbuzz::{FizzBuzz, FizzBuzzAnswer, MultiFizzBuzz};
use pyo3::{exceptions::PyValueError, prelude::*, types::PySlice};
use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};

#[derive(FromPyObject)]
enum FizzBuzzable {
    Int(isize),
    Float(f64),
    Vec(Vec<isize>),
    Slice(MySlice),
}

#[derive(FromPyObject)]
struct MySlice {
    start: isize,
    stop: isize,
    step: Option<isize>,
}

impl IntoPy<Py<PyAny>> for MySlice {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        PySlice::new_bound(py, self.start, self.stop, self.step.unwrap_or(1)).into_py(py)
    }
}

/// A wrapper struct for FizzBuzzAnswer to provide a custom implementation of `IntoPy`.
enum FizzBuzzReturn {
    One(String),
    Many(Vec<Cow<'static, str>>),
}

impl From<FizzBuzzAnswer> for FizzBuzzReturn {
    fn from(value: FizzBuzzAnswer) -> Self {
        FizzBuzzReturn::One(value.into())
    }
}

impl IntoPy<Py<PyAny>> for FizzBuzzReturn {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        match self {
            FizzBuzzReturn::One(answer) => answer.into_py(py),
            FizzBuzzReturn::Many(answers) => answers.into_py(py),
        }
    }
}

/// Returns the correct fizzbuzz answer for any number or list/range of numbers.
///
/// This is an optimised algorithm compiled in rust. Large lists will utilise multiple CPU cores for processing.
/// Passing a slice, to represent a range, is fastest.
///
/// Arguments:
///     n: the number(s) to fizzbuzz
///
/// Returns:
///     In the case of a single number: a `str` with the correct fizzbuzz answer.
///     In the case of a list or range of inputs: a `list` of `str` with the correct fizzbuzz answers.
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
///     ['1', 'fizz']
///     ```
///     a `slice` representing a range:
///     ```
///     from fizzbuzz.fizzbuzzo3 import fizzbuzz
///     >>> fizzbuzz(slice(1,4,2))
///     ['1', 'fizz']
///     >>> fizzbuzz(slice(1,4))
///     ['1', '2', 'fizz']
///     >>> fizzbuzz(slice(4,1,-1))
///     ['4', 'fizz', '2']
///     >>> fizzbuzz(slice(1,5,-1))
///     []
///     ```
///     Note: Slices are inclusive on the left, exclusive on the right and can contain an optional step.
///     Note: Slices are inclusive on the left, exclusive on the right and can contain an optional step.
///     Negative steps require start > stop, positive steps require stop > start; other combinations return `[]`.
///     A step of zero is invalid and will raise a `ValueError`.
#[pyfunction]
#[pyo3(name = "fizzbuzz", text_signature = "(n)")]
fn py_fizzbuzz(num: FizzBuzzable) -> PyResult<FizzBuzzReturn> {
    match num {
        FizzBuzzable::Int(n) => Ok(n.fizzbuzz().into()),
        FizzBuzzable::Float(n) => Ok(n.fizzbuzz().into()),
        FizzBuzzable::Vec(v) => Ok(FizzBuzzReturn::Many(v.fizzbuzz().collect())),
        FizzBuzzable::Slice(s) => match s.step {
            // Can only be tested from python: Cannot create a PySlice with no step in rust.
            None => Ok(FizzBuzzReturn::Many((s.start..s.stop).fizzbuzz().collect())), // GRCOV_EXCL_LINE

            Some(1) => Ok(FizzBuzzReturn::Many((s.start..s.stop).fizzbuzz().collect())),

            Some(step) => match step {
                1.. => Ok(FizzBuzzReturn::Many(
                    (s.start..s.stop)
                        .into_par_iter()
                        .step_by(step.try_into().unwrap())
                        .fizzbuzz()
                        .collect(),
                )),

                //  ```python
                //  >>> foo[1:5:0]
                //  Traceback (most recent call last):
                //    File "<stdin>", line 1, in <module>
                //  ValueError: slice step cannot be zero
                //  ```
                0 => Err(PyValueError::new_err("step cannot be zero")),

                //  ```python
                //  >>> foo=[0,1,2,3,4,5,6]
                //  >>> foo[6:0:-2]
                //  [6, 4, 2]
                //  ```
                // Rust doesn't accept step < 0 or stop < start so need some trickery
                ..=-1 => Ok(FizzBuzzReturn::Many(
                    (s.start.neg()..s.stop.neg())
                        .into_par_iter()
                        .step_by(step.neg().try_into().unwrap())
                        .map(|x| x.neg())
                        .fizzbuzz()
                        .collect(),
                )),
            },
        },
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
    use pyo3::exceptions::{PyTypeError, PyValueError};
    use pyo3_testing::{pyo3test, with_py_raises};

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
        let expected = vec![
            "1".to_string(),
            "2".to_string(),
            "fizz".to_string(),
            "4".to_string(),
            "buzz".to_string(),
        ];
        let result: Vec<String> = fizzbuzz!(input);
        assert_eq!(result, expected);
    }

    #[pyo3test]
    #[allow(unused_macros)]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_string() {
        with_py_raises!(PyTypeError, { fizzbuzz.call1(("4",)) })
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_slice() {
        let input = MySlice {
            start: 1,
            stop: 6,
            step: Some(1),
        };
        let expected = vec![
            "1".to_string(),
            "2".to_string(),
            "fizz".to_string(),
            "4".to_string(),
            "buzz".to_string(),
        ];
        let result: Vec<String> = fizzbuzz!(input);
        assert_eq!(result, expected);
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_slice_no_step() {
        let input = MySlice {
            start: 1,
            stop: 6,
            step: None,
        };
        let expected = vec![
            "1".to_string(),
            "2".to_string(),
            "fizz".to_string(),
            "4".to_string(),
            "buzz".to_string(),
        ];
        let result: Vec<String> = fizzbuzz!(input);
        assert_eq!(result, expected);
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_slice_step() {
        let input = MySlice {
            start: 1,
            stop: 6,
            step: Some(2),
        };
        let expected = vec!["1".to_string(), "fizz".to_string(), "buzz".to_string()];
        let result: Vec<String> = fizzbuzz!(input);
        assert_eq!(result, expected);
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_slice_backwards() {
        let input = MySlice {
            start: 5,
            stop: 0,
            step: Some(1),
        };
        let result: Vec<String> = fizzbuzz!(input);
        let expected: Vec<String> = vec![];
        assert_eq!(result, expected);
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_slice_negative_step() {
        let input = MySlice {
            start: 5,
            stop: 0,
            step: Some(-2),
        };
        let expected = vec!["buzz".to_string(), "fizz".to_string(), "1".to_string()];
        let result: Vec<String> = fizzbuzz!(input);
        assert_eq!(result, expected);
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_slice_negative_step_boundaries() {
        let input = MySlice {
            start: 5,
            stop: 1,
            step: Some(-1),
        };
        let expected = vec![
            "buzz".to_string(),
            "4".to_string(),
            "fizz".to_string(),
            "2".to_string(),
        ];
        let result: Vec<String> = fizzbuzz!(input);
        assert_eq!(result, expected);
    }

    #[pyo3test]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_slice_negative_step_boundaries_2() {
        let input = MySlice {
            start: 6,
            stop: 0,
            step: Some(-2),
        };

        let expected = vec!["fizz".to_string(), "4".to_string(), "2".to_string()];
        let result: Vec<String> = fizzbuzz!(input);
        assert_eq!(result, expected);
    }
    #[pyo3test]
    #[allow(unused_macros)]
    #[pyo3import(py_fizzbuzzo3: from fizzbuzzo3 import fizzbuzz)]
    fn test_fizzbuzz_slice_zero_step() {
        let slice: MySlice = py
            .eval_bound("slice(1,2,0)", None, None)
            .unwrap()
            .extract()
            .unwrap();
        with_py_raises!(PyValueError, { fizzbuzz.call1((slice,)) });
    }
}
