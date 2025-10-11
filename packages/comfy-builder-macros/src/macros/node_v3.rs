use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn node(_: TokenStream, input: TokenStream) -> TokenStream {
    let input_struct = parse_macro_input!(input as DeriveInput);
    let ident = &input_struct.ident;

    TokenStream::from(quote! {
        use pyo3::prelude::*;
        use comfy_builder_core::node::OutputPort;
        use pyo3::IntoPyObjectExt;
        use comfy_builder_core::{In, Out};

        #[derive(std::default::Default)]
        #input_struct

        inventory::submit! {
            comfy_builder_core::registry::NodeRegistration::new::<#ident>()
        }

        #[pyo3::pyfunction]
        #[pyo3(signature = (class, **kwargs))]
        fn __execute<'a>(
            class: pyo3::Bound<'a, pyo3::types::PyType>,
            kwargs: Option<pyo3::Bound<'a, pyo3::types::PyDict>>,
        ) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::PyAny>> {
            let instance = #ident::new();
            let output = instance.execute(instance.initialize_inputs(kwargs.into()));

            let python = class.py();
            let node_output = python
                .import("comfy_api.latest")?
                .getattr("io")?
                .getattr("NodeOutput")?;

            println!("execute function called... {:?}", class);

            let kwargs = pyo3::types::PyDict::new(python);
            kwargs.set_item("node_id", "Example")?;

            node_output.call1(output.to_schema(python)?)
        }

        #[pyfunction]
        fn __define_schema<'a>(class: pyo3::Bound<'a, pyo3::types::PyType>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::PyAny>> {
            let python = class.py();
            let io = python.import("comfy_api.latest")?.getattr("io")?;
            let inputs = <#ident as comfy_builder_core::prelude::Node>::In::blueprints(python)?;
            let outputs = <#ident as comfy_builder_core::prelude::Node>::Out::blueprints(python)?;

            let kwargs = pyo3::types::PyDict::new(python);

            kwargs.set_item("node_id", "Example")?;
            kwargs.set_item("display_name", "Example Node")?;
            kwargs.set_item("category", "examples")?;
            kwargs.set_item("description", "Node description here")?;
            kwargs.set_item("inputs", inputs)?;
            kwargs.set_item("outputs", outputs)?;

            io.getattr("Schema")?.call((), Some(&kwargs))
        }

        impl comfy_builder_core::ExtractNodeFunctions for #ident {

            fn define_function(python: pyo3::Python) -> pyo3::PyResult<pyo3::Bound<pyo3::types::PyCFunction>> {
                wrap_pyfunction!(__define_schema, python)
            }

            fn run_function(python: pyo3::Python) -> pyo3::PyResult<pyo3::Bound<pyo3::types::PyCFunction>> {
                wrap_pyfunction!(__execute, python)
            }

        }

    })
}
