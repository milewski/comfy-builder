use pyo3::types::PyDict;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyResult};
use std::fmt::{Display, Formatter};

pub trait AsInput<'py>: FromPyObject<'py> {
    fn comfy_type() -> ComfyType;

    fn set_options(_: &mut Bound<'py, PyDict>, _: &Bound<'py, PyAny>) -> PyResult<()> {
        Ok(())
    }
}

pub trait AsOutput<'py>: IntoPyObject<'py> {}

#[derive(PartialEq)]
pub enum ComfyType {
    Int,
    Float,
    String,
    Boolean,
    Image,
    Mask,
    Latent,
    Enum,
    Slider,
    Sigmas,
}

impl Display for ComfyType {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{}",
            match self {
                ComfyType::Int => "Int".to_string(),
                ComfyType::Float => "Float".to_string(),
                ComfyType::String => "String".to_string(),
                ComfyType::Image => "Image".to_string(),
                ComfyType::Mask => "Mask".to_string(),
                ComfyType::Latent => "Latent".to_string(),
                ComfyType::Boolean => "Boolean".to_string(),
                ComfyType::Enum => "Combo".to_string(),
                ComfyType::Sigmas => "Sigmas".to_string(),
                // Custom
                ComfyType::Slider => "Int".to_string(),
            }
        )
    }
}
