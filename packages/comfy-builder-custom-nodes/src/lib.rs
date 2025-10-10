use pyo3::ffi::c_str;
use pyo3::impl_::pyclass::class_offset;
use pyo3::impl_::pyfunction::WrapPyFunctionArg;
use pyo3::prelude::{PyAnyMethods, PyListMethods, PyModule, PyModuleMethods, PyTypeMethods};
use pyo3::types::{PyDict, PyList, PyTuple, PyType};
use pyo3::{
    Bound, BoundObject, IntoPyObject, IntoPyObjectExt, Py, PyAny, PyErr, PyResult, Python, pyclass,
    pyfunction, pymethods, pymodule, wrap_pyfunction,
};

struct Awaitable<'py, T> {
    value: Bound<'py, T>,
}

impl<'py, T> IntoPyObject<'py> for Awaitable<'py, T> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let asyncio = py.import("asyncio")?;
        let future_class = asyncio.getattr("Future")?;
        let future = future_class.call0()?;
        future.call_method1("set_result", (&self.value,))?;

        Ok(future)
    }
}

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

    let extension = module.getattr("RustExtension")?;

    let class = extend_class(
        module,
        &(extension, base).into_pyobject(python)?,
        "RustExtensionExtended",
    )?;

    class.call0()
}

#[pyclass(subclass)]
struct RustNode;

#[pymethods]
impl RustNode {
    #[new]
    pub fn new() -> Self {
        Self
    }

    // #[classmethod]
    // #[pyo3(name = "GET_BASE_CLASS")]
    // fn get_base_class<'a>(cls: &pyo3::Bound<'a, pyo3::types::PyType>) -> Option<Bound<'a, PyType>> {
    //     println!("get_base_class called");
    //
    //     None
    // }

    #[classmethod]
    fn define_schema<'a>(
        cls: &pyo3::Bound<'a, pyo3::types::PyType>,
    ) -> pyo3::PyResult<Bound<'a, PyType>> {
        println!("defined schema function called...");
        Ok(cls.get_type())
    }
}

#[pyfunction]
fn define_schema<'a>(cls: &pyo3::Bound<'a, pyo3::types::PyType>) -> PyResult<Bound<'a, PyAny>> {
    println!("defined schema function called...");

    let py = cls.py();
    let io = py.import("comfy_api.latest")?.getattr("io")?;

    let dict = PyDict::new(py);
    dict.set_item("default", 0)?;
    dict.set_item("min", 0)?;
    dict.set_item("max", 4096)?;
    dict.set_item("step", 64)?;

    let int_input = io
        .getattr("Int")?
        .getattr("Input")?
        .call1(("int_field", Some(&dict)))?;

    // 3️⃣ Build the keyword‑argument dictionary that will be passed to the
    //     dataclass constructor.
    let kwargs = PyDict::new(py);
    kwargs.set_item("node_id", "Example")?;
    kwargs.set_item("display_name", "Example Node")?;
    kwargs.set_item("category", "examples")?;
    kwargs.set_item("description", "Node description here")?;
    kwargs.set_item("inputs", PyList::new(py, &[int_input])?)?;
    // kwargs.set_item("outputs", PyList::new(py, &[io.getattr("Image")?.getattr("Output")?.call0()?]))?;

    // 4️⃣ Call the constructor (`io.Schema(...)`).
    let schema_cls = io.getattr("Schema")?;

    Ok(schema_cls.call((), Some(&kwargs))?)
}

#[pyfunction]
fn execute() {
    println!("execute function called...");
}

#[pyclass(subclass)]
struct RustExtension;

#[pymethods]
impl RustExtension {
    #[new]
    pub fn new() -> Self {
        Self
    }

    #[classmethod]
    fn get_node_list<'a>(
        _cls: Bound<'a, PyType>,
        python: Python<'a>,
    ) -> PyResult<Awaitable<'a, PyList>> {
        let base = python
            .import("comfy_api.latest")?
            .getattr("io")?
            .getattr("ComfyNode")?;

        let module = python.import("comfy_builder_custom_nodes")?;
        let attribute = module.getattr("RustNode")?;

        module.add_function(wrap_pyfunction!(define_schema, &module)?)?;
        module.add_function(wrap_pyfunction!(execute, &module)?)?;

        /////
        let methods = PyDict::new(python);
        let builtins = python.import("builtins")?;
        let type_fn = builtins.getattr("type")?;
        let object_type = builtins.getattr("object")?;
        let classmethod_ctor = builtins.getattr("classmethod")?;

        let rust_define = module.getattr("define_schema")?;
        let rust_execute = module.getattr("define_schema")?;

        let define_cm = classmethod_ctor.call1((rust_define,))?;
        let execute_cm = classmethod_ctor.call1((rust_execute,))?;

        methods.set_item("define_schema", define_cm)?;
        methods.set_item("execute", execute_cm)?;

        let object_cls = type_fn.call1(("RustNode", (base,), methods))?;
        module.add("RustNode", &object_cls)?;

        ////
        let list = PyList::empty(python);
        list.append(object_cls)?;

        Ok(Awaitable { value: list })
    }
}

#[pymodule]
fn comfy_builder_custom_nodes(
    python: pyo3::Python,
    module: &pyo3::Bound<'_, pyo3::prelude::PyModule>,
) -> PyResult<()> {
    module.add_class::<RustNode>()?;
    module.add_class::<RustExtension>()?;
    module.add_function(wrap_pyfunction!(comfy_entrypoint, module)?)?;

    Ok(())
}
