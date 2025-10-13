use crate::registry::EnumRegistration;
pub use crate::types::IntoDict;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyDict, PyList, PyTuple};
use pyo3::{Bound, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::ops::Deref;

pub mod attributes;
pub mod node;
pub mod prelude;
pub mod registry;
pub mod types;

pub trait ComfyInput<'py>: ToComfyType<'py> + IntoDict<'py> {}

pub struct Kwargs<'py>(pub Option<Bound<'py, PyDict>>);

impl<'py> From<Option<Bound<'py, PyDict>>> for Kwargs<'py> {
    fn from(value: Option<Bound<'py, PyDict>>) -> Self {
        Kwargs(value)
    }
}

impl<'a> Deref for Kwargs<'a> {
    type Target = Option<Bound<'a, PyDict>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// impl<'a> TryFrom<Kwargs<'a>> for () {
//     type Error = PyErr;
//
//     fn try_from(value: Kwargs<'a>) -> Result<Self, Self::Error> {
//        Ok(())
//     }
// }
//
// impl<'py> In<'py> for () {
//     fn blueprints(python: Python<'py>, _: &Bound<PyAny>) -> PyResult<Bound<'py, PyList>> {
//         Ok(PyList::empty(python))
//     }
//
//     fn is_list() -> bool {
//         false
//     }
// }

impl<'py> Out<'py> for () {
    fn blueprints(python: Python<'py>, _: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>> {
        Ok(PyList::empty(python))
    }

    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>> {
        ((),).into_pyobject(python)
    }
}

pub trait In<'py>: TryFrom<Kwargs<'py>> {
    fn blueprints(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
    fn is_list() -> bool;
}

pub trait Out<'py> {
    fn blueprints(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>>;
}

pub trait ToComfyType<'py>: IntoDict<'py> {
    fn comfy_type() -> ComfyDataTypes;
}

// impl<'py, T: ToComfyType<'py>> IntoDict<'py> for Option<T> {
//     fn into_dict(dict: &mut Bound<'py, PyDict>, io: &Bound<'py, PyAny>) -> PyResult<()> {
//         T::into_dict(dict, io)
//     }
// }
//
// impl<'py, T: ToComfyType<'py>> ToComfyType<'py> for Option<T> {
//     fn comfy_type() -> ComfyDataTypes {
//         T::comfy_type()
//     }
// }

pub enum ComfyDataTypes {
    Int(&'static str),
    Float(&'static str),
    String,
    Boolean,
    Image,
    Mask,
    Latent,
    Enum,
    ImageUpload,
    Slider,
}

impl ComfyDataTypes {
    pub fn to_comfy(&self) -> String {
        match self {
            ComfyDataTypes::Int(_) => "Int".to_string(),
            ComfyDataTypes::Float(_) => "Float".to_string(),
            ComfyDataTypes::String => "String".to_string(),
            ComfyDataTypes::Image => "Image".to_string(),
            ComfyDataTypes::Mask => "Mask".to_string(),
            ComfyDataTypes::Latent => "Latent".to_string(),
            ComfyDataTypes::Boolean => "Boolean".to_string(),
            ComfyDataTypes::Enum => "Combo".to_string(),
            ComfyDataTypes::ImageUpload => "Combo".to_string(),
            // Custom
            ComfyDataTypes::Slider => "Int".to_string(),
        }
    }
}

impl TryFrom<&'static str> for ComfyDataTypes {
    type Error = PyErr;

    fn try_from(value: &'static str) -> Result<Self, Self::Error> {
        let enums = inventory::iter::<EnumRegistration>
            .into_iter()
            .map(|registration| registration.name)
            .collect::<Vec<_>>();

        if enums.contains(&value) {
            return Ok(ComfyDataTypes::Enum);
        }

        let value = match value {
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => ComfyDataTypes::Int(value),
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => ComfyDataTypes::Int(value),
            "f32" | "f64" => ComfyDataTypes::Float(value),
            "bool" => ComfyDataTypes::Boolean,
            "String" => ComfyDataTypes::String,
            "Image" => ComfyDataTypes::Image,
            "Mask" => ComfyDataTypes::Mask,
            "Latent" => ComfyDataTypes::Latent,
            "Slider" => ComfyDataTypes::Slider,
            kind => Err(PyValueError::new_err(format!(
                "Unknown data type {:?}",
                kind
            )))?,
        };

        Ok(value)
    }
}

#[macro_export]
macro_rules! set_defaults {
    ($dict:expr, $( $key:expr => $value:expr ),* $(,)?) => {
        $(
            if let Err(_) = $dict.get_item($key) {
                $dict.set_item($key, $value)?;
            }
        )*
    };
}
