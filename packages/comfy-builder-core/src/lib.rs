use crate::types::IntoDict;
use pyo3::types::{PyDict, PyList, PyTuple};
use pyo3::{Bound, IntoPyObject, PyAny, PyResult, Python};
use std::ops::Deref;

pub mod attributes;
pub mod node;
pub mod prelude;
pub mod registry;
pub mod types;

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

impl<'a> From<Kwargs<'a>> for () {
    fn from(_: Kwargs<'a>) -> Self {}
}

impl<'py> In<'py> for () {
    fn blueprints(python: Python<'py>, _: &Bound<PyAny>) -> PyResult<Bound<'py, PyList>> {
        Ok(PyList::empty(python))
    }

    fn is_list() -> bool {
        false
    }
}

impl<'py> Out<'py> for () {
    fn blueprints(python: Python<'py>, _: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>> {
        Ok(PyList::empty(python))
    }

    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>> {
        ((),).into_pyobject(python)
    }
}

pub trait In<'py>: From<Kwargs<'py>> {
    fn blueprints(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
    fn is_list() -> bool;
}

pub trait Out<'py> {
    fn blueprints(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>>;
}

pub enum ComfyDataTypes {
    Int(&'static str),
    Float(&'static str),
    String,
    Boolean,
    Image,
    Mask,
    Latent,
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
        }
    }

    pub fn generate_dict<'py>(
        &self,
        dict: &mut Bound<'py, PyDict>,
        io: &Bound<'py, PyAny>,
    ) -> PyResult<()> {
        match *self {
            ComfyDataTypes::Int(value) => match value {
                "i8" => types::int::Int::<i8>::into_dict(dict, io),
                "i16" => types::int::Int::<i16>::into_dict(dict, io),
                "i32" => types::int::Int::<i32>::into_dict(dict, io),
                "i64" => types::int::Int::<i64>::into_dict(dict, io),
                "i128" => types::int::Int::<i128>::into_dict(dict, io),
                "isize" => types::int::Int::<isize>::into_dict(dict, io),
                "u8" => types::int::Int::<u8>::into_dict(dict, io),
                "u16" => types::int::Int::<u16>::into_dict(dict, io),
                "u32" => types::int::Int::<u32>::into_dict(dict, io),
                "u64" => types::int::Int::<u64>::into_dict(dict, io),
                "u128" => types::int::Int::<u128>::into_dict(dict, io),
                "usize" => types::int::Int::<usize>::into_dict(dict, io),
                value => unreachable!("invalid int type {}", value),
            },
            ComfyDataTypes::Float(value) => match value {
                "f32" => types::int::Int::<f32>::into_dict(dict, io),
                "f64" => types::int::Int::<f64>::into_dict(dict, io),
                value => unreachable!("invalid int type {}", value),
            },
            ComfyDataTypes::String => types::string::String::into_dict(dict, io),
            ComfyDataTypes::Image => types::image::Image::<f32>::into_dict(dict, io),
            ComfyDataTypes::Mask => types::mask::Mask::<f32>::into_dict(dict, io),
            ComfyDataTypes::Latent => types::latent::Latent::<f32>::into_dict(dict, io),
            ComfyDataTypes::Boolean => types::boolean::Boolean::into_dict(dict, io),
        }
    }
}

impl From<&'static str> for ComfyDataTypes {
    fn from(value: &'static str) -> Self {
        match value {
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => ComfyDataTypes::Int(value),
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => ComfyDataTypes::Int(value),
            "f32" | "f64" => ComfyDataTypes::Float(value),
            "bool" => ComfyDataTypes::Boolean,
            "String" => ComfyDataTypes::String,
            "Image" => ComfyDataTypes::Image,
            "Mask" => ComfyDataTypes::Mask,
            "Latent" => ComfyDataTypes::Latent,
            kind => panic!("Unknown data type {:?}", kind),
        }
    }
}

#[macro_export]
macro_rules! set_defaults {
    ($dict:expr, $( $key:expr => $value:expr ),* $(,)?) => {
        $(
            if $dict.get_item($key)?.is_none() {
                $dict.set_item($key, $value)?;
            }
        )*
    };
}
