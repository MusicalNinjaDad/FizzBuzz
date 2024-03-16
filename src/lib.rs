pub fn fizzbuzz(number: &usize) -> String {
    let response: String;
    if number % 15 == 0 {
        response = "fizzbuzz".to_string();
    } else if number % 3 == 0 {
        response = "fizz".to_string();
    } else if number % 5 == 0 {
        response = "buzz".to_string();
    } else {
        response = number.to_string();
    }
    response
}

pub trait FizzBuzz {
    fn fizzbuzz(&self) -> String;
}

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

#[cfg(test)]
mod dumbtests {
    use super::*;

    #[test]
    fn one() {
        let result = fizzbuzz(&1);
        assert_eq!(result, "1");
    }
    #[test]
    fn three() {
        let result = fizzbuzz(&3);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn six() {
        let result = fizzbuzz(&6);
        assert_eq!(result, "fizz");
    }

    #[test]
    fn five() {
        let result = fizzbuzz(&5);
        assert_eq!(result, "buzz");
    }
    #[test]
    fn fifteen() {
        let result = fizzbuzz(&15);
        assert_eq!(result, "fizzbuzz");
    }
}
#[cfg(test)]
mod itertests {
    use super::*;

    #[test]
    fn three_is_fizz_or_fizzbuzz() {
        for num in (0..=300).step_by(3) {
            let result = fizzbuzz(&num);
            assert!(result == "fizz" || result == "fizzbuzz");
        }
    }

    #[test]
    fn five_is_buzz_or_fizzbuzz() {
        for num in (0..=300).step_by(5) {
            let result = fizzbuzz(&num);
            assert!(result == "buzz" || result == "fizzbuzz");
        }
    }

    #[test]
    fn fifteen_is_fizzbuzz() {
        for num in (0..=300).step_by(15) {
            let result = fizzbuzz(&num);
            assert!(result == "fizzbuzz");
        }
    }

    #[test]
    fn allnumbers() {
        let allnums: Vec<usize> = (1..=300).collect();
        let fifteens: Vec<usize> = (0..=300).step_by(15).collect();
        let fives: Vec<usize> = (0..=300).step_by(5).collect();
        let threes: Vec<usize> = (0..=300).step_by(3).collect();

        for num in allnums {
            let result = fizzbuzz(&num);
            if fifteens.contains(&num) {
                assert_eq!(result, "fizzbuzz")
            } else if fives.contains(&num) {
                assert_eq!(result, "buzz")
            } else if threes.contains(&num) {
                assert_eq!(result, "fizz")
            } else {
                assert_eq!(&result, &num.to_string())
            }
        }
    }
}