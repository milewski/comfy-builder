use pyo3::types::{PyDict, PyList, PyTuple};
use pyo3::{Bound, IntoPyObject, Py, PyResult, Python};
use std::ops::Deref;

pub mod attributes;
pub mod node;
pub mod prelude;
pub mod registry;
pub mod tensors;

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
    fn to_schema<'a>(&self, python: Python<'a>) -> PyResult<Bound<'a, PyTuple>> {
        ((),).into_pyobject(python)
    }
}

pub trait In<'py>: From<Kwargs<'py>> {
    fn blueprints(python: Python<'py>) -> PyResult<Bound<'py, PyList>>;
}

pub trait Out {
    fn to_schema<'a>(&self, python: Python<'a>) -> PyResult<Bound<'a, PyTuple>>;
}

pub trait Node<'a> {
    type In: In<'a>;
    type Out: Out;

    fn initialize_inputs(&self, kwargs: Kwargs<'a>) -> Self::In {
        Self::In::from(kwargs)
    }

    fn execute(&self, input: Self::In) -> Self::Out;
}
