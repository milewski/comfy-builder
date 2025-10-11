use crate::types::IntoDict;
use pyo3::types::{PyCFunction, PyDict, PyList, PyTuple};
use pyo3::{Bound, IntoPyObject, PyAny, PyResult, Python};
use std::ops::Deref;

pub mod attributes;
pub mod node;
pub mod prelude;
pub mod registry;
pub mod tensors;
pub mod types;

// #[derive(Default)]
// pub struct Int<T> {
//     pub default: T,
//     pub min: T,
//     pub max: T,
//     pub step: T,
// }
//
// pub trait IntoPyDict<'py> {
//     fn to_dict(self, python: Python<'py>) -> PyResult<Bound<'py, PyDict>>;
// }
//
// impl<'py, T: IntoPyObject<'py>> IntoPyDict<'py> for Int<T> {
//     fn to_dict(self, python: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
//         let dict = PyDict::new(python);
//         dict.set_item("default", self.default)?;
//         dict.set_item("min", self.min)?;
//         dict.set_item("max", self.max)?;
//         dict.set_item("step", self.step)?;
//
//         Ok(dict)
//     }
// }

// dict.set_item("default", 0)?;
// dict.set_item("min", 0)?;
// dict.set_item("max", 4096)?;
// dict.set_item("step", 64)?;

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
}

pub trait Out<'py> {
    fn blueprints(python: Python<'py>, io: &Bound<'py, PyAny>) -> PyResult<Bound<'py, PyList>>;
    fn to_schema(self, python: Python) -> PyResult<Bound<PyTuple>>;
}

pub trait Node<'a>: Default {
    type In: In<'a>;
    type Out: Out<'a>;

    fn new() -> Self {
        Default::default()
    }

    fn initialize_inputs(&self, kwargs: Kwargs<'a>) -> Self::In {
        Self::In::from(kwargs)
    }

    fn execute(&self, input: Self::In) -> Self::Out;
}

pub trait ExtractNodeFunctions {
    fn define_function(python: Python) -> PyResult<Bound<PyCFunction>>;
    fn run_function(python: Python) -> PyResult<Bound<PyCFunction>>;
}

pub enum ComfyDataTypes {
    Int(&'static str),
    String,
}

// fn test() {
//     Python::attach(|py| {
//         let kind = ComfyDataTypes::from("usize").to_type();
//         let kind = kind.into_dict(py).unwrap();
//     })
// }

impl ComfyDataTypes {
    pub fn to_comfy(&self) -> String {
        match self {
            ComfyDataTypes::Int(_) => "Int".to_string(),
            ComfyDataTypes::String => "String".to_string(),
        }
    }

    pub fn to_type<'py>(&self) -> Box<dyn IntoDict<'py>> {
        match *self {
            ComfyDataTypes::Int(value) => match value {
                "i8" => Box::new(types::int::Int::<i8>::default()),
                "i16" => Box::new(types::int::Int::<i16>::default()),
                "i32" => Box::new(types::int::Int::<i32>::default()),
                "i64" => Box::new(types::int::Int::<i64>::default()),
                "i128" => Box::new(types::int::Int::<i128>::default()),
                "isize" => Box::new(types::int::Int::<isize>::default()),
                "u8" => Box::new(types::int::Int::<i8>::default()),
                "u16" => Box::new(types::int::Int::<i16>::default()),
                "u32" => Box::new(types::int::Int::<i32>::default()),
                "u64" => Box::new(types::int::Int::<i64>::default()),
                "u128" => Box::new(types::int::Int::<i128>::default()),
                "usize" => Box::new(types::int::Int::<isize>::default()),
                value => unreachable!("invalid int type {}", value),
            },
            ComfyDataTypes::String => Box::new(types::string::String::default()),
        }
    }
}

impl From<&'static str> for ComfyDataTypes {
    fn from(value: &'static str) -> Self {
        match value {
            "i8" | "i16" | "i32" | "i64" | "i128" | "isize" => ComfyDataTypes::Int(value),
            "u8" | "u16" | "u32" | "u64" | "u128" | "usize" => ComfyDataTypes::Int(value),
            "String" => ComfyDataTypes::String,
            kind => panic!("Unknown data type {:?}", kind),
        }
    }
}
