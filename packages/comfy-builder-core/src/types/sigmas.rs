use crate::types::comfy_type::{AsInput, AsOutput, ComfyType};
use crate::types::image::{tensor_to_pytensor, torch_to_candle};
use candle_core::{DType, Device, Shape, Tensor, WithDType};
use numpy::Element;
use pyo3::types::PyAnyMethods;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::marker::PhantomData;

pub struct Sigmas<T = f32> {
    tensor: Tensor,
    inner: PhantomData<T>,
}

impl<T> Sigmas<T> {
    pub fn zeros<S: Into<Shape>>(shape: S, dtype: DType) -> candle_core::Result<Self> {
        Ok(Self {
            tensor: Tensor::zeros(shape.into(), dtype, &Device::Cpu)?,
            inner: PhantomData,
        })
    }

    pub fn blank() -> candle_core::Result<Self> {
        Self::zeros((0, 0), DType::F32)
    }
}

impl<'py, T: Element + WithDType> FromPyObject<'py> for Sigmas<T> {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Sigmas {
            tensor: torch_to_candle::<T>(object.extract::<Bound<'py, PyAny>>()?, &Device::Cpu)?,
            inner: PhantomData,
        })
    }
}

impl<'py> AsInput<'py> for Sigmas {
    fn comfy_type() -> ComfyType {
        ComfyType::Sigmas
    }
}

impl<'py, T: Element + WithDType> IntoPyObject<'py> for Sigmas<T> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, python: Python<'py>) -> Result<Self::Output, Self::Error> {
        tensor_to_pytensor::<T>(python, self.tensor)
    }
}

impl<'py> AsOutput<'py> for Sigmas {}
