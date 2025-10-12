use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{Image, NodeInput, NodeOutput, node};
use std::error::Error;

#[derive(NodeInput)]
pub struct Input {
    images: Vec<Image>,
    string: Option<usize>,
}

#[derive(NodeOutput)]
pub struct Output {
    string: Option<usize>,
}

#[node]
struct Example;

impl<'a> Node<'a> for Example {
    type In = Input;
    type Out = Output;
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(Output {
            string: input.string,
        })
    }
}
