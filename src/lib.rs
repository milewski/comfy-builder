#![allow(unused_imports, unused_variables, unused_mut, unused_unsafe, dead_code)]
mod attributes;
mod node;
mod nodes;
mod tensor;

use crate::node::CustomNode;
use candle_core::backend::BackendDevice;
use nodes::resize_image::ResizeImage;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{Python, pymodule};
use rayon::prelude::*;
use resize::px::RGB;
use resize::{Pixel, PixelFormat, Type};

fn register_node<'a, T: CustomNode<'a>>(
    python: Python,
    module: &'a Bound<'a, PyModule>,
) -> PyResult<()> {
    module.add_class::<T>()?;

    // NODE_CLASS_MAPPINGS
    let node_class_mappings = PyDict::new(python);
    node_class_mappings.set_item("Example", python.get_type::<T>())?;

    // NODE_DISPLAY_NAME_MAPPINGS
    let node_display_name_mappings = PyDict::new(python);
    node_display_name_mappings.set_item("Demo", "Demo Node")?;

    module.add("NODE_CLASS_MAPPINGS", node_class_mappings)?;
    module.add("NODE_DISPLAY_NAME_MAPPINGS", node_display_name_mappings)?;

    Ok(())
}

#[pymodule]
fn god_nodes(python: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    register_node::<ResizeImage>(python, module)?;
    Ok(())
}
