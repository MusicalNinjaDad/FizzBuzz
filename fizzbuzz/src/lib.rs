//! A Trait implementation of "fizzbuzz" providing a default implementation for all types which can be
//! constructed from `0`,`3`,`5` and support the `%` operator.
//!
//!
//! ## Example usage:
//!
//! ```
//! use fizzbuzz::FizzBuzz;
//!
//! let one: String = 1.fizzbuzz().into();
//! let three: String = 3.fizzbuzz().into();
//! assert_eq!(one, "1".to_string());
//! assert_eq!(three, "fizz".to_string());
//! ```

/// Used to obtain the correct fizzbuzz answer for a given number
///
/// ### Required:
/// - fn fizzbuzz() -> String

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum FizzBuzzAnswer {
    String(String),
    Vec(Vec<String>),
}

impl Into<String> for FizzBuzzAnswer {
    fn into(self) -> String {
        match self {
            Self::String(s) => s,
            Self::Vec(v) => v.join(", "),
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

pub trait MultiFizzBuzz {
    fn fizzbuzz(&mut self) -> FizzBuzzAnswer;
}

impl<Num> MultiFizzBuzz for Vec<Num>
where
    Num: FizzBuzz,
{
    fn fizzbuzz(&mut self) -> FizzBuzzAnswer {
        FizzBuzzAnswer::Vec(self.iter().map(|n| n.fizzbuzz().into()).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn vec_to_string() {
        let input = FizzBuzzAnswer::Vec(vec![
            "1".to_string(),
            "2".to_string(),
            "fizz".to_string(),
            "4".to_string(),
            "buzz".to_string(),
        ]);
        let output: String = input.into();
        let expected = "1, 2, fizz, 4, buzz".to_string();
        assert_eq!(output, expected)
    }

    #[test]
    fn fizzbuzz_a_vec() {
        let mut input = vec![1, 2, 3, 4, 5];
        let answer = input.fizzbuzz();
        let expected = FizzBuzzAnswer::Vec(vec![
            "1".to_string(),
            "2".to_string(),
            "fizz".to_string(),
            "4".to_string(),
            "buzz".to_string(),
        ]);
        assert_eq!(answer, expected)
    }
}
