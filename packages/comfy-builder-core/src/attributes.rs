use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, PyAny, PyResult};
use std::ops::Deref;

/// UNIQUE_ID is the unique identifier of the node, and matches the id property of the node on the client side.
/// It is commonly used in client-server communications.
/// see https://docs.comfy.org/development/comfyui-server/comms_messages#getting-node-id
#[derive(Debug)]
pub struct UniqueId(pub String);

/// PROMPT is the complete prompt sent by the client to the server.
/// See [the prompt object](https://docs.comfy.org/custom-nodes/js/javascript_objects_and_hijacking#prompt) for a full description.
#[derive(Debug)]
pub struct Prompt(pub String);

/// EXTRA_PNGINFO is a dictionary that will be copied into the metadata of any .png files saved.
/// Custom nodes can store additional information in this dictionary for saving (or as a way to communicate with a downstream node).
#[derive(Debug)]
pub struct ExtraPngInfo(pub String);

/// DYNPROMPT is an instance of comfy_execution.graph.DynamicPrompt.
/// It differs from PROMPT in that it may mutate during the course of execution in response to [Node Expansion](https://docs.comfy.org/custom-nodes/backend/expansion).
#[derive(Debug)]
pub struct DynPrompt(pub String);

impl Deref for UniqueId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'py> FromPyObject<'py> for UniqueId {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        object
            .extract::<String>()
            .map(UniqueId)
    }
}

impl<'py> FromPyObject<'py> for Prompt {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        // @todo convert from dict to json string
        Ok(Prompt(object.to_string()))
    }
}

impl<'py> FromPyObject<'py> for ExtraPngInfo {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        // @todo convert from dict to json string
        Ok(ExtraPngInfo(object.to_string()))
    }
}

impl<'py> FromPyObject<'py> for DynPrompt {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        // @todo convert from comfy_execution.graph.DynamicPrompt to a better format
        Ok(DynPrompt(object.to_string()))
    }
}