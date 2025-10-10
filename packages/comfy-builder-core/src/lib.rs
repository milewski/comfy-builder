use std::ops::Deref;
use pyo3::Bound;
use pyo3::types::PyDict;

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

impl<'a> In<'a> for () {}
impl Out for () {}

pub trait In<'a>: From<Kwargs<'a>> {}
pub trait Out {}

pub trait Node<'a> {
    type In: In<'a>;
    type Out: Out;

    fn initialize_inputs(&self, kwargs: Kwargs<'a>) -> Self::In {
        Self::In::from(kwargs)
    }

    fn execute(&self, input: Self::In) -> Self::Out;
}