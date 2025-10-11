use comfy_builder_core::prelude::{In, Node, Out};
use comfy_builder_core::{Int, IntoPyDict};
use pyo3::prelude::{PyAnyMethods, PyListMethods, PyModule, PyModuleMethods};
use pyo3::types::{PyDict, PyList};
use pyo3::{Bound, Py, PyAny, PyResult, Python, pyfunction, pymodule, wrap_pyfunction};

mod nodes;

#[pyfunction]
async fn get_node_list() -> PyResult<Py<PyList>> {
    Python::attach(|python| {
        let comfy_node = python
            .import("comfy_api.latest")?
            .getattr("io")?
            .getattr("ComfyNode")?;

        let nodes = PyList::empty(python);
        let builtins = python.import("builtins")?;
        let type_fn = builtins.getattr("type")?;
        let decorator = builtins.getattr("classmethod")?;
        
        println!("COUNT {:?}", inventory::iter::<comfy_builder_core::registry::NodeRegistration>().count());
        
        for registration in inventory::iter::<comfy_builder_core::registry::NodeRegistration>() {
            nodes.append(registration.register_v2(python, &decorator, &type_fn, &comfy_node)?)?;
        }

        // {
        //     #[pyfunction]
        //     fn define_schema<'a>(
        //         class: Bound<'a, pyo3::types::PyType>,
        //     ) -> PyResult<Bound<'a, PyAny>> {
        //         println!("defined schema function called...");
        //
        //         let python = class.py();
        //         let io = python.import("comfy_api.latest")?.getattr("io")?;
        //
        //         let inputs = <Example as comfy_builder_core::prelude::Node>::In::blueprints(python)?;
        //         let outputs = <Example as comfy_builder_core::prelude::Node>::Out::blueprints(python)?;
        //
        //         let kwargs = PyDict::new(python);
        //         kwargs.set_item("node_id", "Example")?;
        //         kwargs.set_item("display_name", "Example Node")?;
        //         kwargs.set_item("category", "examples")?;
        //         kwargs.set_item("description", "Node description here")?;
        //         kwargs.set_item("inputs", inputs)?;
        //         kwargs.set_item("outputs", outputs)?;
        //
        //         let schema = io.getattr("Schema")?;
        //
        //         schema.call((), Some(&kwargs))
        //     }
        //
        //     #[pyfunction]
        //     #[pyo3(signature = (class, **kwargs))]
        //     fn execute<'a>(
        //         class: Bound<'a, pyo3::types::PyType>,
        //         kwargs: Option<Bound<'a, PyDict>>,
        //     ) -> PyResult<Bound<'a, PyAny>> {
        //         let instance = Example::default();
        //         let output = instance.execute(instance.initialize_inputs(kwargs.into()));
        //
        //         let python = class.py();
        //         let node_output = python
        //             .import("comfy_api.latest")?
        //             .getattr("io")?
        //             .getattr("NodeOutput")?;
        //
        //         println!("execute function called... {:?}", class);
        //
        //         let kwargs = PyDict::new(python);
        //         kwargs.set_item("node_id", "Example")?;
        //
        //         node_output.call1(output.to_schema(python)?)
        //     }
        //
        //     let methods = PyDict::new(python);
        //     let define_schema_function =
        //         decorator.call1((wrap_pyfunction!(define_schema, python)?,))?;
        //
        //     let execute_function = decorator.call1((wrap_pyfunction!(execute, python)?,))?;
        //
        //     methods.set_item("define_schema", define_schema_function)?;
        //     methods.set_item("execute", execute_function)?;
        //
        //     nodes.append(type_fn.call1(("RustNode", (comfy_node,), methods))?)?;
        // };

        Ok(nodes.unbind())
    })
}

#[pyfunction]
#[pyo3(pass_module)]
fn comfy_entrypoint<'py>(module: &'py Bound<'py, PyModule>) -> PyResult<Bound<'py, PyAny>> {
    println!("comfy entrypoint called");

    let python = module.py();
    let base = python
        .import("comfy_api.latest")?
        .getattr("ComfyExtension")?;

    let methods = PyDict::new(python);

    methods.set_item("get_node_list", wrap_pyfunction!(get_node_list, python)?)?;

    let builtins = python.import("builtins")?;
    let type_fn = builtins.getattr("type")?;

    let extension = type_fn.call1(("RustExtension", (base,), methods))?;

    extension.call0()
}

#[pymodule]
fn comfy_builder_custom_nodes<'py>(
    python: Python<'py>,
    module: Bound<'py, PyModule>,
) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(comfy_entrypoint, python)?)
}
