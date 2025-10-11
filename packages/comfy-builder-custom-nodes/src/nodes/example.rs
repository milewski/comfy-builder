use comfy_builder_core::prelude::{Node, NodeInput, NodeOutput, node_v3};

#[node_v3]
struct Example;

#[derive(NodeInput)]
pub struct Input {
    left: i8,
    right: usize,
    string: String,
}

#[derive(NodeOutput)]
pub struct Output {
    number: usize,
    string: String,
}

impl<'a> Node<'a> for Example {
    type In = Input;
    type Out = Output;

    fn execute(&self, input: Self::In) -> Self::Out {
        Output {
            number: input.left as usize + input.right,
            string: input.string,
        }
    }
}
