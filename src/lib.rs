//! A Trait implementation of "fizzbuzz" providing implemented Traits on all standard number
//! types *except* `i8` and a macro to implement the Trait on custom types.
//!
//! Looking at rust for the first time a couple of things stood out to me that were applicable in the
//! context of fizzbuzz:
//!
//! - firstly; the segregation of traditional classes into their data structures and method
//! implementations. This made a `FizzBuzz` `Trait` feel like the right approach.
//! - secondly; the languages love of exclamation marks (`!`), which make a regular appearance in
//! "the book" and require you to be sure in your belief that `macro_rules!` ;) Exporting the macro
//! seemed like the friendly thing to do for anyone who happens to have their own `struct` which could
//! be made more valuable by it's ability to fizzbuzz!
//!
//! ## Example usage:
//!
//! ```
//! use fizzbuzzo3::FizzBuzz;
//!
//! assert_eq!(1.fizzbuzz(), "1".to_string());
//! assert_eq!(3.fizzbuzz(), "fizz".to_string());
//! ```

use pyo3::prelude::*;

/// Used to obtain the correct fizzbuzz answer for a given number
///
/// ### Required:
/// - fn fizzbuzz() -> String
pub trait FizzBuzz {
    /// Required function which will return a `String` containing:
    ///
    /// - `fizzbuzz` if the number is directly divisible by 5 and 3
    /// - `fizz` if the number is directly divisible by 3
    /// - `buzz` if the number is directly divisible by 5
    /// - the number in other cases
    fn fizzbuzz(&self) -> String;
}

/// Implements the FizzBuzz trait for any type `<T>` which supports `<T>::from(<u8>)`
/// and `<T> % <T>`
#[macro_export]
macro_rules! impl_fizzbuzz {
    ( $( $t:ty), *) => { // Any number of types, optionally separated by commas
        $(
            impl FizzBuzz for $t {
                fn fizzbuzz(&self) -> String {
                    match (
                        self % <$t>::from(3u8) == <$t>::from(0u8),
                        self % <$t>::from(5u8) == <$t>::from(0u8)
                    ) {
                        (true, true)    => "fizzbuzz".to_string(),
                        (true, false)   => "fizz".to_string(),
                        (false, true)   => "buzz".to_string(),
                        _               => self.to_string()
                    }
                }
            }
        )*
    };
}

impl_fizzbuzz!(f32, f64, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);

#[pyfunction]
#[pyo3(name = "fizzbuzz")]
fn py_fizzbuzz(num: i32) -> String {
    num.fizzbuzz()
}

#[pymodule]
#[pyo3(name = "fizzbuzzo3")]
fn py_fizzbuzzo3(_py: Python, module: &PyModule) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(py_fizzbuzz, module)?)?;
    Ok(())
}