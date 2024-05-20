# Developing and Building

This section won't go into the actual coding of your core code in rust - see the excursions for that. Assuming that you have your core code written (in rust):

## Wrapping with pyo3

The [relevant section of the pyo3 book](https://pyo3.rs/latest/rust-from-python) does a great job of explaining how to wrap your code, so I'll just touch the highlights here:

!!! pyo3 "rust/fizzbuzzo3/Cargo.toml"
    1. Name your library the way you want the module to appear in python. For example to import using `from fizzbuzz import fizzbuzzo3`
    1. Use the `cdylib` library type
    1. Add a dependency to `pyo3`

    ```toml
    ...
    [lib]
      name = "fizzbuzzo3"
      path = "src/lib.rs"
      crate-type = ["cdylib"]  # cdylib required for python import, rlib required for rust tests.

    [dependencies]
      pyo3 = { git = "https://github.com/MusicalNinjaDad/pyo3.git", branch = "pyo3-testing" }
    ...
    ```

    Note: for now this uses a git dependency to a branch on my fork - until either [PR pyo3/#4099](https://github.com/PyO3/pyo3/pull/4099) lands or I pull the testing support out into an independent crate

!!! warning "Use the same name for the library and exported module"
    I have not spent much time trying but I couldn't get the import to work if you have different names for the library and imported module. Trying to rename the library to `fizzbuzzo3lib` leads to a file like `python/fizzbuzz/fizzbuzzo3lib.cpython-312-x86_64-linux-gnu.so` being generated but unusable:

    ```pycon
    >>> from fizzbuzz import fizzbuzzo3
    Traceback (most recent call last):
    File "<stdin>", line 1, in  <module>
    ImportError: cannot import name 'fizzbuzzo3' from 'fizzbuzz' (/workspaces/FizzBuzz/python/fizzbuzz/__init__.py)
    >>> from fizzbuzz import fizzbuzzo3lib
    Traceback (most recent call last):
    File "<stdin>", line 1, in <module>
    ImportError: dynamic module does not define module export function (PyInit_fizzbuzzo3lib)
    ```

??? pyo3 "Full source: rust/fizzbuzzo3/Cargo.toml"
    ```toml
    --8<-- "rust/fizzbuzzo3/Cargo.toml"
    ```

## venv / `pip -e.[dev]`

## performance / API design
