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

## Testing Rust code wrapped for use in Python

Testing that the Rust code you have created works, and works in Python can be simple. PyO3 includes
some helper macros to make this task easier when coupled with a few good practices.

This chapter of the Guide explains:

- [How to structure your code to make testing easier](#structuring-for-testability)
- [How to test your functionality](#testing-your-functionality-in-rust)
- [Testing your wrapping with `#[pyo3test]`](#testing-your-wrapped-functions-in-rust)
- [Final integration testing in Python](#testing-the-final-integration-in-python)
- [Compatibility with older Python versions (CI)](#compatibility-with-older-python-versions)

## Structuring for testability

If your code contains anything more than the most basic logic, you will probably want to test that it
functions correctly. This is best done in the Rust eco-system. Depending on

- whether you want to provide your library for use in rust (via crates.io)
- the overall complexity of your code base

you have two options:

1. For more complex libraries, or where you wish to provide a rust library as well as your Python
package: you should create a dedicated crate for your rust library and a second crate for the PyO3
bindings.
1. For simpler cases, or where your code is only destined to be used in Python: you should create your
basic functionality as rust modules and functions, without wrapping them using `[#pyo3...]`

In the first case: you can create both unit- and integration tests as defined and described in
["The Book"](https://doc.rust-lang.org/stable/book/ch11-00-testing.html) to validate your functionality.

In the second case: you are restricted to "unit tests" within the same source file as the code itself.
This can be perfectly adequate, as you will test integration with Python later...

For the remainder of this guide we will focus on the second case.

## Testing your wrapped functions in Rust

Once you are confident that your functionality is sound, you can wrap it for Python with a simple
one-liner:

```rust
#[pyfunction]
#[pyo3(name = "addone")]
fn py_addone(num: isize) -> isize {
    o3_addone(num)
}
```

and then create a Python module which can be imported:

```rust
#[pymodule]
#[pyo3(name = "adders")]
fn py_adders(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(py_addone, module)?)?;
    Ok(())
}
```

Still in Rust, you can test that the wrapped functionality can be executed by the Python interpreter.
PyO3 provides the `#[pyo3test]` proc-macro and associated `#[pyo3import(...)]` attribute to make this
simpler:

```rust
#[pyo3test]
#[pyo3import(py_adders: from adders import addone)]
fn test_one_plus_one_wrapped() {
    let result: PyResult<isize> = match addone.call1((1_isize,)) {
        Ok(r) => r.extract(),
        Err(e) => Err(e),
    };
    let result = result.unwrap();
    let expected_result = 2_isize;
    assert_eq!(result, expected_result);
}
```

`#[pyo3test]` takes care of wrapping the whole test case in `Python::with_gil(|py| {...})` and making
`addone` available in Rust.

> **Note:** running multiple tests which `#[pyo3import]` the same wrapped module requires _at least python3.9_.
>
> This does not affect which systems you can build and release for, only the interpreter used for these tests.

In a non-trivial case, you will likely have Type conversions and Error handling which you wish to
validate at this point.

## The full example in Rust

The full code then looks like this:

```rust
use pyo3::prelude::*;

/// Add one to an isize
fn o3_addone(num: isize) -> isize {
    num + 1
}

/// Rust function for use in Python which adds one to a given int
#[pyfunction]
#[pyo3(name = "addone")]
fn py_addone(num: isize) -> isize {
    o3_addone(num)
}

/// A module containing various "adders", written in Rust, for use in Python.
#[pymodule]
#[pyo3(name = "adders")]
fn py_adders(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(py_addone, module)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Check that the `o3_addone` function correctly adds one to 1_isize
    #[test]
    fn test_one_plus_one () {
        let result = o3_addone(1_isize);
        asserteq!(result, 2_isize)
    }

    /// Check that the Python function `adders.addone` can be run in Python
    #[pyo3test]
    #[pyo3import(py_adders: from adders import addone)]
    fn test_one_plus_one_wrapped() {
        let result: PyResult<isize> = match addone.call1((1_isize,)) {
            Ok(r) => r.extract(),
            Err(e) => Err(e),
        };
        let result = result.unwrap();
        let expected_result = 2_isize;
        assert_eq!(result, expected_result);
    }
}
```

## Testing the final integration in Python

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
