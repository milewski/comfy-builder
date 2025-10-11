use comfy_builder_core::prelude::{In, Node, Out};
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
        
        for registration in inventory::iter::<comfy_builder_core::registry::NodeRegistration>() {
            nodes.append(registration.register_v2(python, &decorator, &type_fn, &comfy_node)?)?;
        }
       
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
