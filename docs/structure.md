# Structuring the codebase

```text
FizzBuzz
- Cargo.toml
- pyproject.toml
- MANIFEST.in
- rust
  - fizzbuzz
    - Cargo.toml
    - src
      ... core rust code goes here
    - tests
      ... tests for core rust code go here
  - fizzbuzzo3
    Cargo.toml
    - src
      ... pyo3 code goes here
- python
  - fizzbuzz
    - __init__.py
    ... additional python code goes here
- tests
  ... python tests go here
```

If your code contains anything more than the most basic logic, you will probably want to test that it
functions correctly. This is best done as close the the code as possible in the Rust eco-system. Depending on

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