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
//! use fizzbuzz::FizzBuzz;
//!
//! assert_eq!(1.fizzbuzz(), "1".to_string());
//! assert_eq!(3.fizzbuzz(), "fizz".to_string());
//! ```
//!
//! ## The experience:
//!
//! Rust is an interesting language to work with and I'd probably try choosing it in preference to C
//! for work where I need a super-fast, compiled language with the abilty to really tweek the system.
//! Rust is highly opinionated (not a bad thing) and the compiler is a real pain forcing you into perfect
//! form - yes that's a good thing, "the book" keeps telling you to be grateful for this! (and always ends
//! with an exclamation point!)
//!
//! ### Pros
//! I like the focus on raising all the little bugs the C users may find at some point in runtime and
//! confronting the developer with them upfront.
//!
//! ### Cons
//! For a language with such a strong opinion and focus on maintaining good form, type safety etc. the
//! testing support and explanation is terrible. Testing is first covered in Chapter 11 of "the book", far too late.
//!  I was very disappointed with the poor (and late) integration of tests into VSCode's testing API. The discussions
//! in the language development community regarding how, and whether, to integrate were saddening. Additionally the
//! explanation of unit vs integration tests in "the book" are simply not quite right and a real chance was missed here
//! to create a language that not only values good coding form, but also good testing form. Add to that the reliance on
//! convention forcing a specific folder structure if you want tests to work, not only do tests have to go in the `tests`
//! directory and helper functions have to go in a subdirectory; there is no support for fixtures, parameterisation etc.
//! in the built in framework and the VSCode integration won't work if you don't have your source directly in the `src`
//! directory. In short: very disappointing.
//!
//! > In case you're wondering how I would have described testing, here it is in a nutshell:
//! >
//! > 1. Well tested code should have tests which are complete *and* decoupled from the specific implementation
//! > of what they are testing. Test the contracts your interfaces make with the rest of the world.
//! >
//! > 1. Place tests that test the internal interface one part of your code offers to another part of your code
//! > in the `src` folder in the same file as the code you are testing. Aim to decouple this from the specific
//! > implementation of that code - your test shouldn't change when your code under test does; if your test has
//! > to change, then something elsewhere in your code should have to change too.
//! >
//! > 1. Place tests that test the interface you are offering to the users of our library / crate in the `tests`
//! > directory. Again, these tests should be independent of the implementation - if you change them then users
//! > of your library / crate will have to change something as well.
//!
//! ## fizzbuzz rules
//!
//! Fizzbuzz is a child's counting game where instead of saying a number you must say:
//! "fizz" if the number is divisible by 3 and "buzz" if divisible by 5 ("fizzbuzz" for
//! those numbers divsible by both 3 and 5)

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
