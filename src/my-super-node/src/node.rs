use crate::tensor::TensorWrapper;
use candle_core::Device;
use pyo3::prelude::PyDictMethods;
use pyo3::types::{PyAnyMethods, PyDict, PyType};
use pyo3::{Bound, PyAny, PyClass, pyclass, pymethods};
use std::fmt::Debug;

#[derive(Debug)]
pub struct Input {
    width: usize,
    height: usize,
    image: TensorWrapper,
}

impl FromKwargs for Input {
    fn from_kwargs(kwargs: &Bound<PyDict>) -> Self {
        Self {
            image: kwargs
                .get_item("image")
                .unwrap()
                .and_then(|v| v.extract::<Bound<PyAny>>().ok())
                .map(|v| TensorWrapper::new(&v, &Device::Cpu))
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'image'"))
                .unwrap(),

            width: kwargs
                .get_item("width")
                .unwrap()
                .and_then(|v| v.extract::<usize>().ok())
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'width'"))
                .unwrap(),

            height: kwargs
                .get_item("height")
                .unwrap()
                .and_then(|v| v.extract::<usize>().ok())
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'height'"))
                .unwrap(),
        }
    }
}

struct Output {
    width: usize,
    height: usize,
}

#[pyclass]
pub struct ResizeImage {
    device: Device,
    // parameters: PhantomData<Parameters<Input, Output>>,
}

struct Parameters<I, O> {
    inputs: I,
    outputs: O,
}

#[pymethods]
impl ResizeImage {
    #[new]
    fn initialize() -> Self {
        ResizeImage::new()
    }

    #[classattr]
    #[pyo3(name = "DESCRIPTION")]
    fn description() -> &'static str {
        "A full descriptive description about what this node is supposed to do."
    }

    #[classattr]
    #[pyo3(name = "FUNCTION")]
    fn function() -> &'static str {
        "run"
    }

    #[classmethod]
    #[pyo3(name = "INPUT_TYPES")]
    fn input_types<'a>(cls: &Bound<'a, PyType>) -> Bound<'a, PyDict> {
        let py = cls.py();
        let out = PyDict::new(py);
        let required = PyDict::new(py);

        // image
        required.set_item("image", ("IMAGE",)).unwrap();

        // width
        let width = PyDict::new(py);
        width.set_item("default", 1024).unwrap();
        width.set_item("min", 0).unwrap();
        // width.set_item("max", 1024).unwrap();
        width.set_item("step", 1).unwrap();

        required.set_item("width", ("INT", width)).unwrap();

        // width
        let height = PyDict::new(py);
        height.set_item("default", 1024).unwrap();
        height.set_item("min", 0).unwrap();
        // height.set_item("max", 1024).unwrap();
        height.set_item("step", 1).unwrap();

        required.set_item("height", ("INT", height)).unwrap();

        out.set_item("required", required).unwrap();

        out
    }

    #[classattr]
    #[pyo3(name = "RETURN_TYPES")]
    fn return_types() -> (&'static str, &'static str) {
        ("INT", "IMAGE")
    }

    #[classattr]
    #[pyo3(name = "CATEGORY")]
    fn category() -> &'static str {
        "Example"
    }

    #[classmethod]
    #[pyo3(signature = (**kwargs))]
    pub fn run<'a>(
        py: &'a Bound<PyType>,
        kwargs: Option<&Bound<PyDict>>,
    ) -> (usize, Bound<'a, PyAny>) {
        println!("GOT {:?}", kwargs.unwrap().keys());

        let instance = Self::new();
        instance.execute(instance.parse_input(kwargs));

        todo!()
    }
}

impl ResizeImage {
    pub fn new() -> ResizeImage {
        Self {
            device: Device::Cpu,
            // parameters: PhantomData::default(),
        }
    }

    // pub fn execute(
    //     &self,
    //     tensor: TensorWrapper,
    //     height: usize,
    //     width: usize,
    // ) -> (usize, TensorWrapper) {
    //     let (batch, orig_h, orig_w, channels) = tensor.dims4().unwrap();
    //     assert_eq!(channels, 3, "Only 3-channel (RGB) images supported");
    //     println!("hello world");
    //
    //     (1, tensor)
    // }
}

impl CustomNode for ResizeImage {
    type In = Input;
    type Out = ();

    fn execute(&self, input: Self::In) -> Self::Out {
        println!("GOT {:?}", input);
    }
}

pub trait FromKwargs: Sized {
    fn from_kwargs(kwargs: &Bound<PyDict>) -> Self;
}

pub trait CustomNode: PyClass {
    type In: FromKwargs;
    type Out;

    fn parse_input(&self, kwargs: Option<&Bound<PyDict>>) -> Self::In {
        Self::In::from_kwargs(kwargs.unwrap())
    }

    fn execute(&self, input: Self::In) -> Self::Out;
}
