use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

fn importmodule(module: TokenStream2, input: TokenStream2) -> TokenStream2 {
    quote!(
        pyo3::append_to_inittab!(#module);
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

        let attr = quote!(
            py_fizzbuzzo3
        );

        let expected = quote!{
            pyo3::append_to_inittab!(py_fizzbuzzo3);
            pyo3::prepare_freethreaded_python();
            Python::with_gil(|py| {
                let fizzbuzzo3 = py
                .import_bound("fizzbuzzo3")
                .expect("Failed to import fizzbuzzo3");
                assert!(true);
            });
        };

        let output = importmodule(attr, input);

        assert_eq!(output.to_string(), expected.to_string());

    }
}