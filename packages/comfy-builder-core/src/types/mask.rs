use crate::types::image::Image;
use crate::types::IntoDict;
use candle_core::shape::ShapeWithOneHole;
use candle_core::{Device, Tensor as CandleTensor, WithDType};
use numpy::Element;
use pyo3::prelude::PyAnyMethods;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::ops::Deref;
use crate::types::comfy_type::{ComfyType, ToComfyType};

#[derive(Clone, Debug)]
pub struct Mask<T = f32>(Image<T>);

impl<'py> ToComfyType<'py> for Mask<f32> {
    fn comfy_type() -> ComfyType {
        ComfyType::Mask
    }
}

impl<'py> IntoDict<'py> for Mask<f32> {}

impl<T: Element + WithDType> Deref for Mask<T> {
    type Target = CandleTensor;

    fn deref(&self) -> &Self::Target {
        self.0.inner_tensor()
    }
}

impl<'py, T: Element + WithDType> IntoPyObject<'py> for Mask<T> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        self.0.into_pyobject(py)
    }
}

impl<T: Element + WithDType, S: ShapeWithOneHole> TryFrom<(Vec<T>, S, &Device)> for Mask<T> {
    type Error = candle_core::Error;

    fn try_from(value: (Vec<T>, S, &Device)) -> Result<Self, Self::Error> {
        Ok(Mask(Image::from_raw(value.0, value.1, value.2)?))
    }
}

impl<'py, T: Element + WithDType> FromPyObject<'py> for Mask<T> {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        Ok(Image::new(object.extract::<Bound<'py, PyAny>>()?, &Device::Cpu)?.into())
    }
}

impl<T: Element + WithDType> From<Image<T>> for Mask<T> {
    fn from(tensor: Image<T>) -> Self {
        Mask(tensor)
    }
}
