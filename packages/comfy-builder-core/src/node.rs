use std::error::Error;
use pyo3::{Bound, PyResult, Python};
use pyo3::types::PyCFunction;
use crate::{In, Kwargs, Out};

pub type NodeResult<'a, T> = Result<<T as Node<'a>>::Out, Box<dyn std::error::Error>>;

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