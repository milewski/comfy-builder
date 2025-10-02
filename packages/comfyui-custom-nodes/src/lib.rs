#![allow(unused_imports, unused_variables, unused_mut, unused_unsafe, dead_code)]
mod nodes;

use crate::nodes::example::Sum;
use candle_core::backend::BackendDevice;
use comfyui_plugin::node::Node;
use nodes::resize_image::ResizeImage;
use once_cell::unsync::Lazy;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{Python, pymodule};
use rayon::prelude::*;
use resize::px::RGB;
use resize::{Pixel, PixelFormat, Type};
use std::sync::Mutex;

fn register_node<'a, T: Node<'a>>(
    python: Python,
    module: &'a Bound<'a, PyModule>,
    class_mappings: &Bound<'a, PyDict>,
    display_mappings: &Bound<'a, PyDict>,
) -> PyResult<()> {
    module.add_class::<T>()?;

    let type_name = std::any::type_name::<T>();
    let class_name = type_name.split("::").last().unwrap_or(type_name);

    class_mappings.set_item(class_name, python.get_type::<T>())?;
    display_mappings.set_item(class_name, class_name)?;

    Ok(())
}

#[pymodule]
fn god_nodes(python: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    let node_class_mappings = PyDict::new(python);
    let node_display_name_mappings = PyDict::new(python);

    register_node::<ResizeImage>(
        python,
        module,
        &node_class_mappings,
        &node_display_name_mappings,
    )?;

    register_node::<Sum>(
        python,
        module,
        &node_class_mappings,
        &node_display_name_mappings,
    )?;

    module.add("NODE_CLASS_MAPPINGS", node_class_mappings)?;
    module.add("NODE_DISPLAY_NAME_MAPPINGS", node_display_name_mappings)?;

    Ok(())
}
