use comfy_builder_core::prelude::*;
use inventory;

#[derive(NodeInput)]
pub struct Input {
    latent: Latent,
}

#[derive(NodeOutput)]
pub struct Output {
    latent: Latent,
}

#[node]
pub struct Sum;

impl<'a> Node<'a> for Sum {
    type In = Input;
    type Out = Output;

    const CATEGORY: &'static str = "Sum / Math";

    const DESCRIPTION: &'static str = r#"
        Sums the left input with the right input.
    "#;

    fn execute(&self, input: Self::In) -> NodeResult<'a, Self> {

        println!("{:#?}", input.latent);

        Ok(Output {
            latent: input.latent,
        })
    }
}
