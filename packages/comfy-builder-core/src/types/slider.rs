use crate::types::IntoDict;
use crate::set_defaults;
use num_traits::{Bounded, Num};
use pyo3::conversion::FromPyObjectBound;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyDict;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyResult};
use std::ops::Deref;
use crate::types::comfy_type::{ComfyType, ToComfyType};

pub struct Slider<T> {
    value: T,
}

impl<T> Slider<T> {
    pub fn new(value: T) -> Self {
        Slider { value }
    }
}

impl<T> Deref for Slider<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<'py, T> ToComfyType<'py> for Slider<T>
where
    T: Num + Bounded + PartialOrd + IntoPyObject<'py> + for<'a> FromPyObjectBound<'a, 'py>,
{
    fn comfy_type() -> ComfyType {
        ComfyType::Int
    }
}

impl<'py, T> IntoDict<'py> for Slider<T>
where
    T: Num + Bounded + PartialOrd + IntoPyObject<'py> + for<'a> FromPyObjectBound<'a, 'py>,
{
    fn into_dict(dict: &mut Bound<'py, PyDict>, io: &Bound<'py, PyAny>) -> PyResult<()> {
        set_defaults!(dict,
            "min" => T::min_value(),
            "max" => T::max_value(),
            "default" => T::zero(),
        );

        if let (Ok(min), Ok(max), Ok(default)) = (
            dict.get_item("min"),
            dict.get_item("max"),
            dict.get_item("default"),
        ) {
            let min = min.extract::<T>()?;
            let max = max.extract::<T>()?;
            let default = default.extract::<T>()?;

            if default < min {
                dict.set_item("default", min)?;
            }

            if default > max {
                dict.set_item("default", max)?;
            }
        }

        dict.set_item(
            "display_mode",
            io.getattr("NumberDisplay")?.getattr("slider")?,
        )?;

        Ok(())
    }
}

impl<'py, T: for<'a> FromPyObjectBound<'a, 'py>> FromPyObject<'py> for Slider<T> {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Slider {
            value: object.extract::<T>()?,
        })
    }
}
