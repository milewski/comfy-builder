use comfy_builder_core::EnumVariants;
use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{Enum, Image, NodeInput, NodeOutput, node};
use std::error::Error;

#[derive(Enum)]
enum Demo {
    A,
    B,
    C,
}

#[derive(NodeInput)]
pub struct Input {
    number: usize,
}

// impl<'py> From<comfy_builder_core::prelude::Kwargs<'py>> for Input {
//     fn from(kwargs: comfy_builder_core::prelude::Kwargs) -> Self {
//         Input {
//             number: kwargs
//                 .as_ref()
//                 .and_then(|kwargs| kwargs.get_item("number").ok())
//                 .flatten()
//                 .and_then(|value| value.extract::<String>().ok())
//                 // .map(|value| value.wrap())
//                 // .and_then(|string| string.map(|string| if string.is_empty() { None } else { Some(string) }))
//                 // .flatten()
//                 .expect("unable to retrieve attribute."),
//
//         }
//     }
// }

#[derive(NodeOutput)]
pub struct Output {
    number: usize,
}

#[node]
struct Example;

impl<'a> Node<'a> for Example {
    type In = Input;
    type Out = Output;
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(Output {
            number: input.number,
        })
    }
}
