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
use comfy_builder_core::prelude::{NodeInput, node};
use std::error::Error;

#[derive(NodeInput, Default)]
pub struct Input {
    string: String,
}

#[node(category = "example")]
struct Attributes;

impl Node for Attributes {
    type In = Input;
    type Out = ();
    type Error = Box<dyn Error + Send + Sync>;

    const IS_EXPERIMENTAL: bool = true;
    const IS_OUTPUT_NODE: bool = true;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        println!("{}", input.string);
        Ok(())
    }
}
