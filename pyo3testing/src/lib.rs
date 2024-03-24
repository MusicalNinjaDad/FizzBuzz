use quote::quote;
use proc_macro2::{TokenStream as TokenStream2, Span};
use syn::Ident;

fn importmodule(module: Pyo3Import, input: TokenStream2) -> TokenStream2 {
    
    let moduleident = module.identifier;
    let pymoduleident = Ident::new(&module.name, Span::mixed_site());
    let modulename = module.name;
    let modulerror = "Failed to import ".to_string() + &modulename;

    quote!(
        pyo3::append_to_inittab!(#moduleident);
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let #pymoduleident = py
            .import_bound(#modulename)
            .expect(#modulerror);
            #input
        });
    )
}

struct Pyo3Import {
    identifier: Ident,
    name: String
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_importmodule() {
        let input = quote!{
            assert!(true);
        };

        let py_fizzbuzzo3 = Ident::new("py_fizzbuzzo3", Span::call_site());

        let module = Pyo3Import {
            identifier: py_fizzbuzzo3,
            name: "fizzbuzzo3".to_string()
        };

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

        let output = importmodule(module, input);

        assert_eq!(output.to_string(), expected.to_string());

    }
}