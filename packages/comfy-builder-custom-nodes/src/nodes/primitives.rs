use comfy_builder_core::node::Node;
use comfy_builder_core::prelude::{NodeInput, NodeOutput, node};
use std::error::Error;

#[derive(NodeInput, Default)]
pub struct Input {
    string: String,
    boolean: bool,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
    isize: isize,
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    usize: usize,
}

#[derive(NodeOutput)]
pub struct Output {
    string: String,
    boolean: bool,
    i8: i8,
    i16: i16,
    i32: i32,
    i64: i64,
    i128: i128,
    isize: isize,
    u8: u8,
    u16: u16,
    u32: u32,
    u64: u64,
    u128: u128,
    usize: usize,
}

#[node(
    description = "Test all primitive types to ensure they do not cause crashes or other issues when used.",
    category = "_test"
)]
struct Primitives;

impl<'a> Node<'a> for Primitives {
    type In = Input;
    type Out = Output;
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(Output {
            string: input.string,
            boolean: input.boolean,
            i8: input.i8,
            i16: input.i16,
            i32: input.i32,
            i64: input.i64,
            i128: input.i128,
            isize: input.isize,
            u8: input.u8,
            u16: input.u16,
            u32: input.u32,
            u64: input.u64,
            u128: input.u128,
            usize: input.usize,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::nodes::primitives::{Input, Primitives};
    use comfy_builder_core::prelude::*;
    use comfy_builder_core::run_node;

    #[test]
    pub fn test_all_primitive_inputs() {
        let output = run_node!(Primitives, Input::default());

        assert_eq!(output.string, String::default());
        assert_eq!(output.boolean, bool::default());

        assert_eq!(output.u8, u8::default());
        assert_eq!(output.u16, u16::default());
        assert_eq!(output.u32, u32::default());
        assert_eq!(output.u64, u64::default());
        assert_eq!(output.u128, u128::default());
        assert_eq!(output.usize, usize::default());

        assert_eq!(output.i8, i8::default());
        assert_eq!(output.i16, i16::default());
        assert_eq!(output.i32, i32::default());
        assert_eq!(output.i64, i64::default());
        assert_eq!(output.i128, i128::default());
        assert_eq!(output.isize, isize::default());
    }
}
