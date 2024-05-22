# Testing

If you have structured your code into three sections as suggested in [structuring the codebase](structure.md) then testing will be simplest.

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

    ??? rust "**`rust/fizzbuzz/src/lib.rs`** - full source with unit tests at the end"
        ```rust
        --8<-- "rust/fizzbuzz/src/lib.rs"
        ```

    ??? rust "**`rust/fizzbuzz/tests/test_vec.rs`** (simple example of an integration test) - full source"
        ```rust
        --8<-- "rust/fizzbuzz/tests/test_vec.rs"
        ```

## Code wrapped by pyo3

Given that you have tested the core code functionality well, you don't need to repeat a full suite of functional tests on the wrapped code. Instead you can focus on the wrapping and any type and error handling you implemented.

!!! rocket "The `pyo3-testing` crate"
    The `pyo3-testing` crate is designed to make this step simple.

    It is currently available on a forked version of pyo3, I'll release it separately as a dedicated crate ASAP unless [PR pyo3/#4099](https://github.com/PyO3/pyo3/pull/4099) lands in the meantime.

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

??? pyo3 "**`rust/fizzbuzzo3/src/lib.rs`** - full source (tests at the end)"
    ```rust
    --8<-- "rust/fizzbuzzo3/src/lib.rs"
    ```

## Integration testing with pytest

Now that you are confident that your functionality is correct and your wrappings work, you can create your final tests in Python to validate that the wrapped functionality imports and really does what you expect when called from python.

??? python "pytest vs. unittest"
    Python offers testing via the [`unittest` module](https://docs.python.org/3/library/unittest.html) in the standard library. [pytest](https://docs.pytest.org/) is a central part of the python ecosystem which removes a lot of the boilerplate and provides a lot of additional features. You will find it used in an overwhelmingly large number of libraries.

!!! python "Testing that you can import your module"
    Testing the import mechanics is as simple as adding the import statement to the top of your test case.

    From **`tests/test_fizbuzzo3.py`**:
    ```python
    from fizzbuzz.fizzbuzzo3 import fizzbuzz
    ```

    If this doesn't work, you'll recieve an `ModuleNotFoundError` when running your tests

    !!! tip "Don't forget to recompile your wrapped functions"
        You'll need to (re)compile wrapped functions after changing them and before testing from python:
        ```sh
        (.venv)/projectroot$ pip install -e .
        ```

    !!! warning "Import and namespace errors"
        It's easy to end up in a situation where your pytests pass when you run them locally but imports fail at other times. See the warning in [Structuring the Codebase](structure.md#basic-structure) for more details.

        Avoid this problem by:
        1. Following the guidance on project structure
        1. **Not** placing any `__init__.py` files in your `tests` directory. Rely on `pip install -e .` to properly place your packing into `globals`

!!! python "Testing functionality for each type"
    Similar to how you focussed your tests in rust on the wrapped code, you want to also focus here on ensuring you cover all the possible types you can pass to your function. You probably also want to check the functionality a bit more as well, but not to the extent you did in the core rust code.

    From **`tests/test_fizbuzzo3.py`**:
    ```python
    ...
    def test_list():
      assert fizzbuzz([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 15]) == "1, 2, fizz, 4, buzz, fizz, 7, 8, fizz, buzz, fizzbuzz"
    ...
    ```

!!! python "Testing error cases"
    pytest offers an easy way to check that an Exception is raised using `raises`:

    From **`tests/test_fizbuzzo3.py`**:
    ```python
    import pytest
    ...
    def test_string():
    with pytest.raises(TypeError):
        fizzbuzz("1")
    ```

??? python "**`tests/test_fizzbuzzo3.py`** - full source:"
    ```python
    --8<-- "tests/test_fizzbuzzo3.py"
    ```

!!! python "Doctests"
    If you've followed the guidance on type hinting (later) you can also run [doctests](https://docs.python.org/3/library/doctest.html) on the docstrings attached to your wrapped functions.

    Add the following to **`./pyproject.toml`**:
    ```toml
    [tool.pytest.ini_options]
    ...
    addopts = [
        ...
        "--doctest-modules",
        "--doctest-glob=*.pyi",
        ...
    ]
    testpaths = ["tests", "python"]
    ...
    ```

    ??? python "**`./pyproject.toml`** - full source"
        ```toml
        --8<-- "./pyproject.toml"
        ```

## Running the tests

!!! rust "Running all rust tests (unit, integration, doc)"
    ```sh
    /projectroot$ cargo test -p fizzbuzz
    ```

    or

    ```sh
    /projectroot/rust/fizzbuzz$ cargo test
    ```

!!! pyo3 "Running all tests on wrapped code"
    ```sh
    /projectroot$ cargo test -p fizzbuzzo3
    ```

    or

    ```sh
    /projectroot/rust/fizzbuzzo3$ cargo test
    ```

!!! python "Running all python tests"
    ```sh
    (.venv)/projectroot$ pytest
    ```

    !!! warning "Don't forget to recompile your wrapped functions"
    You'll need to (re)compile wrapped functions after changing them and before testing from python:
    ```sh
    (.venv)/projectroot$ pip install -e .
    ```

!!! abstract "Running all tests _and lints_ with just"
    Just run `just check` or `just reset check` to also clean and recompile first

    ??? abstract "**`./justfile`** - full source"
        ```justfile
        --8<-- "justfile"
        ```

## TODO - CI
