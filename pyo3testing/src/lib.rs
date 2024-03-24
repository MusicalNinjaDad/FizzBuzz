use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

fn importmodule(input: TokenStream2) -> TokenStream2 {
    quote!(
        pyo3::append_to_inittab!(py_fizzbuzzo3);
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            #input
        });
    )
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
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| {
                assert!(true);
            });
        };

        let output = importmodule(input);

        assert_eq!(output.to_string(), expected.to_string());

    }
}