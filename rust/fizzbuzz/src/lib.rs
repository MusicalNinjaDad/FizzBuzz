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

use std::borrow::Cow;

use rayon::prelude::*;
static BIG_VECTOR: usize = 300_000; // Size from which parallelisation makes sense

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Provides conversion to `String` and `Vec<String>` via `.into()`,
/// ::From() etc.
pub enum FizzBuzzAnswer {
    /// Stores a single FizzBuzz value
    One(Cow<'static, str>),
    /// Stores a series of FizzBuzz values
    Many(Vec<Cow<'static, str>>),
}

impl From<FizzBuzzAnswer> for String {
    fn from(value: FizzBuzzAnswer) -> Self {
        match value {
            FizzBuzzAnswer::One(s) => s.into(),
            FizzBuzzAnswer::Many(v) => v.join(", "),
        }
    }
}

impl From<FizzBuzzAnswer> for Cow<'static, str> {
    fn from(value: FizzBuzzAnswer) -> Self {
        match value {
            FizzBuzzAnswer::One(s) => s,
            FizzBuzzAnswer::Many(v) => v.join(", ").into(),
        }
    }
}

impl From<FizzBuzzAnswer> for Vec<Cow<'static, str>>
{
    fn from(value: FizzBuzzAnswer) -> Self {
        match value {
            FizzBuzzAnswer::One(s) => vec![s],
            FizzBuzzAnswer::Many(v) => v,
        }
    }
}

/// Used to obtain the correct fizzbuzz answer for a given number
///
/// ### Required:
/// - fn fizzbuzz() -> String
pub trait FizzBuzz {
    /// Computes the FizzBuzz value for the implementing type.
    /// Returns a `FizzBuzzAnswer` which provides a structured representation
    /// of the FizzBuzz result.
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
            Err(_) => return FizzBuzzAnswer::One(self.to_string().into()),
        };
        let five = match <Num>::try_from(5_u8) {
            Ok(five) => five,
            Err(_) => return FizzBuzzAnswer::One(self.to_string().into()),
        };
        let zero = match <Num>::try_from(0_u8) {
            Ok(zero) => zero,
            Err(_) => return FizzBuzzAnswer::One(self.to_string().into()),
        };
        match (self % three == zero, self % five == zero) {
            (true, true) => FizzBuzzAnswer::One("fizzbuzz".into()),
            (true, false) => FizzBuzzAnswer::One("fizz".into()),
            (false, true) => FizzBuzzAnswer::One("buzz".into()),
            _ => FizzBuzzAnswer::One(self.to_string().into()),
        }
    }
}

/// Used to obtain the correct `FizzBuzzAnswer` for a `Vec` of numbers
///
/// ### Required:
/// - fn fizzbuzz(self) -> FizzBuzzAnswer
pub trait MultiFizzBuzz {
    fn fizzbuzz(self) -> FizzBuzzAnswer;
}

impl<Iterable, Num> MultiFizzBuzz for Iterable
where
    Iterable: rayon::iter::IntoParallelIterator<Item = Num>,
    <Iterable as IntoParallelIterator>::Iter: IndexedParallelIterator,
    Num: FizzBuzz,
{
    fn fizzbuzz(self) -> FizzBuzzAnswer {
        let par_iter = self.into_par_iter();
        if par_iter.len() < BIG_VECTOR {
            FizzBuzzAnswer::Many(
                par_iter
                    .with_min_len(BIG_VECTOR) //Don't parallelise when small
                    .map(|n| n.fizzbuzz().into())
                    .collect(),
            )
        } else {
            FizzBuzzAnswer::Many(par_iter.map(|n| n.fizzbuzz().into()).collect())
        }
    }
}

#[cfg(testx)]
mod test {
    use super::*;

    #[test]
    fn vec_to_string() {
        let input = FizzBuzzAnswer::Many(vec![
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
    fn big_vector_is_well_ordered() {
        let input: Vec<_> = (1..BIG_VECTOR + 2).collect();
        let output: Vec<_> = input.clone().fizzbuzz().into();
        let mut expected: Vec<String> = vec![];
        for i in input.iter() {
            expected.push(i.fizzbuzz().into())
        }
        assert_eq!(output, expected);
    }

    #[test]
    fn fizzbuzz_range() {
        let input = 1..20;
        let mut expected: Vec<String> = vec![];
        for i in 1..20 {
            expected.push(i.fizzbuzz().into())
        }
        let output: Vec<String> = input.fizzbuzz().into();
        assert_eq!(output, expected)
    }
}
