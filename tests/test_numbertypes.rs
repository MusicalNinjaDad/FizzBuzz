use fizzbuzz::FizzBuzz;
use googletest::*;
use googletest::prelude::*;

macro_rules! test_this {
    ($($id:ident: $t:ty),*) => {
        $(
            #[googletest::test]
            fn $id() {
                let allnums: Vec<$t> = (1i16..=300).step_by(1).map(|i| {<$t>::from(i)}).collect();
                let fifteens: Vec<$t> = (0i16..=300).step_by(15).map(|i| {<$t>::from(i)}).collect();
                let fives: Vec<$t> = (0i16..=300).step_by(5).map(|i| {<$t>::from(i)}).collect();
                let threes: Vec<$t> = (0i16..=300).step_by(3).map(|i| {<$t>::from(i)}).collect();

                for num in allnums {
                    let result = num.fizzbuzz();
                    if fifteens.contains(&num) {
                        expect_that!(result, eq("fizzbuzz"))
                    } else if fives.contains(&num) {
                        expect_that!(result, eq("buzz"))
                    } else if threes.contains(&num) {
                        expect_that!(result, eq("fizz"))
                    } else {
                        expect_that!(&result, eq(&num.to_string()))
                    }
                }
            }
        )*
    }
}


test_this! {
    macro_f64: f64
}


#[googletest::test]
fn googletest_f64() {
    let allnums: Vec<f64> = (1i16..=300).step_by(1).map(|i| f64::from(i)).collect();
    let fifteens: Vec<f64> = (0i16..=300).step_by(15).map(|i| f64::from(i)).collect();
    let fives: Vec<f64> = (0i16..=300).step_by(5).map(|i| f64::from(i)).collect();
    let threes: Vec<f64> = (0i16..=300).step_by(3).map(|i| f64::from(i)).collect();

    for num in allnums {
        let result = num.fizzbuzz();
        if fifteens.contains(&num) {
            expect_that!(result, eq("fizzbuzz"))
        } else if fives.contains(&num) {
            expect_that!(result, eq("buzz"))
        } else if threes.contains(&num) {
            expect_that!(result, eq("fizz"))
        } else {
            expect_that!(&result, eq(&num.to_string()))
        }
    }
}

#[googletest::test]
fn googletest_f32() {
    let allnums: Vec<f32> = (1i16..=300).step_by(1).map(|i| f32::from(i)).collect();
    let fifteens: Vec<f32> = (0i16..=300).step_by(15).map(|i| f32::from(i)).collect();
    let fives: Vec<f32> = (0i16..=300).step_by(5).map(|i| f32::from(i)).collect();
    let threes: Vec<f32> = (0i16..=300).step_by(3).map(|i| f32::from(i)).collect();

    for num in allnums {
        let result = num.fizzbuzz();
        if fifteens.contains(&num) {
            expect_that!(result, eq("fizzbuzz"))
        } else if fives.contains(&num) {
            expect_that!(result, eq("buzz"))
        } else if threes.contains(&num) {
            expect_that!(result, eq("fizz"))
        } else {
            expect_that!(&result, eq(&num.to_string()))
        }
    }
}