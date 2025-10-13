use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{Enum, Image, Latent, Mask, NodeInput, NodeOutput, node};
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
    image: Image,
    r#enum: Demo,
    images: Vec<Image>,
    latent: Latent,
    mask: Mask,
}

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
    latent: Latent,
    mask: Mask,
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
            latent: input.latent,
            mask: input.mask,
        })
    }
}
