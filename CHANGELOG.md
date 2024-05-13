# FizzBuzz Changelog

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
