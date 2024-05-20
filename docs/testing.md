# Testing

If you have structured your code into three sections as suggested in [Structuring the codebase](structure.md) then testing will be simplest.

## Core functionality in rust

Comprehensively testing your functionality directly in Rust gives you the fastest test execution and
makes finding any issues easier, as you know that they can only originate in the underlying Rust functions,
not in your wrapping, importing or use in Python.

??? rust "Two levels of tests out-of-the-box"
    Rust supports two levels of tests and names them "Unit tests" and "Integration tests". I will stick to those terms for this section.

    - Unit tests:
        1. Go directly into the source file with the code they are testing.
        1. You should expect these tests to be more tightly coupled to the implementation detail and to have a higher change frequency for that reason. I find that I start out with many tests like this as I code new internal APIs for other parts of my code to use. Over time I delete many of them if I find that they add more cost than value when refactoring. When I break out a generic function I'll add new ones to help check it works as expected.
        1. Go in a dedicated module (called `test` by convention) and are annotated to be excluded from final builds:
        ```rust
        #[cfg(test)]
        mod test {
            use super::*;
            
            #[test]
            fn vec_to_string() {
        ...
        ```
    - Integration tests:
        1. Go in a separate directory `tests`.
        1. You should expect these tests to be coupled to your public API. Changes to these tests are a clear indication of the type of version number change you should apply.
        1. Changing the internal implementation details of your code base should _not_ require you to change these tests.
        1. Can consist of as many different `.rs` files as you like.
        1. Need to import your library, just as your users would and each test needs to be annotated as such for the compiler:
        ```rust
        use fizzbuzz::MultiFizzBuzz;

        mod vectors {

            use super::*;

            #[test]
            fn test_small_vec() {
        ...
        ```
  
    [Chapter 11 of the rust book](https://doc.rust-lang.org/book/ch11-00-testing.html) contains more details

??? rust "`rust/fizzbuzz/src/lib.rs` - full source with unit tests at the end"
    ```rust
    --8<-- "rust/fizzbuzz/src/lib.rs"
    ```

??? rust "`rust/fizzbuzz/tests/test_vec.rs`" (simple example) - full source"
    ```rust
    --8<-- "rust/fizzbuzz/tests/test_vec.rs"
    ```

## Code wrapped by pyo3

Given that you have tested the core code functionality well, you don't need to repeat a full suite of functional tests on the wrapped code. Instead you can focus on the wrapping and any type and error handling you implemented.

!!! rocket "The `pyo3-testing` crate"
    The `pyo3-testing` crate is designed to make this step simple. It is currently available on a forked version of pyo3, I'll release it separately as a dedicated crate ASAP unless [PR pyo3/#4099](https://github.com/PyO3/pyo3/pull/4099) lands in the meantime.

    Use it by importing pyo3 from the fork in **rust/fizzbuzzo3/Cargo.toml**:
    ```toml
    ...
    [dependencies]
      pyo3 = { git = "https://github.com/MusicalNinjaDad/pyo3.git", branch = "pyo3-testing" }
    ...
    ```

??? warning "Testing without the `pyo3-testing` crate"
    If you chose to test without the `pyo3-testing` crate you will need to make use of the guidance in [Chapter 3 of the pyo3 book](https://pyo3.rs/latest/python-from-rust) and also be willing to accept random failures due to mistimed interpreter initialisation.

    You can take a look at [an example case from pyo3-testing](https://github.com/MusicalNinjaDad/pyo3/blob/pyo3-testing/tests/test_pyo3test.rs) to see how I worked around these issues.

!!! pyo3 "Testing the wrapping once for each supported argument type"
    Use pyo3's capability to embed a python interpreter and call python code from rust to create one test per type to check that you get the right result when you call the resulting python function:

    Example from **`rust/fizzbuzzo3/src.rs`**:
    ```rust
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
    ```

!!! pyo3 "Testing error cases"
    Again using pyo3's ablity to embed python in rust, check that you get the expected python Exception type from bad input.

    Example from **`rust/fizzbuzzo3/src.rs`**:
    ```rust
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
    ```

!!! warning "Only unit tests possible"
    You can only create tests directly in the source file along side your wrappings. This should be fine, as your integration tests will be run natively by pytest.

    If for some reason you did want to create external rust tests you need to change the library type in `rust/fizzbuzzo3/Cargo.toml` to `crate-type = ["cdylib, lib"]`

??? pyo3 "`rust/fizzbuzzo3/src.rs` - full source (tests at the end)"
    ```rust
    --8<-- "rust/fizzbuzzo3/src.rs"
    ```

## Integration testing with pytest



Now that you are confident that your functionality is correct and your wrappings work, you can create
your final tests in Python, using either pytest or unittest. In this guide we will use pytest for the
examples.

```python
from adders import addone

def test_one_plus_one ():
    assert addone(1) == 2
```

Before you can execute this test, you need to compile and install your rust library.

The easiest way to do this, with both maturin and setuptools-rust is to run `pip install -e .` in the
root of your Python package. This will build and install the package in editable mode.

Note: If you have additional dependencies for development, e.g.: pytest, then you will need to install
these manually, or include them as optional dependencies in `pyproject.toml` e.g.:

```toml
[project.optional-dependencies]
dev = [
    "pytest",
    ]
```

and then run `pip install -e .[dev]`

## Compatibility with older Python versions

Due to limitations in older Python interpreters the `#[pyo3test]` macro can only be used with cPython >= 3.9,
it is also not compatible with PyPy or GraalPy. This is because the macro attempts to (re-)initialise your
wrapped `PyModule` for each test case and this is not handled well in older versions if done in the same
interpreter instance.

Your wrapped code can still be built for, and used in, other versions of Python as per standard Pyo3 compatibility.
You should ensure that any automated CI tasks which run on multiple versions of Python skip these tests where
applicable and only execute the rust unit tests and python-side integration tests.
