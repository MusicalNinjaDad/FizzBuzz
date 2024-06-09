//! A Trait implementation of "fizzbuzz" providing a default implementation for all types which can be
//! constructed from `0`,`3`,`5` and support the `%` operator.
//!
//!
//! ## Example usage for single item:
//!
//! ```
//! use fizzbuzz::FizzBuzz;
//! use std::borrow::Cow;
//!
//! let one: Cow<str> = 1.fizzbuzz().into();
//! let three: String = 3.fizzbuzz().into();
//! assert_eq!(&one, "1");
//! assert_eq!(three, "fizz");
//! ```
//!
//! ## Example usage for multiple items:
//!
//! ```
//! use fizzbuzz::MultiFizzBuzz;
//! use rayon::iter::ParallelIterator; // required to `.collect()` the results
//!
//! let one_to_five = vec![1,2,3,4,5];
//! let fizzbuzzed: Vec<String> = one_to_five.fizzbuzz().collect();
//! assert_eq!(fizzbuzzed, vec!["1", "2", "fizz", "4", "buzz"]);
//! ```

use std::borrow::Cow;

use rayon::prelude::*;
static BIG_VECTOR: usize = 300_000; // Size from which parallelisation makes sense

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
/// Represents a valid answer to fizzbuzz and provides conversion to `String` and `Cow<&str>` via `.into()`
pub enum FizzBuzzAnswer {
    Fizz,
    Buzz,
    Fizzbuzz,
    Number(String),
}

impl From<FizzBuzzAnswer> for Cow<'static, str> {
    fn from(answer: FizzBuzzAnswer) -> Self {
        match answer {
            FizzBuzzAnswer::Fizz => "fizz".into(),
            FizzBuzzAnswer::Buzz => "buzz".into(),
            FizzBuzzAnswer::Fizzbuzz => "fizzbuzz".into(),
            FizzBuzzAnswer::Number(n) => n.into(),
        }
    }
}

impl From<FizzBuzzAnswer> for String {
    fn from(answer: FizzBuzzAnswer) -> Self {
        match answer {
            FizzBuzzAnswer::Fizz => "fizz".into(),
            FizzBuzzAnswer::Buzz => "buzz".into(),
            FizzBuzzAnswer::Fizzbuzz => "fizzbuzz".into(),
            FizzBuzzAnswer::Number(n) => n,
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
            Err(_) => return FizzBuzzAnswer::Number(self.to_string()),
        };
        let five = match <Num>::try_from(5_u8) {
            Ok(five) => five,
            Err(_) => return FizzBuzzAnswer::Number(self.to_string()),
        };
        let zero = match <Num>::try_from(0_u8) {
            Ok(zero) => zero,
            Err(_) => return FizzBuzzAnswer::Number(self.to_string()),
        };
        match (self % three == zero, self % five == zero) {
            (true, true) => FizzBuzzAnswer::Fizzbuzz,
            (true, false) => FizzBuzzAnswer::Fizz,
            (false, true) => FizzBuzzAnswer::Buzz,
            _ => FizzBuzzAnswer::Number(self.to_string()),
        }
    }
}

/// Used to obtain the correct `FizzBuzzAnswer` for a multiple fizzbuzz-able numbers
pub trait MultiFizzBuzz {
    /// Returns an iterator which provides the FizzBuzz values for the elements of the implementing type.
    ///
    /// Note:
    /// - This function **consumes** the input
    /// - The returned iterator is a `rayon::iter::IndexedParallelIterator`
    /// - The Items in the returned iterator will be converted to a requested type
    /// (e.g. `FizzBuzzAnswer`, `String`, `Cow<str>`)
    fn fizzbuzz<Rtn>(self) -> impl IndexedParallelIterator<Item = Rtn>
    where
        Rtn: From<FizzBuzzAnswer> + Send;
}

/// Implements the MultiFizzBuzz trait for any type which can be easily converted into a
/// `rayon::iter::IndexedParallelIterator` over Items which implement `fizzbuzz::FizzBuzz`
///
/// Note:
/// - The returned iterator is _lazy_ - no calculations are performed until you use it
/// - Collecting this iterator requires that `rayon::iter::ParallelIterator` is in scope
/// - This implementation will decide whether it is worth the overhead of spawning multiple parallel threads
impl<Iterable, Num> MultiFizzBuzz for Iterable
where
    Iterable: rayon::iter::IntoParallelIterator<Item = Num>,
    <Iterable as IntoParallelIterator>::Iter: IndexedParallelIterator,
    Num: FizzBuzz,
{
    fn fizzbuzz<Rtn>(self) -> impl IndexedParallelIterator<Item = Rtn>
    where
        Rtn: From<FizzBuzzAnswer> + Send,
    {
        let par_iter = self.into_par_iter();
        let min_len = if par_iter.len() < BIG_VECTOR {
            BIG_VECTOR //Don't parallelise when small
        } else {
            1
        };
        par_iter.with_min_len(min_len).map(|n| n.fizzbuzz().into())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn big_vector_is_well_ordered() {
        let input: Vec<_> = (1..BIG_VECTOR + 2).collect();
        let output: Vec<FizzBuzzAnswer> = input.clone().fizzbuzz().collect();
        let mut expected: Vec<FizzBuzzAnswer> = vec![];
        for i in input.iter() {
            expected.push(i.fizzbuzz())
        }
        assert_eq!(output, expected);
    }

    #[test]
    fn fizzbuzz_range() {
        let input = 1..20;
        let mut expected: Vec<FizzBuzzAnswer> = vec![];
        for i in 1..20 {
            expected.push(i.fizzbuzz().into())
        }
        let output: Vec<FizzBuzzAnswer> = input.fizzbuzz().collect();
        assert_eq!(output, expected)
    }
}
