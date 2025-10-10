use comfy_builder_core::prelude::{Node, NodeInput};
use pyo3::prelude::{PyAnyMethods, PyDictMethods};

#[derive(Default)]
pub struct Example;

#[derive(NodeInput)]
pub struct Input {
    number: usize,
}

#[derive(NodeInput)]
pub struct Input2 {
    number: usize,
}

impl<'a> Node<'a> for Example {
    type In = Input;
    type Out = ();

    fn execute(&self, input: Self::In) -> Self::Out {
        println!("Received {:?}", input.number);
    }
}
