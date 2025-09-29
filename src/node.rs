use indexmap::IndexMap;
use pyo3::types::PyDict;
use pyo3::{Bound, PyClass, PyResult, Python};
use std::fmt::Display;
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

pub enum DataType {
    Int,
    Float,
    String,
    Boolean,
    Image,
    Latent,
    Mask,
    Audio,
    Noise,
    Sampler,
    Sigmas,
    Guider,
    Model,
    Clip,
    Vae,
    Conditioning,
    Custom(&'static str),
}

impl From<&str> for DataType {
    fn from(value: &str) -> Self {
        match value {
            "u8" | "u16" | "u32" | "u128" | "u64" | "usize" => DataType::Int,
            "i8" | "i16" | "i32" | "i128" | "i64" | "isize" => DataType::Int,
            "f32" | "f64" => DataType::Float,
            "bool" => DataType::Boolean,
            "String" => DataType::String,
            "TensorWrapper" => DataType::Image,
            kind => todo!("handle more types {:?}", kind),
        }
    }
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            DataType::Int => "INT",
            DataType::Float => "FLOAT",
            DataType::String => "STRING",
            DataType::Boolean => "BOOLEAN",
            DataType::Image => "IMAGE",
            DataType::Latent => "LATENT",
            DataType::Mask => "MASK",
            DataType::Audio => "AUDIO",
            DataType::Noise => "NOISE",
            DataType::Sampler => "SAMPLER",
            DataType::Sigmas => "SIGMAS",
            DataType::Guider => "GUIDER",
            DataType::Model => "MODEL",
            DataType::Clip => "CLIP",
            DataType::Vae => "VAE",
            DataType::Conditioning => "CONDITIONING",
            DataType::Custom(name) => name,
        };

        write!(f, "{}", value)
    }
}

pub trait InputPort<'a>: From<&'a Bound<'a, PyDict>> {
    fn get_inputs(py: Python<'a>) -> PyResult<Bound<'a, PyDict>>;
}

pub trait OutputPort<'a> {
    fn get_outputs() -> IndexMap<&'static str, DataType>;
    fn values() -> Vec<String> {
        Self::get_outputs()
            .into_values()
            .map(|value| value.to_string())
            .collect()
    }

    fn keys() -> Vec<&'static str> {
        Self::get_outputs().into_keys().collect()
    }
}

pub trait CustomNode<'a>: PyClass + Default {
    type In: InputPort<'a>;
    type Out: OutputPort<'a>;

    const CATEGORY: &'static str;
    const DESCRIPTION: &'static str;

    fn initialize_input(&'a self, kwargs: Option<&'a Bound<'a, PyDict>>) -> Self::In {
        Self::In::from(kwargs.unwrap())
    }

    fn new() -> Self {
        Self::default()
    }

    fn execute(&self, input: Self::In) -> Self::Out;
}
