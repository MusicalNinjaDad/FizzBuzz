# Structuring the codebase

## Basic structure

!!! abstract "Keep your rust, wrapped-rust and python code separate"
    You want a top level directory structure like this - with directories for `rust`, `python` and `tests`. Within the `rust` directory you want separate directories for your main logic and the wrapped exports for python.

    ```text
    FizzBuzz
    - rust
      - fizzbuzz
        - src
          ... core rust code goes here
        - tests
          ... tests for core rust code go here
      - fizzbuzzo3
        - src
          ... pyo3 code goes here
    - python
      - fizzbuzz
        ... additional python code goes here
    - tests
      ... python tests go here
    ```

!!! warning "Warning: Import errors"
    Having any top-level directories with names that match your package leads to all kinds of fun import errors. Depending on the exact context, python can decide that you have implicit namespace packages which collide with your explicit package namespaces.

    I ran into problems twice:

    - firstly, I had a time where my tests ran fine but I couldn't import my code in repl;
    - later, the final wheels were unusable but sdist was fine.  
    
    There are also [reports](https://github.com/PyO3/maturin/issues/490) of end-users being the first to run into trouble

    This is also the reason for keeping your final set of python tests in a separate top-level directory: you can be sure they are using the right import logic.

## Considerations

1. When you distribute the final python library as source the primary audience are python-coders, make it understandable for them without a lot of extra explanation by putting your rust code in a clearly marked location
1. You will want to test your code at each stage of integration: core rust, wrapped rust, final python result; so that you can easily track down bugs
1. Context switching between two languages is hard, even if you know both parts of the codebase well. Keeping it structured by language boundary helps when coding. For much larger projects you may want to provide a higher-level structure by domain and _then_ structure each domain as above ... but that's outside the scope of a simple starter how-to :wink:
1. Your underlying code probably does something useful - so you could also publish it to the rust eco-system as a dedicated crate for others to use directly in rust. Those users don't want the extra code wrapping your function for python!

## Configuration files (pyproject.toml & Cargo.toml)

!!! python "./pyproject.toml"
    We'll be using `setuptools` as the main build agent and `pyproject.toml` to manage all the config and tooling - that way we keep the number of files to a minimum and stick to as few languages as possible.

    The important section is:

    ```toml
    [tool.setuptools.packages.find]
      where = ["python"]
    ```

    More info on `pyproject.toml` is available from both [pypa -  the python packaging authority](https://packaging.python.org/en/latest/guides/writing-pyproject-toml/) and [pip - python's package manager](https://pip.pypa.io/en/stable/reference/build-system/pyproject-toml/)

!!! rust "./Cargo.toml"
    You need to set up a [cargo workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)

    ```toml
    [workspace]

      members = [
          "rust/*"
      ]

      resolver = "2"
    ```

!!! pyo3 "rust/fizzbuzzo3/Cargo.toml"
    In the `Cargo.toml` for your wrapped code you need to specify the core rust code as a path dependency:

    ```toml
    ...
    [dependencies]
      fizzbuzz = { path = "../fizzbuzz" }
    ...
    ```

    Full details on `Cargo.toml` in [the Cargo book](https://doc.rust-lang.org/cargo/reference/manifest.html)

!!! warning "Avoiding namespace, plug-in and publishing issues"
    1. Point setuptools explicitly to look in `python` - this helps avoid implicit/explicit namespace errors later
    1. Use a cargo workspace to avoid occasional strange errors from rust-analyzer in your IDE and to make running tests, lints etc. easier from the root directory
    1. Use a path dependency to your core code in the wrapped-code `Cargo.toml` so you are always using the latest changes. Without a version tag, any attempts to upload the wrapped code as a crate to fail - but you don't want to do that anyway and this is another safety measure to make sure you never do.
    1. You don't need anything special in your core-code `Cargo.toml` but don't forget to add one!

!!! abstract "Overview of all config files"
    The root has a `pyproject.toml` and `Cargo.toml`, each folder under `rust` also has a `Cargo.toml`

    ```text
    FizzBuzz
    - rust
      - fizzbuzz
        - src
          ... core rust code goes here
        - tests
          ... tests for core rust code go here
        - Cargo.toml
      - fizzbuzzo3
        - src
          ... pyo3 code goes here
        - Cargo.toml
    - python
      - fizzbuzz
        ... additional python code goes here
    - tests
      ... python tests go here
    - Cargo.toml
    - pyproject.toml
    ```

    ??? python "**`./pyproject.toml`** - full source"
        ```toml
        --8<-- "pyproject.toml"
        ```

    ??? rust "**`./Cargo.toml`** - full source"
        ```toml
        --8<-- "Cargo.toml"
        ```

    ??? rust "**`rust/fizzbuzz/Cargo.toml`** - full source"
        ```toml
        --8<-- "rust/fizzbuzz/Cargo.toml"
        ```

    ??? rust "**`rust/fizzbuzzo3/Cargo.toml`** - full source"
        ```toml
        --8<-- "rust/fizzbuzzo3/Cargo.toml"
        ```
