use crate::registry::Registerable;
use indexmap::IndexMap;
use pyo3::types::PyDict;
use pyo3::{Bound, PyClass, PyResult, Python};
use std::fmt::Display;

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
    Combo,
    Conditioning,
    Custom(&'static str),
}

impl From<&str> for DataType {
    fn from(value: &str) -> Self {
        match value {
            // Primitive
            "u8" | "u16" | "u32" | "u128" | "u64" | "usize" => DataType::Int,
            "i8" | "i16" | "i32" | "i128" | "i64" | "isize" => DataType::Int,
            "f32" | "f64" => DataType::Float,
            "bool" => DataType::Boolean,
            "String" => DataType::String,

            // Tensors
            "Image" => DataType::Image,
            "Mask" => DataType::Mask,
            "Latent" => DataType::Latent,

            // Hidden Inputs
            "UniqueId" => DataType::String,
            "Prompt" => DataType::String,
            "DynPrompt" => DataType::String,
            "ExtraPngInfo" => DataType::String,

            kind => todo!("handle more types {:?}", kind),
        }
    }
}

impl Display for DataType {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            DataType::Int => "INT",
            DataType::Combo => "COMBO",
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

        write!(formatter, "{}", value)
    }
}

pub trait InputPort<'a>: From<Option<&'a Bound<'a, PyDict>>> {
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

pub type NodeResult<'a, T> = Result<<T as Node<'a>>::Out, Box<dyn std::error::Error>>;

pub trait Node<'a>: PyClass + Default + Sync + Send {
    type In: InputPort<'a>;
    type Out: OutputPort<'a>;

    const CATEGORY: &'static str;
    const DESCRIPTION: &'static str;

    fn initialize_input(&'a self, kwargs: Option<&'a Bound<'a, PyDict>>) -> Self::In {
        Self::In::from(kwargs)
    }

    fn new() -> Self {
        Self::default()
    }

    fn execute(&self, input: Self::In) -> NodeResult<'a, Self>;
}

impl<'a, T: Node<'a>> Registerable for T {}

pub trait EnumVariants: From<String> {
    fn variants() -> Vec<&'static str>;
}
