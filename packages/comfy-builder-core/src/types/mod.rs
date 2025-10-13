use pyo3::types::PyDict;
use pyo3::{Bound, PyAny, PyResult};

pub mod boolean;
pub mod image;
pub mod int;
pub mod latent;
pub mod mask;
pub mod string;
pub mod slider;

pub trait IntoDict<'py> {
    fn into_dict(_: &mut Bound<'py, PyDict>, _: &Bound<'py, PyAny>) -> PyResult<()> {
        Ok(())
    }
}
