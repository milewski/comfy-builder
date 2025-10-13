use comfy_builder_core::ComfyDataTypes;
use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{Enum, Image, NodeInput, NodeOutput, node};
use pyo3::types::PyDict;
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Enum)]
enum Demo {
    #[label = "Hey"]
    A,
    #[label = "Yeah"]
    B,
    #[label = "Oh Oh"]
    C,
}

#[derive(NodeInput)]
pub struct Input {
    string: String,
    string_option: Option<String>,
    usize: usize,
    usize_option: Option<usize>,
    boolean: bool,
    image: Image,
    r#enum: Demo,
    images: Vec<Image>,
}

// impl<'a> From<comfy_builder_core::prelude::Kwargs<'a>> for Input {
//     // string: String,
//     // string_option: Option<String>,
//     // usize: usize,
//     // usize_option: Option<usize>,
//     fn from(kwargs: comfy_builder_core::prelude::Kwargs) -> Self {
//         println!("ARGS {:?}", kwargs.as_ref());
//         Input {
//             string_option: kwargs
//                 .as_ref()
//                 .and_then(|kwargs| kwargs.get_item("string_option").ok())
//                 .flatten()
//                 .and_then(|value| value.extract::<Option<Vec<String>>>().ok())
//                 .and_then(|list| list.map(|list| list.into_iter().next()))
//                 .and_then(|string| {
//                     println!("GOOOOOOT {:?}", string);
//                     string
//                         .map(|string| {
//                             if string.is_empty() { None } else { Some(string) }
//                         })
//                 })
//                 .flatten(),
//
//             images: kwargs
//                 .as_ref()
//                 .and_then(|kwargs| kwargs.get_item("images").ok())
//                 .flatten()
//                 .and_then(|value| value.extract::<Vec<Image>>().ok())
//                 .expect("unable to retrieve attribute."),
//
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
    image: Image,
    r#enum: Demo,
    images: Vec<Image>,
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
            image: input.image,
            images: input.images,
            r#enum: input.r#enum,
        })
    }
}
