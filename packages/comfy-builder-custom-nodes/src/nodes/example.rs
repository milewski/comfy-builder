use comfy_builder_core::prelude::{Image, Node, NodeInput, NodeOutput, node_v3};

#[node_v3]
struct Example;

#[derive(NodeInput)]
pub struct Input {
    images: Vec<Image>,
    string: Option<usize>,
}

#[derive(NodeOutput)]
pub struct Output {
    string: Option<usize>,
}

impl<'a> Node<'a> for Example {
    type In = Input;
    type Out = Output;

    fn execute(&self, input: Self::In) -> Self::Out {
        println!("{:?}", input.string);
        Output {
            string: input.string,
        }
    }
}
