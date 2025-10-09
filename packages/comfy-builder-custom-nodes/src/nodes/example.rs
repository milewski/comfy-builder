use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyModule, PyString, PyType};

/// Helper that imports `comfy_api.latest.io` and returns the module.
fn import_comfy_io(py: Python) -> PyResult<Bound<PyModule>> {
    py.import("comfy_api.latest.io")
}

/// ---------------------------------------------------------------------------
/// Node: StringConcatenate
/// ---------------------------------------------------------------------------

#[pyclass]
#[derive(Default)]
pub struct StringConcatenate;

/// The Rust implementation of the Python `define_schema` and `execute` methods.
#[pymethods]
impl StringConcatenate {
    /// ``@classmethod
    /// def define_schema(cls):
    ///     ...```
    ///
    #[new]
    pub fn new() -> Self {
        println!("NEWWWWWWWWW");
        Self
    }

    #[classmethod]
    fn define_schema<'a>(_cls: Bound<'a, PyType>, py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        // Import the io module
        let io_mod = import_comfy_io(py)?;

        // Grab the helper classes
        let schema_cls = io_mod.getattr("Schema")?;
        let string_cls = io_mod.getattr("String")?;

        let output = string_cls.getattr("Output")?.call0()?;

        // Construct the Schema instance
        let schema_ = PyDict::new(py);
        schema_.set_item("node_id", "Example")?;
        schema_.set_item("display_name", "Example Noid")?;
        schema_.set_item("category", "Rust / Works")?;
        // schema_.set_item("inputs", PyList::from_vec(py, vec![input_a, input_b, delimiter]))?;
        // schema_.set_item("outputs", PyList::from_vec(py, vec![output]))?;

        // schema_cls.call0()?.call(py, (), Some(schema_.into()))

        Ok(schema_)
    }

    /// ``@classmethod
    /// def execute(cls, string_a, string_b, delimiter):
    ///     return io.NodeOutput(delimiter.join((string_a, string_b)))``
    #[classmethod]
    fn execute(_cls: Bound<PyType>, py: Python) -> PyResult<PyObject> {
        todo!()
    }
}

/// ---------------------------------------------------------------------------
/// Extension: StringExtension
/// ---------------------------------------------------------------------------

#[pyclass]
pub struct StringExtension;

#[pymethods]
impl StringExtension {
    pub fn get_node_list<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        println!("GOT NOOOOOOOODES");

        // Create a future that returns the list of node classes
        let list = PyList::empty(py);
        let node_cls = py.get_type::<StringConcatenate>();
        list.append(node_cls)?;

        Ok(list)
    }
}
