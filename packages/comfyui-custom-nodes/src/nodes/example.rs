use comfyui_plugin::node::{InputPort, Node, NodeResult, OutputPort};

use candle_core::backend::BackendDevice;
use comfyui_macro::{InputDerive, OutputPort as OutputPortDerive, node};
use pyo3::IntoPyObject;
use pyo3::types::{PyAnyMethods, PyDictMethods};
use rayon::prelude::*;
use resize::PixelFormat;
use std::ops::Deref;

#[derive(Debug, InputDerive)]
pub struct Input {
    #[attribute(min = 0, step = 1, default = 1024)]
    width: usize,

    #[attribute(min = 0, step = 1, default = 1024)]
    height: usize,
}

#[derive(OutputPortDerive)]
pub struct Output {
    width: usize,
    height: usize,
}

#[node]
pub struct Example;

impl<'a> Node<'a> for Example {
    type In = Input;
    type Out = Output;

    const CATEGORY: &'static str = "God Nodes / Image2";

    const DESCRIPTION: &'static str = r#"
        A full descriptive description about `what` this node is supposed to do.
        This node is extremely versatile you can do whatever you want it is kind magical
    "#;

    fn execute(&self, input: Self::In) -> NodeResult<'a, Self> {
        Ok(Output {
            height: input.height,
            width: input.width,
        })
    }
}
