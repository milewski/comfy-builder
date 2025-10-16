use proc_macro::TokenStream;
use proc_macro2::Ident;
use quote::quote;
use syn::{LitStr, Token, parse_macro_input};

#[derive(Debug)]
struct BootstrapArgs {
    api_version: String,
}

impl syn::parse::Parse for BootstrapArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;

        if ident != "api_version" {
            return Err(syn::Error::new_spanned(ident, "expected `api_version` keyword"));
        }

        input.parse::<Token![:]>()?;

        Ok(BootstrapArgs {
            api_version: input.parse::<LitStr>()?.value(),
        })
    }
}

pub fn boostrap(input: TokenStream) -> TokenStream {
    let crate_name = std::env::var("CARGO_PKG_NAME")
        .expect("Failed to determine crate name. Please ensure you're building this package with Cargo and that the CARGO_PKG_NAME environment variable is set correctly.");

    let safe_crate_name = crate_name.replace("-", "_");
    let module_name_ident = Ident::new(&safe_crate_name, proc_macro2::Span::call_site());

    let arguments = parse_macro_input!(input as BootstrapArgs);
    let api_version = arguments.api_version;

    TokenStream::from(quote! {
        use pyo3::types::*;

        pub mod __injected {
            pub static API_VERSION: &'static str = #api_version;
            pub static MODULE_NAME: &'static str = stringify!(#module_name_ident);
        }

        #[pyo3::pyfunction]
        async fn get_node_list() -> pyo3::PyResult<pyo3::Py<pyo3::types::PyList>> {
            pyo3::Python::attach(|python| {
                let comfy_node = python
                    .import(format!("comfy_api.{}", #api_version))?
                    .getattr("io")?
                    .getattr("ComfyNode")?;

                let nodes = pyo3::types::PyList::empty(python);
                let builtins = python.import("builtins")?;
                let type_fn = builtins.getattr("type")?;
                let decorator = builtins.getattr("classmethod")?;

                for registration in inventory::iter::<comfy_builder_core::registry::NodeRegistration>() {
                    nodes.append(
                        registration.create_node(python, &decorator, &type_fn, &comfy_node, stringify!(#module_name_ident))?
                    )?;
                }

                Ok(nodes.unbind())
            })
        }

        #[pyo3::pyfunction]
        #[pyo3(pass_module)]
        fn comfy_entrypoint<'py>(module: &pyo3::Bound<'py, pyo3::prelude::PyModule>) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::PyAny>> {
            let python = module.py();
            let base = python
                .import(format!("comfy_api.{}", #api_version))?
                .getattr("ComfyExtension")?;

            let methods = pyo3::types::PyDict::new(python);

            methods.set_item("get_node_list", pyo3::wrap_pyfunction!(get_node_list, python)?)?;

            let builtins = python.import("builtins")?;
            let type_fn = builtins.getattr("type")?;

            let extension = type_fn.call1((format!("{}_extension", stringify!(#module_name_ident)), (base,), methods))?;

            extension.call0()
        }

        #[pyo3::pymodule]
        fn #module_name_ident<'py>(
            python: pyo3::Python<'py>,
            module: pyo3::Bound<'py, pyo3::prelude::PyModule>,
        ) -> pyo3::PyResult<()> {
            module.add_function(pyo3::wrap_pyfunction!(comfy_entrypoint, python)?)
        }

    })
}
