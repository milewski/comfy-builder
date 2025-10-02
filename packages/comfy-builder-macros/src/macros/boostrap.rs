use proc_macro::TokenStream;
use quote::quote;

pub fn boostrap(_: TokenStream) -> TokenStream {
    let crate_name = std::env::var("CARGO_PKG_NAME")
        .expect("Failed to determine crate name. Please ensure you're building this package with Cargo and that the CARGO_PKG_NAME environment variable is set correctly.");

    let safe_crate_name = crate_name.replace("-", "_");
    let module_name_ident = syn::Ident::new(&safe_crate_name, proc_macro2::Span::call_site());

    let expanded = quote! {
        use pyo3::types::PyModuleMethods;

        #[pyo3::pymodule]
        fn #module_name_ident(python: pyo3::Python, module: &pyo3::Bound<'_, pyo3::prelude::PyModule>) -> pyo3::PyResult<()> {
            let node_class_mappings = pyo3::types::PyDict::new(python);
            let node_display_name_mappings = pyo3::types::PyDict::new(python);

            for registration in inventory::iter::<comfy_builder_core::registry::NodeRegistration>() {
                registration.register(
                    python,
                    module,
                    &node_class_mappings,
                    &node_display_name_mappings,
                )?;
            }

            module.add("NODE_CLASS_MAPPINGS", node_class_mappings)?;
            module.add("NODE_DISPLAY_NAME_MAPPINGS", node_display_name_mappings)?;

            Ok(())
        }
    };

    TokenStream::from(expanded)
}
