use crate::node::DataType;
use pyo3::prelude::PyDictMethods;
use pyo3::types::PyDict;
use pyo3::{Bound, PyResult, Python};

pub enum Kind {
    Required(Attribute),
    Optional(Attribute),
    Hidden { unique_id: bool, prompt: bool },
}

pub enum Attribute {
    Image {
        label: String,
    },
    Boolean {
        label: String,
        label_on: Option<String>,
        label_off: Option<String>,
    },
    Int {
        label: String,
        min: usize,
        max: usize,
        step: usize,
    },
    String {
        label: String,
        placeholder: Option<String>,
        multiline: bool,
    },
}

impl Attribute {
    pub fn label(&self) -> String {
        match self {
            Attribute::Image { label, .. } => label.clone(),
            Attribute::Int { label, .. } => label.clone(),
            Attribute::String { label, .. } => label.clone(),
            Attribute::Boolean { label, .. } => label.clone(),
        }
    }

    fn data_type(&self) -> DataType {
        match self {
            Attribute::Image { .. } => DataType::Image,
            Attribute::Int { .. } => DataType::Int,
            Attribute::String { .. } => DataType::String,
            Attribute::Boolean { .. } => DataType::Boolean,
        }
    }

    fn to_dict(self, py: Python) -> PyResult<Bound<PyDict>> {
        let dict = PyDict::new(py);

        match self {
            Attribute::Image { .. } => {}
            Attribute::Int { min, max, step, .. } => {
                dict.set_item("min", min)?;
                dict.set_item("max", max)?;
                dict.set_item("step", step)?;
            }
            Attribute::String {
                placeholder,
                multiline,
                ..
            } => {
                dict.set_item("placeholder", placeholder)?;
                dict.set_item("multiline", multiline)?;
            }
            Attribute::Boolean {
                label_on,
                label_off,
                ..
            } => {
                dict.set_item("label_on", label_on)?;
                dict.set_item("label_off", label_off)?;
            }
        }

        Ok(dict)
    }

    pub fn apply(self, root: &Bound<PyDict>) -> PyResult<()> {
        root.set_item(
            self.label(),
            (self.data_type().to_string(), self.to_dict(root.py())?),
        )
    }
}
