use crate::node::{CustomNode, DataType, InputPort, OutputPort};
use comfyui_macro::{node, InputDerive, OutputPort as OutputPortDerive};
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{Bound, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::ops::Deref;

#[derive(Debug, InputDerive)]
pub struct Input {
    // node_id: UniqueId,
    //
    // workflow: Prompt,
    // extra_png_info: ExtraPngInfo,
    // dyn_prompt: DynPrompt,
    //
    #[attribute(multiline = true, placeholder = "demo")]
    message: String,
    //
    // #[attribute(label_on = "ON", label_off = "OFF", default = true)]
    // boolean: bool,
    //
    // #[attribute(min = 0, max = 10, step = 2, default = 6)]
    // width: usize,
    //
    // #[attribute(min = 0, max = 10, step = 2, default = 6)]
    // height: Option<usize>,
    //
    // image_1: TensorWrapper,
    // image_2: Option<TensorWrapper>,
}

#[derive(Debug, OutputPortDerive)]
pub struct Output {
    width: usize,
    // height: usize,
    // image: TensorWrapper,
    message: String,
    // boolean: bool,
    // node_id: String,
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
        dbg!(&input);
        Output {
            // image: input.image_2.unwrap_or_else(|| input.image_1),
            // node_id: input.node_id.to_owned(),
            // boolean: input.boolean,
            message: input.message,
            width: 123,
            // height: input.height.unwrap_or(0),
        }
    }
}
