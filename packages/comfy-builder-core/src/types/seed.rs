use crate::types::comfy_type::{AsInput, ComfyType};
use crate::types::int::numeric_defaults;
use num_traits::{Bounded, Num};
use pyo3::conversion::FromPyObjectBound;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyDict;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyResult};
use std::ops::Deref;

pub struct Seed<T> {
    value: T,
}

impl<T> Seed<T> {
    pub fn new(value: T) -> Self {
        Seed { value }
    }
}

impl<T> Deref for Seed<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'py, T> AsInput<'py> for Seed<T>
where
    T: Num + Bounded + PartialOrd + IntoPyObject<'py> + for<'a> FromPyObjectBound<'a, 'py>,
{
    fn comfy_type() -> ComfyType {
        ComfyType::Int
    }

    fn set_options(dict: &mut Bound<'py, PyDict>, _: &Bound<'py, PyAny>) -> PyResult<()> {
        dict.set_item("control_after_generate", true)?;

        numeric_defaults::<T>(dict)
    }
}

impl<'py, T: for<'a> FromPyObjectBound<'a, 'py>> FromPyObject<'py> for Seed<T> {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Seed {
            value: object.extract::<T>()?,
        })
    }
}
