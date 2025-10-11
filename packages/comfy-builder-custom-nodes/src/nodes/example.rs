use comfy_builder_core::prelude::{Node, NodeInput, NodeOutput};
use pyo3::prelude::{PyAnyMethods, PyDictMethods};
use comfy_builder_core::Kwargs;

#[derive(Default)]
pub struct Example;

#[derive(NodeInput)]
pub struct Input {
    number: usize,
}

#[derive(NodeOutput)]
pub struct Output {
    number: usize,
}

impl<'a> Node<'a> for Example {
    type In = Input;
    type Out = Output;

    fn execute(&self, input: Self::In) -> Self::Out {
        Output {
            number: input.number + 1,
        }
    }
}
