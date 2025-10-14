use num_traits::ConstZero;
use pyo3::prelude::{PyAnyMethods, PyDictMethods};
use pyo3::types::PyDict;
use pyo3::{Bound, PyAny, PyResult};
use crate::types::comfy_type::{ComfyType, AsInput};

macro_rules! impl_comfy_type {
    ($($primitive:ty),*) => {
        $(
            impl<'py> AsInput<'py> for $primitive {
                fn comfy_type() -> ComfyType {
                    ComfyType::Int
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
    usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128, f32, f64
);
