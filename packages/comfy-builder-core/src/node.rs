use pyo3::types::{PyCFunction, PyDict, PyList, PyTuple};
use pyo3::{Bound, PyAny, PyResult, Python};
use std::error::Error;
use std::ops::Deref;

pub trait Node<'a>: Default {
    type In: In<'a>;
    type Out: Out<'a>;

    type Error: Into<Box<dyn Error + Send + Sync>> + 'static;

    fn new() -> Self {
        Default::default()
    }

    fn initialize_inputs(&self, kwargs: Kwargs<'a>) -> Result<Self::In, <Self::In as TryFrom<Kwargs<'a>>>::Error> {
        Self::In::try_from(kwargs)
    }

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error>;
}

pub trait NodeFunctionProvider {
    fn define_fn(python: Python) -> PyResult<Bound<PyCFunction>>;
    fn execute_fn(python: Python) -> PyResult<Bound<PyCFunction>>;
}

pub trait In<'py>: TryFrom<Kwargs<'py>> {
    fn blueprints(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
    fn is_list() -> bool;
}

pub trait Out<'py> {
    fn blueprints(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>>;
}

pub struct Kwargs<'py>(pub Option<Bound<'py, PyDict>>);

impl<'py> From<Option<Bound<'py, PyDict>>> for Kwargs<'py> {
    fn from(value: Option<Bound<'py, PyDict>>) -> Self {
        Kwargs(value)
    }
}

impl<'a> Deref for Kwargs<'a> {
    type Target = Option<Bound<'a, PyDict>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
