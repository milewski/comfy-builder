use crate::types::IntoDict;
use crate::{ComfyDataTypes, set_defaults, ToComfyType};
use num_traits::{Bounded, ConstZero, Num};
use pyo3::conversion::FromPyObjectBound;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::{PyAnyMethods, PyDictMethods};
use pyo3::types::PyDict;
use pyo3::{Bound, IntoPyObject, PyAny, PyResult};
use std::any::type_name;

impl<'py> IntoDict<'py> for usize
// where
//     T: Num + Bounded + PartialOrd + IntoPyObject<'py> + for<'a> FromPyObjectBound<'a, 'py>,
{
    fn into_dict(dict: &mut Bound<'py, PyDict>, io: &Bound<'py, PyAny>) -> PyResult<()> {
        set_defaults!(dict,
            "min" => Self::MIN,
            "max" => Self::MAX,
            // "step" => T::one(),
            "default" => Self::ZERO,
        );

        // never allow the default be bellow or above the min/max
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

        if let Some(mode) = dict.get_item("display_mode")? {
            let number_display = io.getattr("NumberDisplay")?;

            dict.set_item(
                "display_mode",
                match mode.to_string().as_str() {
                    "number" => number_display.getattr("number")?,
                    "slider" => number_display.getattr("slider")?,
                    value => {
                        return Err(PyValueError::new_err(format!(
                            "Invalid `display_mode`: `{}`. Expected one of: `number`, `slider`.",
                            value
                        )));
                    }
                },
            )?;
        }

        Ok(())
    }

    fn to_native_type() -> ComfyDataTypes {
        ComfyDataTypes::Int(type_name::<Self>())
    }
}

impl ToComfyType for usize {
    fn comfy_type() -> ComfyDataTypes {
        ComfyDataTypes::Int(type_name::<Self>())
    }
}
