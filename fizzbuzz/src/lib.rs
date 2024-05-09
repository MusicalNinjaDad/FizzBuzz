//! A Trait implementation of "fizzbuzz" providing a default implementation for all types which can be
//! constructed from `0`,`3`,`5` and support the `%` operator.
//!
//!
//! ## Example usage:
//!
//! ```
//! use fizzbuzz::FizzBuzz;
//!
//! assert_eq!(1.fizzbuzz(), fizzbuzz::FizzBuzzResult::String("1".to_string()));
//! assert_eq!(3.fizzbuzz(), fizzbuzz::FizzBuzzResult::String("fizz".to_string()));
//! ```

/// Used to obtain the correct fizzbuzz answer for a given number
///
/// ### Required:
/// - fn fizzbuzz() -> String

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FizzBuzzAnswer {
    String(String),
    Vec(Vec<String>)
}

impl Into<String> for FizzBuzzAnswer {
    fn into(self) -> String {
        match self {
            Self::String(s) => s,
            Self::Vec(_) => "Dummy String".to_string()
        }
    }
}

pub trait FizzBuzz {
    /// Accepts a number and returns a `String` containing:
    ///
    /// - `fizzbuzz` if the number is directly divisible by 5 *and* 3
    /// - `fizz` if the number is directly divisible by 3
    /// - `buzz` if the number is directly divisible by 5
    /// - the number in other cases
    ///
    /// A default implementation is available for any type `<Num>` which supports
    /// - `<Num>::try_from(<u8>)`: `Num` must be able to be constructed from `0`, `3` & `5`.
    /// - `std::fmt::Display`: Allows `Num` to be formatted as a `String`.
    /// - `PartialEq`: Enables comparison operations for `Num`.
    /// - `<&Num>::Rem<Num, Output = Num>`: Allows `&Num % Num`.
    fn fizzbuzz(&self) -> FizzBuzzAnswer;
}

/// Implements the FizzBuzz trait for any type `<T>` which supports `<T>::from(<u8>)`
/// and `<T> % <T>`
impl<Num> FizzBuzz for Num
where
    Num: TryFrom<u8> + std::fmt::Display + PartialEq,
    for<'a> &'a Num: std::ops::Rem<Num, Output = Num>,
{
    fn fizzbuzz(&self) -> FizzBuzzAnswer {
        let three = match <Num>::try_from(3_u8) {
            Ok(three) => three,
            Err(_) => return FizzBuzzAnswer::String(self.to_string()),
        };
        let five = match <Num>::try_from(5_u8) {
            Ok(five) => five,
            Err(_) => return FizzBuzzAnswer::String(self.to_string()),
        };
        let zero = match <Num>::try_from(0_u8) {
            Ok(zero) => zero,
            Err(_) => return FizzBuzzAnswer::String(self.to_string()),
        };
        match (self % three == zero, self % five == zero) {
            (true, true) => FizzBuzzAnswer::String("fizzbuzz".to_string()),
            (true, false) => FizzBuzzAnswer::String("fizz".to_string()),
            (false, true) => FizzBuzzAnswer::String("buzz".to_string()),
            _ => FizzBuzzAnswer::String(self.to_string()),
        }
    }
}
