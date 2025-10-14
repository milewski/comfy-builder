use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{NodeInput, NodeOutput, node};
use std::error::Error;

#[derive(NodeInput, Default)]
pub struct Input {
    string: Vec<String>,
    string_option: Option<Vec<String>>,
}

#[derive(NodeOutput)]
pub struct Output {
    string: Vec<String>,
    string_option: Option<Vec<String>>,
}

#[node(
    description = "Verify that the string option is set to null when its value is blank, and to an empty string when it is not optional.",
    category = "_test"
)]
struct Options;

impl<'a> Node<'a> for Options {
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
    pub fn test_vector() {
        let output = run_node!(Options, Input::default());

        assert_eq!(output.string, Vec::<String>::new());
        assert_eq!(output.string_option, None);
    }
}
