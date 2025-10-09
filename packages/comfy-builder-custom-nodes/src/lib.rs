#![allow(unused, dead_code)]

use pyo3::ffi::c_str;
use pyo3::impl_::pyfunction::WrapPyFunctionArg;
use pyo3::prelude::{PyAnyMethods, PyListMethods, PyModule, PyModuleMethods, PyTypeMethods};
use pyo3::types::{PyDict, PyList, PyTuple, PyType};
use pyo3::{
    Bound, BoundObject, IntoPyObject, IntoPyObjectExt, Py, PyAny, PyErr, PyResult, Python, pyclass,
    pyfunction, pymethods, pymodule, wrap_pyfunction,
};

mod nodes;

#[pyfunction]
#[pyo3(pass_module)]
fn extend_class<'py>(
    module: &Bound<'py, PyModule>,
    bases: &Bound<'py, PyTuple>,
    name: &str,
) -> PyResult<Bound<'py, PyAny>> {
    let methods = PyDict::new(module.py());
    let args = (name, bases, methods);
    let cls = module.py().eval(c_str!("type"), None, None)?.call1(args)?;
    cls.setattr("__module__", module.name()?)?;
    module.add(name, cls.clone())?;
    Ok(cls)
}

#[pyfunction]
#[pyo3(pass_module)]
fn comfy_entrypoint<'py>(module: &'py Bound<'py, PyModule>) -> PyResult<Bound<'py, PyAny>> {
    println!("comfy entrypoint called");

    let python = module.py();
    let base = python
        .import("comfy_api.latest")?
        .getattr("ComfyExtension")?;

    let extension = module.getattr("Extension")?;

    let class = extend_class(
        module,
        &(extension, base).into_pyobject(python)?,
        "ExtensionExtended",
    )?;

    class.call0()
}

#[pyclass(subclass)]
struct MyNode;

#[pymethods]
impl MyNode {
    #[new]
    pub fn new() -> Self {
        Self
    }

    // fn __input_types<'a>(cls: &pyo3::Bound<'a, pyo3::types::PyType>) -> pyo3::PyResult<pyo3::Bound<'a, pyo3::types::PyDict>> {
    // #[classattr]
    // fn define_schema<'a>( py: Python<'a>) -> PyResult<Bound<'a, PyAny>> {
    //     println!("----------------------CALLED DEFINE SCHEMA-----------------------------");
    //     let schema_cls = py
    //         .import("comfy_api.latest")?
    //         .getattr("io")?
    //         .getattr("Schema")?;
    //
    //     // Prepare the arguments you want – they can be empty lists,
    //     // tuples, or whatever the Python implementation expects.
    //     let inputs = PyList::empty(py); // ← replace with real inputs
    //     let outputs = PyList::empty(py); // ← replace with real outputs
    //
    //     // Call the constructor
    //     //   Schema(node_id, display_name, category, inputs, outputs)
    //     let schema_obj = schema_cls.call1((
    //         "MyNode",      // node_id
    //         // "My Node",     // display_name
    //         // "my_category", // category
    //         // inputs,
    //         // outputs,
    //     ))?;
    //
    //     // Return the *Python* object (it is a Schema instance)
    //     Ok(schema_obj)
    // }
}

struct ImmediateAwaitable<'py> {
    /// The value that the awaitable will return
    value: Bound<'py, PyList>,
}

impl<'py> ImmediateAwaitable<'py> {
    fn new(value: Bound<'py, PyList>) -> Self {
        Self { value }
    }

    fn __await__(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let asyncio = py.import("asyncio")?;

        // 2. Create a Future instance
        let future_cls = asyncio.getattr("Future")?;
        let fut = future_cls.call0()?;

        // 3. Set its result to our stored value
        fut.call_method1("set_result", (&self.value,))?;

        Ok(fut)
    }
}

impl<'py> IntoPyObject<'py> for ImmediateAwaitable<'py> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.__await__(py)
    }
}

#[pyclass(subclass)]
struct Extension;

#[pymethods]
impl Extension {
    #[new]
    pub fn new() -> Self {
        Self
    }

    #[staticmethod]
    fn get_node_list<'a>(python: Python<'a>) -> PyResult<ImmediateAwaitable<'a>> {
        println!("GETTING LIST --------------------------->");

        let base = python
            .import("comfy_api.latest")?
            .getattr("io")?
            .getattr("ComfyNode")?;

        let module = python.import("comfy_builder_custom_nodes")?;
        let attribute = module.getattr("MyNode")?;

        let list = PyList::empty(python);
        let class = extend_class(
            &module,
            &(attribute, base).into_pyobject(python)?,
            "MyNodeExtended",
        )?;

        list.append(class)?;

        Ok(ImmediateAwaitable::new(list))
    }
}

#[pyfunction]
#[pyo3(pass_module)]
fn define_schema<'py>(module: &Bound<'py, PyModule>) -> PyResult<Bound<'py, PyAny>> {
    println!("------------------------------------------------------------- define_schema");
    let py = module.py();
    let schema_cls = py
        .import("comfy_api.latest")?
        .getattr("io")?
        .getattr("Schema")?;

    let inputs = PyList::empty(py);
    let outputs = PyList::empty(py);

    let schema_obj = schema_cls.call1((
        "MyNode",  // node_id
        "My Node", // display_name
        "my_category", // category
                   // inputs,
                   // outputs,
    ))?;

    // Return the *Python* object (it is a Schema instance)
    Ok(schema_obj)
}

#[pymodule]
fn comfy_builder_custom_nodes(
    python: pyo3::Python,
    module: &pyo3::Bound<'_, pyo3::prelude::PyModule>,
) -> PyResult<()> {
    module.add_class::<Extension>()?;
    module.add_class::<MyNode>()?;
    // module.add_class::<StringConcatenate>()?;
    // module.add_class::<StringExtension>()?;
    // module.add("StringConcatenate", StringConcatenate)?;
    // module.add("StringExtension", StringExtension)?;
    module.add_function(wrap_pyfunction!(define_schema, module)?)?;
    // module.add_function(wrap_pyfunction!(comfy_entrypoint, module)?)?;

    // module.add_function(wrap_pyfunction!(define_schema, module)?)?;
    //
    // module.add("something_nobel", wrap_pyfunction!(define_schema, module)?)?;

    Ok(())
}
