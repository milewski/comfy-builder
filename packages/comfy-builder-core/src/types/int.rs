use crate::types::{ComfyNativeType, IntoDict};
use num_traits::{Bounded, Num};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::{PyAnyMethods, PyDictMethods};
use pyo3::types::PyDict;
use pyo3::{Bound, IntoPyObject, PyAny, PyResult, Python};

pub enum NumberDisplay {
    Number,
    Slider,
}

pub struct Int<T> {
    min: T,
    max: T,
    step: T,
    default: T,
}

impl<'py, T: Num + Bounded + IntoPyObject<'py>> ComfyNativeType<'py> for Int<T> {}

impl<T: Num + Bounded> Default for Int<T> {
    fn default() -> Self {
        Self {
            min: T::min_value(),
            max: T::max_value(),
            step: T::one(),
            default: T::zero(),
        }
    }
}

impl<'py, T: IntoPyObject<'py>> IntoDict<'py> for Int<T> {
    fn into_dict(
        self: Box<Self>,
        python: Python<'py>,
        io: &Bound<'py, PyAny>,
        extra: Bound<'py, PyDict>,
    ) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(python);

        dict.set_item("min", self.min)?;
        dict.set_item("max", self.max)?;
        dict.set_item("step", self.step)?;
        dict.set_item("default", self.default)?;

        let number_display = io.getattr("NumberDisplay")?;

        for (key, value) in extra {
            if key.to_string().as_str() == "display_mode" {
                dict.set_item(
                    key,
                    match value.to_string().as_str() {
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
        }

        Ok(dict)
    }
}
