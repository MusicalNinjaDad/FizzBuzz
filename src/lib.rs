//! A Trait implementation of "fizzbuzz" providing implemented Traits on all standard number
//! types *except* `i8` and a macro to implement the Trait on custom types
//! 
//! ## Example usage:
//! 
//! ```
//! use fizzbuzz::FizzBuzz;
//! 
//! assert_eq!(1.fizzbuzz(), "1".to_string());
//! assert_eq!(3.fizzbuzz(), "fizz".to_string());
//! ```
//! 
//! ## fizzbuzz rules
//! 
//! Fizzbuzz is a child's counting game where instead of saying a number you must say:
//! "fizz" if the number is divisible by 3 and "buzz" if divisible by 5 ("fizzbuzz" for
//! those numbers divsible by both 3 and 5)

/// Used to obtain the correct fizzbuzz answer for a given number
pub trait FizzBuzz {
    /// Required function which will return a `String` containing:
    /// 
    /// - `fizzbuzz` if the number is directly divisible by 5 and 3
    /// - `fizz` if the number is directly divisible by 3
    /// - `buzz` if the number is directly divisible by 5
    /// - the number in other cases
    fn fizzbuzz(&self) -> String;
}

/// Implements the FizzBuzz trait for any type `<T>` which supports `<T>::from(<u8>)
/// and `<T> % <T>`
#[macro_export]
macro_rules! impl_fizzbuzz {
    ( $( $t:ty), *) => { // Any number of types, optionally separated by commas
        $(
            impl FizzBuzz for $t {
                fn fizzbuzz(&self) -> String {
                    let response: String;
                    if self % <$t>::from(15u8) == <$t>::from(0u8) {
                        response = "fizzbuzz".to_string();
                    } else if self % <$t>::from(3u8) == <$t>::from(0u8) {
                        response = "fizz".to_string();
                    } else if self % <$t>::from(5u8) == <$t>::from(0u8) {
                        response = "buzz".to_string();
                    }
                    else {
                        response = self.to_string();
                    }
                    response
                }
            }
        )*
    };
}

impl_fizzbuzz!(f32, f64, i16, i32, i64, i128, u8, u16, u32, u64, u128, usize);