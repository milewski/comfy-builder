use pyo3::types::{PyCFunction, PyDict, PyList, PyTuple};
use pyo3::{Bound, PyAny, PyErr, PyResult, Python};
use std::error::Error;
use std::ops::Deref;

pub trait Node: Default {
    type In: In;
    type Out: Out;

    type Error: Into<Box<dyn Error + Send + Sync>> + 'static;

    fn new() -> Self {
        Default::default()
    }

    fn initialize_inputs<'py>(
        &self,
        kwargs: Kwargs<'py>,
    ) -> Result<Self::In, <Self::In as TryFrom<Kwargs<'py>>>::Error> {
        Self::In::try_from(kwargs)
    }

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error>;
}

pub trait NodeFunctionProvider {
    fn define_fn(python: Python) -> PyResult<Bound<PyCFunction>>;
    fn execute_fn(python: Python) -> PyResult<Bound<PyCFunction>>;
}

pub trait In: for<'py> TryFrom<Kwargs<'py>> {
    fn blueprints<'py>(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
    fn is_list() -> bool;
}

pub trait Out {
    fn blueprints<'py>(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
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

//--- Allow Input / Output to be set as empty unit type

impl Out for () {
    fn blueprints<'py>(python: Python<'py>, _: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>> {
        Ok(PyList::empty(python))
    }

    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>> {
        Ok(PyTuple::empty(python))
    }
}

impl<'py> TryFrom<Kwargs<'py>> for () {
    type Error = PyErr;

    fn try_from(_: Kwargs) -> Result<Self, Self::Error> {
        Ok(())
    }
}

impl In for () {
    fn blueprints<'py>(python: Python<'py>, _: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>> {
        Ok(PyList::empty(python))
    }

    fn is_list() -> bool {
        false
    }
}
