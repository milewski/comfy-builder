//!
//! Verifies that a node can use `()` as its input **or** its output type.
//!

use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::node;
use std::error::Error;

#[node]
struct Unit;

impl<'py> Node<'py> for Unit {
    type In = ();
    type Out = ();
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, _: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use comfy_builder_core::run_node;

    #[test]
    pub fn test_unit() {
        assert!(run_node!(Unit, (), return).is_ok())
    }
}
