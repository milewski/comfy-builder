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
    string: String,
    string_option: Option<String>,
    usize: usize,
    usize_option: Option<usize>,
    boolean: bool,
}

// impl<'py> From<comfy_builder_core::prelude::Kwargs<'py>> for Input {
//     fn from(kwargs: comfy_builder_core::prelude::Kwargs) -> Self {
//         Input {
//             string: kwargs
//                 .as_ref()
//                 .and_then(|kwargs| kwargs.get_item("string").ok())
//                 .flatten()
//                 .and_then(|value| value.extract::<String>().ok())
//                 .and_then(|string| {
//                     string
//                         .map(|string| {
//                             if string.is_empty() { None } else { Some(string) }
//                         })
//                 })
//                 .expect("unable to retrieve attribute."),
//             string_option: kwargs
//                 .as_ref()
//                 .and_then(|kwargs| kwargs.get_item("string_option").ok())
//                 .flatten()
//                 .and_then(|value| value.extract::<Option<String>>().ok())
//                 .and_then(|string| {
//                     string
//                         .map(|string| {
//                             if string.is_empty() { None } else { Some(string) }
//                         })
//                 })
//                 .flatten(),
//             usize: kwargs
//                 .as_ref()
//                 .and_then(|kwargs| kwargs.get_item("usize").ok())
//                 .flatten()
//                 .and_then(|value| value.extract::<usize>().ok())
//                 .expect("unable to retrieve attribute."),
//         }
//     }
// }

#[derive(NodeOutput)]
pub struct Output {
    string: String,
    string_option: Option<String>,
    usize: usize,
    usize_option: Option<usize>,
    boolean: bool,
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
            usize: input.usize,
            usize_option: input.usize_option,
            string_option: input.string_option,
            boolean: input.boolean,
        })
    }
}
