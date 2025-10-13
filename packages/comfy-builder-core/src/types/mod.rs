use crate::ComfyDataTypes;
use pyo3::types::PyDict;
use pyo3::{Bound, PyAny, PyResult};

pub mod boolean;
pub mod r#enum;
pub mod image;
pub mod int;
pub mod latent;
pub mod mask;
pub mod string;

pub trait ComfyNativeType<'py>: IntoDict<'py> {}

pub trait IntoDict<'py> {
    fn into_dict(dict: &mut Bound<'py, PyDict>, io: &Bound<'py, PyAny>) -> PyResult<()> {
        Ok(())
    }
}
