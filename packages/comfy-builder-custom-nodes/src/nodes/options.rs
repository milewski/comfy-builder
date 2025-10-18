//!
//! Verify that a string option is set to `None` when its value is blank, and to an empty string when it is *not* optional.
//!
//! If a user sets an input as `Option<String>`, ComfyUI never supplies an empty string.  
//! Consequently, we need custom logic that returns `None` when the string is empty.
//!
//! So this file test that this is the case
//!

use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{NodeInput, NodeOutput, node};
use std::error::Error;

#[derive(NodeInput, Default)]
pub struct Input {
    string: String,
    string_option: Option<String>,
}

#[derive(NodeOutput)]
pub struct Output {
    string: String,
    string_option: Option<String>,
}

#[node]
struct Options;

impl Node for Options {
    type In = Input;
    type Out = Output;
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(Output {
            string: input.string,
            string_option: input.string_option,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use comfy_builder_core::run_node;

    #[test]
    pub fn test_options() {
        let output = run_node!(Options, Input::default());

        assert_eq!(output.string, String::default());
        assert_eq!(output.string_option, None);
    }
}
