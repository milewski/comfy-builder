use pyo3::types::PyDict;
use pyo3::{Bound, IntoPyObject, PyClass};
use std::fmt::Debug;

// impl FromKwargs for Input {
//     fn from_kwargs(kwargs: &Bound<PyDict>) -> Self {
//         Self {
//             image: kwargs
//                 .get_item("image")
//                 .unwrap()
//                 .and_then(|v| v.extract::<Bound<PyAny>>().ok())
//                 .map(|v| TensorWrapper::new(&v, &Device::Cpu))
//                 .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'image'"))
//                 .unwrap(),
//
//             width: kwargs
//                 .get_item("width")
//                 .unwrap()
//                 .and_then(|v| v.extract::<usize>().ok())
//                 .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'width'"))
//                 .unwrap(),
//
//             height: kwargs
//                 .get_item("height")
//                 .unwrap()
//                 .and_then(|v| v.extract::<usize>().ok())
//                 .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'height'"))
//                 .unwrap(),
//         }
//     }
// }

// pub trait FromKwargs {
//     fn from_kwargs(kwargs: &Bound<PyDict>) -> Self;
// }

pub trait CustomNode<'a>: PyClass {
    type In: From<&'a Bound<'a, PyDict>>;
    type Out: IntoPyObject<'a>;

    const CATEGORY: &'static str;
    const DESCRIPTION: &'static str;

    fn initialize_input(&'a self, kwargs: Option<&'a Bound<'a, PyDict>>) -> Self::In {
        Self::In::from(kwargs.unwrap())
    }

    fn new() -> Self;
    fn execute(&self, input: Self::In) -> Self::Out;
}
