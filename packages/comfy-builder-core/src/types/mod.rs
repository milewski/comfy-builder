use pyo3::types::{IntoPyDict, PyDict};
use pyo3::{Bound, PyAny, PyResult, Python};
use std::collections::HashMap;

pub mod int;
pub mod string;

pub trait ComfyNativeType<'py>: IntoDict<'py> + Default {}

pub trait IntoDict<'py> {
    fn into_dict(
        self: Box<Self>,
        python: Python<'py>,
        io: &Bound<'py, PyAny>,
        extra: Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyDict>>;
}
