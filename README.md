# ComfyUI Node Builder

A Rust framework for building custom nodes for ComfyUI with enhanced performance and simplified development workflow.

> **⚠️ Work in Progress**, 
> While the core functionality is operational, some features from the Python version are still being implemented. The framework is actively developed and aims to eventually match the full capabilities of the original ComfyUI node system.

## Quick Start

Start from this template: [comfy-builder-template](https://github.com/milewski/comfy-builder-template)

## Overview

ComfyUI Node Builder provides a powerful, type-safe framework for creating custom nodes in ComfyUI using Rust.

Built with performance in mind, this framework offers faster execution times and a more streamlined development experience compared to traditional Python-based node development.

## Quick Start

Basic Node Example

```rust
use comfyui_plugin::prelude::*;

#[derive(NodeInput)]
pub struct Input {
    a: usize,
    b: usize,
}

#[derive(NodeOutput)]
pub struct Output {
    sum: usize,
}

#[node(
    category = "MyNode / Math",
    description = "Sums a + b inputs."
)]
pub struct Sum;

impl<'a> Node<'a> for Sum {
    type In = Input;
    type Out = Output;
    type Error = Box<dyn Error + Send + Sync>;

    fn execute(&self, input: Self::In) -> Result<Self::Out, Self::Error> {
        Ok(Output {
            sum: input.a + input.b
        })
    }
}

boostrap!(api_version: "latest");
```

## Join the telegram group

If you'd like to discuss this project, ComfyUI, or generative AI in general, join our Telegram community:
https://t.me/thelatentspace

## Contributing
Contributions are welcome! Please feel free to submit issues, feature requests, or pull requests to improve this framework.

## License

[MIT](LICENSE) © [Rafael Milewski](https://github.com/milewski)
