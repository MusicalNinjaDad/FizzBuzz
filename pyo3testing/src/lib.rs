use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

fn importmodule(input: TokenStream2) -> TokenStream2 {
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_importmodule() {
        let input = quote!{
            assert!(true);
        };

        let expected = quote!{
            pyo3::append_to_inittab!(py_fizzbuzzo3);
            assert!(true);
        };

        let output = importmodule(input);

        assert_eq!(output.to_string(), expected.to_string());

    }
}