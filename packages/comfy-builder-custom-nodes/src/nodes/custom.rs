//!
//! Verify that all custom types compiles and works
//!

use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{NodeInput, NodeOutput, node};
use comfy_builder_core::types::seed::Seed;
use comfy_builder_core::types::slider::Slider;
use std::error::Error;

#[derive(NodeInput)]
pub struct Input {
    seed: Seed<u8>,
    slider: Slider<u16>,
}

#[derive(NodeOutput)]
pub struct Output {
    seed: u8,
    slider: u16,
}

#[node]
struct Custom;

impl<'py> Node<'py> for Custom {
    type In = Input;
    type Out = Output;
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(Output {
            seed: *input.seed,
            slider: *input.slider,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use comfy_builder_core::run_node;

    #[test]
    pub fn test_custom_types() {
        let output = run_node!(
            Custom,
            Input {
                slider: Slider::new(1),
                seed: Seed::new(1),
            }
        );

        assert_eq!(output.slider, 1u16);
        assert_eq!(output.seed, 1u8);
    }
}
