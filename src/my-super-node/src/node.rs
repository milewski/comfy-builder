use crate::tensor::TensorWrapper;
use candle_core::Device;
use pyo3::{PyClass, pyclass, pymethods};

#[pyclass]
pub struct ResizeImage {
    device: Device,
}

#[pymethods]
impl ResizeImage {
    #[classattr]
    #[pyo3(name = "DESCRIPTION")]
    fn description() -> &'static str {
        "A full descriptive description about what this node is supposed to do."
    }
}

impl ResizeImage {
    pub fn new() -> ResizeImage {
        Self {
            device: Device::Cpu,
        }
    }

    pub fn execute(&self, tensor: TensorWrapper, height: usize, width: usize) {
        let (batch, orig_h, orig_w, channels) = tensor.dims4().unwrap();
        assert_eq!(channels, 3, "Only 3-channel (RGB) images supported");
    }
}

impl CustomNode for ResizeImage {
    fn category(&self) -> &'static str {
        todo!()
    }
}

pub trait CustomNode: PyClass {
    fn category(&self) -> &'static str;
}
