use fizzbuzz::FizzBuzz;
use googletest::prelude::*;

/// Create a test case which tests all numbers between 1 and 255 for a given type
///
/// Call with `test_this! {test_case_name: type, ...}`
macro_rules! test_this {
    ($($id:ident: $t:ty),*) => {
        $(
            #[googletest::test]
            fn $id() {
                let allnums: Vec<$t> = (1u8..=127).step_by(1).map(|i| {<$t>::try_from(i)}.unwrap()).collect();
                let fifteens: Vec<$t> = (0u8..=127).step_by(15).map(|i| {<$t>::try_from(i).unwrap()}).collect();
                let fives: Vec<$t> = (0u8..=127).step_by(5).map(|i| {<$t>::try_from(i).unwrap()}).collect();
                let threes: Vec<$t> = (0u8..=127).step_by(3).map(|i| {<$t>::try_from(i).unwrap()}).collect();

                for num in allnums {
                    let result = num.fizzbuzz();
                    if fifteens.contains(&num) {
                        expect_that!(result, eq("fizzbuzz"), "for {num}")
                    } else if fives.contains(&num) {
                        expect_that!(result, eq("buzz"), "for {num}")
                    } else if threes.contains(&num) {
                        expect_that!(result, eq("fizz"), "for {num}")
                    } else {
                        expect_that!(&result, eq(&num.to_string()), "for {num}")
                    }
                }
            }
        )*
    }
}

/// Test all compatible standard types
mod standard_types {
    use super::*;

    test_this! {
        test_f32: f32,
        test_f64: f64,
        test_i8: i8,
        test_i16: i16,
        test_i32: i32,
        test_i64: i64,
        test_i128: i128,
        test_isize: isize,
        test_u8: u8,
        test_u16: u16,
        test_u32: u32,
        test_u64: u64,
        test_u128: u128,
        test_usize: usize
    }

    #[test]
    fn test_negative() {
        assert_eq!((-1_i8).fizzbuzz(), "-1");
        assert_eq!((-3_i8).fizzbuzz(), "fizz");
        assert_eq!((-5_i8).fizzbuzz(), "buzz");
        assert_eq!((-15_i8).fizzbuzz(), "fizzbuzz");
    }

    #[test]
    fn test_not_whole_number() {
        assert_eq!(3.2_f32.fizzbuzz(), "3.2");
    }
}

/// Create a custom type based on i16, add the minimum set of non-derivable
/// traits, impl_fizzbuzz! and test ...
mod custom_types {
    use std::{fmt::Display, ops::Rem};

    use super::*;

    #[derive(PartialEq)]
    struct Myint(i16);

    impl Rem<Myint> for &Myint {
        type Output = Myint;
        fn rem(self, rhs: Myint) -> Self::Output {
            Myint(self.0.rem(rhs.0))
        }
    }
    impl Display for Myint {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }
    impl From<u8> for Myint {
        fn from(value: u8) -> Self {
            Self(<i16>::from(value))
        }
    }

    test_this! {
        my_int16: Myint
    }
}
