use pyo3::PyErr;
use pyo3::exceptions::PyValueError;
use crate::IntoDict;
use crate::registry::EnumRegistration;

pub trait ToComfyType<'py>: IntoDict<'py> {
    fn comfy_type() -> ComfyType;
}

pub enum ComfyType {
    Int,
    Float,
    String,
    Boolean,
    Image,
    Mask,
    Latent,
    Enum,
    ImageUpload,
    Slider,
}

impl ComfyType {
    pub fn to_comfy(&self) -> String {
        match self {
            ComfyType::Int => "Int".to_string(),
            ComfyType::Float => "Float".to_string(),
            ComfyType::String => "String".to_string(),
            ComfyType::Image => "Image".to_string(),
            ComfyType::Mask => "Mask".to_string(),
            ComfyType::Latent => "Latent".to_string(),
            ComfyType::Boolean => "Boolean".to_string(),
            ComfyType::Enum => "Combo".to_string(),
            ComfyType::ImageUpload => "Combo".to_string(),
            // Custom
            ComfyType::Slider => "Int".to_string(),
        }
    }
}

impl TryFrom<&'static str> for ComfyType {
    type Error = PyErr;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let enums = inventory::iter::<EnumRegistration>
            .into_iter()
            .map(|registration| registration.name)
            .collect::<Vec<_>>();

        if enums.contains(&value) {
            return Ok(ComfyType::Enum);
        }

        let value = match value {
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => ComfyType::Int,
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => ComfyType::Int,
            "f32" | "f64" => ComfyType::Float,
            "bool" => ComfyType::Boolean,
            "String" => ComfyType::String,
            "Image" => ComfyType::Image,
            "Mask" => ComfyType::Mask,
            "Latent" => ComfyType::Latent,
            "Slider" => ComfyType::Slider,
            kind => Err(PyValueError::new_err(format!(
                "Unknown data type {:?}",
                kind
            )))?,
        };

        Ok(value)
    }
}