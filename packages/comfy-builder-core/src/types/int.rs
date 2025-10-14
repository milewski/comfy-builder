use crate::types::comfy_type::{AsInput, ComfyType};
use num_traits::ConstZero;
use pyo3::prelude::{PyAnyMethods, PyDictMethods};
use pyo3::types::PyDict;
use pyo3::{Bound, PyAny, PyResult};

macro_rules! impl_comfy_type {
    ($($primitive:ty => $ctype:expr),*) => {
        $(
            impl<'py> AsInput<'py> for $primitive {
                fn comfy_type() -> ComfyType {
                    $ctype
                }

                fn set_options(dict: &mut Bound<'py, PyDict>, _: &Bound<'py, PyAny>) -> PyResult<()> {
                    if let Ok(None) = dict.get_item("min") {
                        dict.set_item("min", Self::MIN)?;
                    }

                    if let Ok(None) = dict.get_item("max") {
                        dict.set_item("max", Self::MAX)?;
                    }

                    if let Ok(None) = dict.get_item("default") {
                        dict.set_item("default", Self::ZERO)?;
                    }

                    if $ctype == ComfyType::Float {
                        if let Ok(None) = dict.get_item("step") {
                            dict.set_item("step", 0.01)?;
                        }
                    }

                    if let (Some(min), Some(max), Some(default)) = (
                        dict.get_item("min")?,
                        dict.get_item("max")?,
                        dict.get_item("default")?,
                    ) {
                        let min = min.extract::<Self>()?;
                        let max = max.extract::<Self>()?;
                        let default = default.extract::<Self>()?;

                        if default < min {
                            dict.set_item("default", min)?;
                        }

                        if default > max {
                            dict.set_item("default", max)?;
                        }
                    }

                    Ok(())
                }
            }
        )*
    };
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
