//! A Trait implementation of "fizzbuzz" providing a default implementation for all types which can be
//! converted from u8 (everything *except* `i8`)
//!
//!
//! ## Example usage:
//!
//! ```
//! use fizzbuzz::FizzBuzz;
//!
//! assert_eq!(1.fizzbuzz(), "1".to_string());
//! assert_eq!(3.fizzbuzz(), "fizz".to_string());
//! ```

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
    ///
    /// A default implementation is available for any type `<Num>` which supports
    /// `<Num>::from(<u8>)`, `std::fmt::Display`, `PartialEq` and `<&Num> % <Num>`
    fn fizzbuzz(&self) -> String;
}

/// Implements the FizzBuzz trait for any type `<T>` which supports `<T>::from(<u8>)`
/// and `<T> % <T>`
impl<Num> FizzBuzz for Num
where
    Num: TryFrom<u8> + std::fmt::Display + PartialEq,
    for<'a> &'a Num: std::ops::Rem<Num, Output = Num>,
{
    fn fizzbuzz(&self) -> String {
        let three = match <Num>::try_from(3_u8) {
            Ok(three) => three,
            Err(_) => return self.to_string(),
        };
        let five = match <Num>::try_from(5_u8) {
            Ok(five) => five,
            Err(_) => return self.to_string(),
        };
        let zero = match <Num>::try_from(0_u8) {
            Ok(zero) => zero,
            Err(_) => return self.to_string(),
        };
        match (self % three == zero, self % five == zero) {
            (true, true) => "fizzbuzz".to_string(),
            (true, false) => "fizz".to_string(),
            (false, true) => "buzz".to_string(),
            _ => self.to_string(),
        }
    }
}
