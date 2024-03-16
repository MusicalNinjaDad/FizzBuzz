use fizzbuzz::FizzBuzz;
use googletest::*;
use googletest::prelude::*;

macro_rules! test_this {
    ($($id:ident: $t:ty),*) => {
        $(
            #[googletest::test]
            fn $id() {
                let allnums: Vec<$t> = (1u8..=255).step_by(1).map(|i| {<$t>::from(i)}).collect();
                let fifteens: Vec<$t> = (0u8..=255).step_by(15).map(|i| {<$t>::from(i)}).collect();
                let fives: Vec<$t> = (0u8..=255).step_by(5).map(|i| {<$t>::from(i)}).collect();
                let threes: Vec<$t> = (0u8..=255).step_by(3).map(|i| {<$t>::from(i)}).collect();

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


test_this! {
    test_f32: f32,
    test_f64: f64,
    test_i16: i16,
    test_i32: i32,
    test_i64: i64,
    test_i128: i128,
    test_u8: u8,
    test_u16: u16,
    test_u32: u32,
    test_u64: u64,
    test_u128: u128,
    test_usize: usize
}
