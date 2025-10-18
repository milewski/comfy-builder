//!
//! Verify that all tensor type works
//!

use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{NodeInput, NodeOutput, node};
use comfy_builder_core::types::sigmas::Sigmas;
use std::error::Error;

#[derive(NodeInput)]
pub struct Input {
    sigmas: Sigmas,
}

#[derive(NodeOutput)]
pub struct Output {
    sigmas: Sigmas,
}

#[node]
struct Custom;

impl Node for Custom {
    type In = Input;
    type Out = Output;
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(Output { sigmas: input.sigmas })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use comfy_builder_core::run_node;

    #[test]
    pub fn test_sigmas() -> comfy_builder_core::candle::Result<()> {
        let output = run_node!(
            Custom,
            Input {
                sigmas: Sigmas::blank()?
            }
        );

        Ok(())
    }
}
