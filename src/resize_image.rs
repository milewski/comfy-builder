use crate::attributes::{Attribute, Kind};
use crate::node::{CustomNode, DataType, InputPort, OutputPort};
use crate::tensor::TensorWrapper;
use candle_core::Device;
use comfyui_macro::{OutputPort as OutputPortDerive, node};
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{Bound, IntoPyObject, PyAny, PyErr, PyResult, Python};

#[derive(Debug)]
pub struct Input {
    node_id: String,
    message: String,
    boolean: bool,
    width: usize,
    height: usize,
    image_1: TensorWrapper,
    image_2: Option<TensorWrapper>,
}

impl<'a> InputPort<'a> for Input {
    fn get_inputs(py: Python<'a>) -> PyResult<Bound<'a, PyDict>> {
        let out = PyDict::new(py);
        let required = PyDict::new(py);
        let optional = PyDict::new(py);
        let hidden = PyDict::new(py);

        let mut attributes: Vec<Kind> = vec![];

        let hidden_inputs = Kind::Hidden {
            unique_id: true,
            prompt: false,
        };

        let boolean = Kind::Required(Attribute::Boolean {
            label: "boolean".to_string(),
            label_on: Some("When On".to_string()),
            label_off: Some("When Off".to_string()),
        });

        let message = Kind::Required(Attribute::String {
            label: "message".to_string(),
            placeholder: Some("I dont care".to_string()),
            multiline: true,
        });

        let width = Kind::Required(Attribute::Int {
            label: "width".to_string(),
            min: 0,
            max: 1024,
            step: 0,
        });

        let height = Kind::Required(Attribute::Int {
            label: "height".to_string(),
            min: 0,
            max: 1024,
            step: 0,
        });

        let image_1 = Kind::Required(Attribute::Image {
            label: "image_1".to_string(),
        });

        let image_2 = Kind::Optional(Attribute::Image {
            label: "image_2 (optional)".to_string(),
        });

        attributes.push(hidden_inputs);
        attributes.push(boolean);
        attributes.push(message);
        attributes.push(width);
        attributes.push(height);
        attributes.push(image_1);
        attributes.push(image_2);

        for kind in attributes {
            match kind {
                Kind::Required(attribute) => attribute.apply(&required)?,
                Kind::Optional(attribute) => attribute.apply(&optional)?,
                Kind::Hidden { prompt, unique_id } => {
                    if prompt {
                        hidden.set_item("prompt", "PROMPT")?;
                    }
                    if unique_id {
                        hidden.set_item("unique_id", "UNIQUE_ID")?;
                    }
                }
            }
        }

        // hidden.set_item("hidden_id", "UNIQUE_ID")?;
        // hidden.set_item("prompt", "PROMPT")?;
        // hidden.set_item("extra_pnginfo", "EXTRA_PNGINFO")?;

        out.set_item("required", required)?;
        out.set_item("optional", optional)?;
        out.set_item("hidden", hidden)?;

        Ok(out)
    }
}

impl<'a> From<&'a Bound<'a, PyDict>> for Input {
    fn from(kwargs: &'a Bound<'a, PyDict>) -> Self {
        println!("kwargs: {:?}", kwargs);
        Self {
            node_id: kwargs
                .get_item("unique_id")
                .unwrap()
                .and_then(|value| value.extract::<String>().ok())
                .unwrap(),

            boolean: kwargs
                .get_item("boolean")
                .unwrap()
                .and_then(|value| value.extract::<bool>().ok())
                .unwrap(),

            message: kwargs
                .get_item("message")
                .unwrap()
                .and_then(|value| value.extract::<String>().ok())
                .unwrap(),

            image_1: kwargs
                .get_item("image_1")
                .unwrap()
                .and_then(|v| v.extract::<Bound<PyAny>>().ok())
                .map(|v| TensorWrapper::new(&v, &Device::Cpu))
                .unwrap(),

            image_2: kwargs
                .get_item("image_2 (optional)")
                .ok()
                .flatten()
                .and_then(|v| v.extract::<Bound<PyAny>>().ok())
                .map(|v| TensorWrapper::new(&v, &Device::Cpu)),

            width: kwargs
                .get_item("width")
                .unwrap()
                .and_then(|v| v.extract::<usize>().ok())
                .unwrap(),

            height: kwargs
                .get_item("height")
                .unwrap()
                .and_then(|v| v.extract::<usize>().ok())
                .unwrap(),
        }
    }
}

#[derive(Debug, OutputPortDerive)]
pub struct Output {
    width: usize,
    height: usize,
    image: TensorWrapper,
    message: String,
    boolean: bool,
    node_id: String,
}

#[node]
pub struct ResizeImage;

impl<'a> CustomNode<'a> for ResizeImage {
    type In = Input;
    type Out = Output;

    const CATEGORY: &'static str = "God Nodes / Image";

    const DESCRIPTION: &'static str = r#"
        A full descriptive description about `what` this node is supposed to do.
        This node is extremely versatile you can do whatever you want it is kind magical
    "#;

    fn execute(&self, input: Self::In) -> Self::Out {
        println!("GOT {:?}", input);

        Output {
            image: {
                if input.image_2.is_some() {
                    input.image_2.unwrap()
                } else {
                    input.image_1
                }
            },
            node_id: input.node_id,
            boolean: input.boolean,
            message: input.message,
            width: input.width,
            height: input.height,
        }
    }
}
