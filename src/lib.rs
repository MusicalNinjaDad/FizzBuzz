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