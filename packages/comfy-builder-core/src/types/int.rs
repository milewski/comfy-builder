use crate::types::comfy_type::{AsInput, ComfyType};
use num_traits::{Bounded, Num};
use pyo3::conversion::FromPyObjectBound;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyDict;
use pyo3::{Bound, IntoPyObject, PyAny, PyResult};

macro_rules! impl_comfy_type {
    ($($primitive:ty => $ctype:expr),*) => {
        $(
            impl<'py> AsInput<'py> for $primitive {
                fn comfy_type() -> ComfyType {
                    $ctype
                }

                fn set_options(dict: &mut Bound<'py, PyDict>, _: &Bound<'py, PyAny>) -> PyResult<()> {
                    if $ctype == ComfyType::Float {
                        if dict.get_item("step").is_err() {
                            dict.set_item("step", 0.01)?;
                        }
                    }

                    numeric_defaults::<$primitive>(dict)
                }
            }
        )*
    };
}

pub fn numeric_defaults<'py, T>(dict: &Bound<'py, PyDict>) -> PyResult<()>
where
    T: Num + Bounded + PartialOrd + IntoPyObject<'py> + for<'a> FromPyObjectBound<'a, 'py>,
{
    if dict.get_item("min").is_err() {
        dict.set_item("min", T::min_value())?;
    }

    if dict.get_item("max").is_err() {
        dict.set_item("max", T::max_value())?;
    }

    if dict.get_item("default").is_err() {
        dict.set_item("default", T::zero())?;
    }

    if let Ok(min) = dict.get_item("min")
        && let Ok(max) = dict.get_item("max")
        && let Ok(default) = dict.get_item("default")
    {
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

    Ok(())
}

impl_comfy_type!(
    usize => ComfyType::Int,
    u8 => ComfyType::Int,
    u16 => ComfyType::Int,
    u32 => ComfyType::Int,
    u64 => ComfyType::Int,
    u128 => ComfyType::Int,
    isize => ComfyType::Int,
    i8 => ComfyType::Int,
    i16 => ComfyType::Int,
    i32 => ComfyType::Int,
    i64 => ComfyType::Int,
    i128 => ComfyType::Int,
    f32 => ComfyType::Float,
    f64 => ComfyType::Float
);
