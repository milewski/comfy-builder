use pyo3::prelude::PyDictMethods;
use pyo3::types::{PyCFunction, PyDict, PyList, PyTuple};
use pyo3::{Bound, IntoPyObject, PyResult, Python};
use std::ops::Deref;

pub mod attributes;
pub mod node;
pub mod prelude;
pub mod registry;
pub mod tensors;

#[derive(Default)]
pub struct Int<T> {
    pub default: T,
    pub min: T,
    pub max: T,
    pub step: T,
}

pub trait IntoPyDict<'py> {
    fn to_dict(self, python: Python<'py>) -> PyResult<Bound<'py, PyDict>>;
}

impl<'py, T: IntoPyObject<'py>> IntoPyDict<'py> for Int<T> {
    fn to_dict(self, python: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(python);
        dict.set_item("default", self.default)?;
        dict.set_item("min", self.min)?;
        dict.set_item("max", self.max)?;
        dict.set_item("step", self.step)?;

        Ok(dict)
    }
}

// dict.set_item("default", 0)?;
// dict.set_item("min", 0)?;
// dict.set_item("max", 4096)?;
// dict.set_item("step", 64)?;

pub struct Kwargs<'a>(pub Option<Bound<'a, PyDict>>);

impl<'a> From<Option<Bound<'a, PyDict>>> for Kwargs<'a> {
    fn from(value: Option<Bound<'a, PyDict>>) -> Self {
        Kwargs(value)
    }
}

impl<'a> Deref for Kwargs<'a> {
    type Target = Option<Bound<'a, PyDict>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<Kwargs<'a>> for () {
    fn from(_: Kwargs<'a>) -> Self {}
}

impl<'py> In<'py> for () {
    fn blueprints(python: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        Ok(PyList::empty(python))
    }
}

impl Out for () {
    fn blueprints(python: Python) -> PyResult<Bound<PyList>> {
        Ok(PyList::empty(python))
    }

    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>> {
        ((),).into_pyobject(python)
    }
}

pub trait In<'py>: From<Kwargs<'py>> {
    fn blueprints(python: Python<'py>) -> PyResult<Bound<'py, PyList>>;
}

pub trait Out {
    fn blueprints(python: Python) -> PyResult<Bound<PyList>>;
    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>>;
}

pub trait Node<'a>: Default {
    type In: In<'a>;
    type Out: Out;

    fn new() -> Self {
        Default::default()
    }

    fn initialize_inputs(&self, kwargs: Kwargs<'a>) -> Self::In {
        Self::In::from(kwargs)
    }

    fn execute(&self, input: Self::In) -> Self::Out;
}

pub trait ExtractNodeFunctions {
    fn define_function(python: Python) -> PyResult<Bound<PyCFunction>>;
    fn run_function(python: Python) -> PyResult<Bound<PyCFunction>>;
}

pub enum ComfyDataTypes {
    Int,
    String,
}

impl ComfyDataTypes {
    pub fn to_comfy(&self) -> String {
        match self {
            ComfyDataTypes::Int => "Int".to_string(),
            ComfyDataTypes::String => "String".to_string(),
        }
    }
}

impl From<&str> for ComfyDataTypes {
    fn from(value: &str) -> Self {
        match value {
            "usize" => ComfyDataTypes::Int,
            "String" => ComfyDataTypes::String,
            kind => panic!("Unknown data type {:?}", kind),
        }
    }
}
