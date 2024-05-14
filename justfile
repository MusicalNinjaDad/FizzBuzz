# list available recipes
list:
  @just --list --justfile {{justfile()}}
  
# remove pre-built rust and python libraries (excluding .venv)
clean:
    cargo clean
    rm -rf .pytest_cache
    find . -depth -type d -not -path "./.venv/*" -name "__pycache__" -exec rm -rf "{}" \;
    find . -depth -type d -path "*.egg-info" -exec rm -rf "{}" \;
    find . -type f -name "*.egg" -delete
    find . -type f -name "*.so" -delete

# clean, remove existing .venv and rebuild the venv with pip install -e .[dev]
reset: clean
    rm -rf .venv
    python -m venv .venv
    . .venv/bin/activate
    pip install -e .[dev]
