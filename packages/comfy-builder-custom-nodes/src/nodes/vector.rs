//!
//! In ComfyUI a node’s input signature is kept identical to the user‑defined one,
//! but the framework internally normalises the data.
//!
//! * If **any** of the supplied inputs is a list, **all** inputs are treated as
//!   lists.  Consequently, passing a `Vec<T>` on an input causes every input to
//!   be converted into `Vec<T>` internally.
//! * When the node is executed, only the **first element** of each input list
//!   is actually used.
//! * This design means that the public API does not require callers to manually
//!   index into the value (`input.value[0]`), which is a cumbersome pattern in
//!   the Python implementation.
//!
//! In short: the node’s signature stays exactly as the user defined it,
//! while the runtime silently coerces inputs to lists and uses only the first
//! element for computation. This avoids “ugly” manual indexing and keeps
//! the Rust API ergonomic.
//!

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

#[node]
struct Vector;

impl Node for Vector {
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
        let output = run_node!(Vector, Input::default());

        assert_eq!(output.string, Vec::<String>::new());
        assert_eq!(output.string_option, None);
    }
}
