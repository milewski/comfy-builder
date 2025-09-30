use crate::node::DataType;
use num_traits::Num;
use pyo3::conversion::FromPyObjectBound;
use pyo3::prelude::PyDictMethods;
use pyo3::types::{PyAnyMethods, PyDict};
use pyo3::{Borrowed, Bound, FromPyObject, PyAny, PyClass, PyResult, Python};
use std::ops::Deref;
// pub enum Kind {
//     Required(Attribute),
//     Optional(Attribute),
//     Hidden { unique_id: bool, prompt: bool },
// }

#[derive(Default)]
pub struct Int<T> {
    pub default: T,
    pub min: T,
    pub max: T,
    pub step: T,
}

#[derive(Default)]
pub struct Boolean {
    default: bool,
    label_on: Option<String>,
    label_off: Option<String>,
}

#[derive(Debug)]
pub struct HiddenUniqueId(pub String);

impl Deref for HiddenUniqueId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'py> FromPyObject<'py> for HiddenUniqueId {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        object
            .extract::<String>()
            .map(|value| HiddenUniqueId(value))
    }
}

impl PluginHiddenAttribute for HiddenUniqueId {
    fn get_key(&self) -> &'static str {
        "UNIQUE_ID"
    }
}

pub trait PluginHiddenAttribute {
    fn get_key(&self) -> &'static str;
}

#[derive(Default)]
pub struct PluginString {
    default: String,
    placeholder: Option<String>,
    multiline: bool,
}

#[derive(Default)]
pub struct Image;

// pub enum Attribute {
//     Image {
//         label: String,
//     },
//     Boolean {
//         label: String,
//         default: bool,
//         label_on: Option<String>,
//         label_off: Option<String>,
//     },
//     Int {
//         label: String,
//         default: usize,
//         min: usize,
//         max: usize,
//         step: usize,
//     },
//     String {
//         label: String,
//         default: String,
//         placeholder: Option<String>,
//         multiline: bool,
//     },
// }

impl PluginAttribute for Int<usize> {
    fn to_dict<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);

        dict.set_item("default", &self.default)?;
        dict.set_item("min", &self.min)?;
        dict.set_item("max", &self.max)?;
        dict.set_item("step", &self.step)?;

        Ok(dict)
    }

    fn to_data_type(&self) -> DataType {
        DataType::Int
    }
}

impl PluginAttribute for Boolean {
    fn to_dict<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);

        dict.set_item("default", &self.default)?;
        dict.set_item("label_on", &self.label_on)?;
        dict.set_item("label_off", &self.label_off)?;

        Ok(dict)
    }

    fn to_data_type(&self) -> DataType {
        DataType::Boolean
    }
}

pub trait PluginAttribute {
    fn to_dict<'py>(&'py self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>>;
    fn to_data_type(&self) -> DataType;
}

// impl Attribute {
//     pub fn label(&self) -> String {
//         match self {
//             Attribute::Image { label, .. } => label.clone(),
//             Attribute::Int { label, .. } => label.clone(),
//             Attribute::String { label, .. } => label.clone(),
//             Attribute::Boolean { label, .. } => label.clone(),
//         }
//     }
//
//     fn data_type(&self) -> DataType {
//         match self {
//             Attribute::Image { .. } => DataType::Image,
//             Attribute::Int { .. } => DataType::Int,
//             Attribute::String { .. } => DataType::String,
//             Attribute::Boolean { .. } => DataType::Boolean,
//         }
//     }
//
//     fn to_dict(self, py: Python) -> PyResult<Bound<PyDict>> {
//         let dict = PyDict::new(py);
//
//         match self {
//             Attribute::Image { .. } => {}
//             Attribute::Int {
//                 min,
//                 max,
//                 step,
//                 default,
//                 ..
//             } => {
//                 dict.set_item("min", min)?;
//                 dict.set_item("max", max)?;
//                 dict.set_item("step", step)?;
//                 dict.set_item("default", default)?;
//             }
//             Attribute::String {
//                 placeholder,
//                 multiline,
//                 default,
//                 ..
//             } => {
//                 dict.set_item("placeholder", placeholder)?;
//                 dict.set_item("multiline", multiline)?;
//                 dict.set_item("default", default)?;
//             }
//             Attribute::Boolean {
//                 label_on,
//                 label_off,
//                 default,
//                 ..
//             } => {
//                 dict.set_item("label_on", label_on)?;
//                 dict.set_item("label_off", label_off)?;
//                 dict.set_item("default", default)?;
//             }
//         }
//
//         Ok(dict)
//     }

// pub fn apply(self, root: &Bound<PyDict>) -> PyResult<()> {
//     root.set_item(
//         self.label(),
//         (self.data_type().to_string(), self.to_dict(root.py())?),
//     )
// }
// }
