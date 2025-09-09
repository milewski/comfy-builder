use pyo3::{pymodule, Python};
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[pymodule]
fn nodes(py: Python, m: &PyModule) -> PyResult<()> {
    // expose the class
    m.add_class::<RustNode>()?;

    /* ----- expose the two dictionaries ---- */
    // Build a Python dict in Rust
    let node_types = PyDict::new(py);return
    node_types.set_item("RustNode", py.get_type::<RustNode>())?;

    // For display names you can use a normal Rust HashMap and convert it
    let display_map = std::collections::HashMap::from([
        ("RustNode".to_string(), "Rust Node".to_string()),
    ]);

    let node_display_names = display_map
        .into_iter()
        .map(|(k, v)| (k.into_py(py), v.into_py(py)))
        .collect::<PyDict>();

    // expose them as module level constants
    m.add("NODE_TYPES", node_types)?;
    m.add("NODE_DISPLAY_NAMES", node_display_names)?;

    Ok(())
}