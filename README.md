# ComfyUI Node Builder

A Rust framework for building custom nodes for ComfyUI with enhanced performance and simplified development workflow.

> Note: Work in Progress
While the basic functionality works, there are still several features present in the Python version that have not yet been fully implemented in this Rust version. 
The framework is actively being developed and will continue to expand its feature set to match the full capabilities of the original ComfyUI node system.

## Overview

ComfyUI Node Builder provides a powerful, type-safe framework for creating custom nodes in ComfyUI using Rust.
Built with performance in mind, this framework offers faster execution times and a more streamlined development experience compared to traditional Python-based node development.

## Quick Start

Basic Node Example

```rust
use comfyui_plugin::prelude::*;

#[derive(NodeInput)]
pub struct Input {
    left: usize,
    right: usize,
}

#[derive(NodeOutput)]
pub struct Output {
    sum: usize,
}

#[node]
pub struct Sum;

impl<'a> Node<'a> for Sum {
    type In = Input;
    type Out = Output;

    const CATEGORY: &'static str = "MyNode / Math";

    const DESCRIPTION: &'static str = r#"
        Sums the left input with the right input.
    "#;

    fn execute(&self, input: Self::In) -> NodeResult<'a, Self> {
        Ok(Output {
            sum: input.left + input.right
        })
    }
}

// Auto Register / Export every custom node created automatically
comfyui_macro::register!();
```

## Installation
- Add the dependency to your Cargo.toml:

```toml
[dependencies]
comfyui-plugin = "0.1"
```

## Join the telegram group

If you'd like to discuss this project, ComfyUI, or generative AI in general, join our Telegram community:
https://t.me/thelatentspace

## Contributing
Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests to improve this framework.

## License

[MIT](LICENSE) © [Rafael Milewski](https://github.com/milewski)
