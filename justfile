# list available recipes
list:
  @just --list --justfile {{justfile()}}
  
# remove pre-built rust and python libraries (excluding .venv)
clean:
    - cargo clean
    find . -type f -name "*.profraw" -delete
    rm -rf .pytest_cache
    find . -depth -type d -not -path "./.venv/*" -name "__pycache__" -exec rm -rf "{}" \;
    find . -depth -type d -path "*.egg-info" -exec rm -rf "{}" \;
    find . -type f -name "*.egg" -delete
    find . -type f -name "*.so" -delete


# clean, remove existing .venv and rebuild the venv with pip install -e .[dev]
reset: clean
    rm -rf .venv
    python -m venv .venv
    .venv/bin/python -m pip install --upgrade pip 
    .venv/bin/pip install -e .[dev]

# lint rust files with fmt & clippy
lint-rust:
  - cargo fmt --check
  - cargo clippy --workspace

# test rust workspace
test-rust:
  - cargo test --quiet --workspace

# lint and test rust
check-rust: lint-rust test-rust

# lint python with ruff
lint-python:
  - .venv/bin/ruff check .

# test python
test-python:
  - .venv/bin/pytest

# lint and test python
check-python: lint-python test-python

# lint and test both rust and python
check: check-rust check-python

# build and test a wheel (a suitable venv must already by active!)
test-wheel: clean
  cibuildwheel --only cp312-manylinux_x86_64

cov:
  RUSTFLAGS="-Cinstrument-coverage" cargo build
  RUSTFLAGS="-Cinstrument-coverage" LLVM_PROFILE_FILE="tests-%p-%m.profraw" cargo test
  grcov . -s . --binary-path ./target/debug/ -t html,lcov --branch --ignore-not-existing --output-path ./rustcov
  pytest --cov --cov-report html:pycov --cov-report term

rust-cov:
  python -m http.server -d ./rustcov/html

python-cov:
  python -m http.server -d ./pycov
