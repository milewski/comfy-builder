use crate::types::{ComfyNativeType, IntoDict};
use pyo3::prelude::PyDictMethods;
use pyo3::types::PyDict;
use pyo3::{Bound, PyAny, PyResult, Python};

pub struct String {
    multiline: bool,
}

impl<'py> ComfyNativeType<'py> for String {}

impl Default for String {
    fn default() -> Self {
        Self { multiline: false }
    }
}

impl<'py> IntoDict<'py> for String {
    fn into_dict(self: Box<Self>, python: Python<'py>, _: &Bound<'py, PyAny>, extra: Bound<'py, PyDict>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(python);

        dict.set_item("multiline", self.multiline)?;

        Ok(dict)
    }
}
