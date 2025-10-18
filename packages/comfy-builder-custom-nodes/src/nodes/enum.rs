//!
//! Verify that enum works
//!

use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{Enum, NodeInput, NodeOutput, node};
use std::error::Error;

#[derive(Enum, Debug, PartialEq)]
enum Interpolation {
    Linear,
    Bilinear,
    Triangle,
}

#[derive(NodeInput, Debug)]
pub struct Input {
    interpolation: Interpolation,
    interpolation_option: Option<Interpolation>,
}

#[derive(NodeOutput, Debug)]
pub struct Output {
    interpolation: Interpolation,
    interpolation_option: Option<Interpolation>,
}

#[node]
struct EnumOption;

impl Node for EnumOption {
    type In = Input;
    type Out = Output;
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(Output {
            interpolation: input.interpolation,
            interpolation_option: input.interpolation_option,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use comfy_builder_core::run_node;

    #[test]
    pub fn test_enums() {
        let output = run_node!(
            EnumOption,
            Input {
                interpolation: Interpolation::Bilinear,
                interpolation_option: None,
            }
        );

        assert_eq!(output.interpolation, Interpolation::Bilinear);
        assert!(output.interpolation_option.is_none());
    }
}
