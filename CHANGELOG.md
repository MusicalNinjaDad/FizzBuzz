# FizzBuzz Changelog

## Python 2.1.0

- Allow a `slice` to be passed to `fizzbuzzo3.fizzbuzz` - this provides a further 1.5x speed increase over passing a list with the same elements.
- Build rust with `--release` optimisations - another 4-7x speed increase.

## Rust 3.0.1

- Additional test case validating use case for stepped range, no code changes required.

## Rust 3.0.0

- **BREAKING CHANGE**: MultiFizzBuzz will consume the input.
- MultiFizzBuzz can accept any type of input which will provide a `rayon::iter::IndexedParallelIterator` via `rayon::iter::IntoParallelIterator`. Specifically this is tested to also accept `Range<i32>` (but not `RangeInclusive<i32>` or Ranges of larger types).

## Python 2.0.1

- Update pyo3testing framework used for rust exports to v0.3.4 (Simplifies unit tests in rust source)

## Python 2.0.0

- Python and rust variants are now under single fizzbuzz namespace as `fizzbuzz.fizzbuzzpy` and `fizzbuzz.fizzbuzzo3`
- Add type and docstring hinting for fizzbuzzo3
- Add signature and docstring info to pyo3function in case needed via `fizzbuzz.__doc__` and `fizzbuzz.____text_signature__`
- Enable python doctests for both python and rust variants

## Python 1.3.1

- Clean before building each set of wheels to ensure that wheels compiled for non x86 linux architectures run correctly
- Add a justfile to make cleaning, linting, testing etc. easier

## Rust 2.1.0 & Python 1.3.0

- Process `Vec`s / `list`s with more than 300k elements in parallel

## Rust 2.0.0 & Python 1.2.0

### Rust

- `fizzbuzz()` can be called on a `Vec<Num>`
- Provide the return from `fizzbuzz()` as a `FizzBuzzAnswer` which supports `.into()` `String` and `Vec<String>`

### Python

- `fizzbuzzo3.fizzbuzz` can be passed a `list` of numbers, the return will be a `str` of all the answers

## Python 1.1.2 & Rust 1.0.1

- Test each wheel as part of build process
- Release from `main` branch

## Python 1.1.1

- Build for additional linux architectures (non-intel)

## Python 1.1.0

- Build wheels for most architectures (except: `musllinux-i686` and `windows-arm64`)
