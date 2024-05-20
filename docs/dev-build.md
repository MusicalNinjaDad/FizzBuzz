# Developing and Building

This section won't go into the actual coding of your core code in rust - see the excursions for that. Assuming that you have your core code written (in rust):

## Wrapping with pyo3

The [relevant section of the pyo3 book](https://pyo3.rs/latest/rust-from-python) does a great job of explaining how to wrap your code, so I'll just touch the highlights here:

!!! pyo3 "rust/fizzbuzzo3/Cargo.toml"
    1. Name your library the way you want the module to appear in python. For example to import using `from fizzbuzz import fizzbuzzo3`
    1. Use the `cdylib` library type
    1. Add a dependency to `pyo3`

    Add the following to **`./rust/fizzbuzzo3/Cargo.toml`**
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

!!! python "Adding the wrapped module to your project"
    I chose to use `setuptools` & `setuptools-rust` as my build backend. Pyo3 offer two backends [`setuptools-rust`](https://github.com/PyO3/setuptools-rust) and [`maturin`](https://github.com/PyO3/maturin). I prefered to try the first because:

    - I am already used to using `setuptools` and didn't want to change out a working system for something else
    - I found `setuptools-rust`to be very easy to use
    - [The docs](https://pyo3.rs/v0.21.2/building-and-distribution#packaging-tools) point out that it offers more flexibility and fits better to a use case where you may also have independent python code

    Add the following to **`./pyproject.toml`**
    ```toml
    ...
    [[tool.setuptools-rust.ext-modules]]
      # The last part of the name (e.g. "_lib") has to match lib.name in Cargo.toml,
      # but you can add a prefix to nest it inside of a Python package.
      target = "fizzbuzz.fizzbuzzo3"
      path = "rust/fizzbuzzo3/Cargo.toml"
      binding = "PyO3"
      features = ["pyo3/extension-module"] # IMPORTANT!!!
    ...
    ```

!!! warning "Avoid errors packaging for linux"
    It is important to specify `features = ["pyo3/extension-module"]` in `./pyproject.toml` to avoid linking the python interpreter into your library and failing quality checks when trying to package for linux.

    Background is available by combining the [pyo3 FAQ](https://pyo3.rs/latest/faq#i-cant-run-cargo-test-or-i-cant-build-in-a-cargo-workspace-im-having-linker-issues-like-symbol-not-found-or-undefined-reference-to-_pyexc_systemerror) and [manylinux specification](https://github.com/pypa/manylinux)

??? python "`./pyproject.toml` - full source"
    ```toml
    --8>-- "./pyproject.toml"
    ```

## Python virtual environment & build

I like to keep things as simple as possible. Python has many virtual environment managers, `venv` is part of the core library and does everything we need while leaving us in control of the entire build and integration process.

!!! abstract "Quick start with justfile"
    The justfile `./justfile` handles all of this for you. Feel free to copy it

??? abstract "`./justfile` - full source"
    ```justfile
    --8<-- "./justfile"
    ```

??? python "Creating a virtual environment with venv"
    If you are unfamiliar with venv here are the [docs](https://docs.python.org/3/library/venv.html)

    Depending on your distro you may need to install venv as a separate package

    Creating a virtual environment is as simple as:

    ```sh
    /projectroot$ python -m venv .venv
    ```

!!! python "Sourcing development dependencies from `./pyproject.toml`"
    To provide a single line python build for local development you will need to source your development dependencies from `./pyproject.toml`. These can be split into multiple groups to give more control during automated processes where you don't need everything.

    Add the following to **`./pyproject.toml`**
    ```toml
    ...
    [project.optional-dependencies]
      lint = ["ruff"]
      test = ["pytest", "pytest-doctest-mkdocstrings"]
      cov = ["fizzbuzz[test]", "pytest-cov"]
      doc = ["black", "mkdocs", "mkdocstrings[python]", "mkdocs-material"]
      dev = ["fizzbuzz[lint,test,cov,doc]"]
    ...
    ```

??? python "`./pyproject.toml` - full source"
    ```toml
    --8>-- "./pyproject.toml"
    ```

!!! python "Building and installing the wrapped rust code for use in python development"
    Before you can use the wrapped rust code you need to build the equivalent of `python/fizzbuzz/fizzbuzzo3.cpython-312-x86_64-linux-gnu.so`:

    1. Make sure your virtual environment is actived. If not run `. .venv/bin/activate` (note the leading dot, which is easier than typing `source` all the time)
    1. Then simply use `pip` to create an editable installation of your codebase:
        ```sh
        (.venv)/projectroot$ pip -e.[dev]
        ```

### cleaning

!!! abstract "Cleaning up old build artefacts"
    As with any built language it is a good idea to clean up old build artefacts before generating new ones, or at least before finalising a change. Cargo offers a simple `cargo clean` for this, but you will also have the python library and various python caches in place which can sometimes cause problems.

    To clean both languages:
    ```sh
    (.venv)/projectroot$ cargo clean || true
    (.venv)/projectroot$ rm -rf .pytest_cache
    (.venv)/projectroot$ rm -rf build
    (.venv)/projectroot$ rm -rf dist
    (.venv)/projectroot$ rm -rf wheelhouse
    (.venv)/projectroot$ rm -rf .ruff_cache
    (.venv)/projectroot$ find . -depth -type d -not -path "./.venv/*" -name "__pycache__" -exec rm -rf "{}" \;
    (.venv)/projectroot$ find . -depth -type d -path "*.egg-info" -exec rm -rf "{}" \;
    (.venv)/projectroot$ find . -type f -name "*.egg" -delete
    (.venv)/projectroot$ find . -type f -name "*.so" -delete
    ```

    **Or just use `just clean` from the `./justfile`**

??? abstract "`./justfile` - full source
    ```justfile
    --8<-- "./justfile"
    ```

## performance / API design
