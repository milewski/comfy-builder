use crate::types::comfy_type::{AsInput, ComfyType};
use candle_core::shape::ShapeWithOneHole;
use candle_core::{Device, Tensor as CandleTensor, WithDType};
use numpy::{Element, PyArray, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::PyAnyMethods;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Image<T: Element + WithDType> {
    tensor: CandleTensor,
    marker: PhantomData<T>,
}

impl<'py, T: Element + WithDType> FromPyObject<'py> for Image<T> {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        Image::new(object.extract::<Bound<'py, PyAny>>()?, &Device::Cpu)
    }
}

impl<'py, T: Element + WithDType> AsInput<'py> for Image<T> {
    fn comfy_type() -> ComfyType {
        ComfyType::Image
    }
}

impl<T: Element + WithDType> Image<T> {
    pub fn new(any: Bound<PyAny>, device: &Device) -> PyResult<Self> {
        Ok(Self {
            tensor: Self::torch_to_candle(any, device)?,
            marker: PhantomData,
        })
    }

    fn torch_to_candle(torch_tensor: Bound<PyAny>, device: &Device) -> PyResult<CandleTensor> {
        let mut numpy = torch_tensor.call_method0("numpy")?;

        let mut array = numpy.downcast::<PyArrayDyn<T>>()?;

        if !array.is_contiguous() {
            numpy = numpy.call_method0("copy")?;
            array = numpy.downcast::<PyArrayDyn<T>>()?;
        }

        let shape = array.shape().to_vec();
        let data = array.to_vec()?;

        CandleTensor::from_vec(data, shape, device)
            .map_err(|error| PyRuntimeError::new_err(format!("Execution failed: {}", error)))
    }

    pub fn from_tensor(tensor: CandleTensor) -> Self {
        Self {
            tensor,
            marker: PhantomData,
        }
    }

    pub fn into_tensor(self) -> CandleTensor {
        self.tensor
    }

    pub fn inner_tensor(&self) -> &CandleTensor {
        &self.tensor
    }
}

impl<'py, T: Element + WithDType> IntoPyObject<'py> for Image<T> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, python: Python<'py>) -> Result<Self::Output, Self::Error> {
        let tensor = self.into_tensor();
        let shape = tensor.dims();

        let data: Vec<T> = tensor
            .flatten_all()
            .map_err(|error| PyErr::new::<PyRuntimeError, _>(error.to_string()))?
            .to_vec1::<_>()
            .map_err(|error| PyErr::new::<PyRuntimeError, _>(error.to_string()))?;

        let array = PyArray::from_iter(python, data)
            .reshape(shape)
            .map_err(|error| PyErr::new::<PyRuntimeError, _>(error.to_string()))?;

        let torch = python.import("torch")?;
        let tensor = torch.getattr("tensor")?.call1((array,))?;

        Ok(tensor)
    }
}

impl<T: Element + WithDType> Image<T> {
    pub fn from_raw<U: ShapeWithOneHole>(data: Vec<T>, shape: U, device: &Device) -> candle_core::Result<Image<T>> {
        Ok(Image::from_tensor(CandleTensor::from_vec(data, shape, device)?))
    }
}

impl<T: Element + WithDType, S: ShapeWithOneHole> TryFrom<(Vec<T>, S, &Device)> for Image<T> {
    type Error = candle_core::Error;

    fn try_from(value: (Vec<T>, S, &Device)) -> Result<Self, Self::Error> {
        Image::from_raw(value.0, value.1, value.2)
    }
}

impl<T: Element + WithDType> Deref for Image<T> {
    type Target = CandleTensor;

    fn deref(&self) -> &Self::Target {
        &self.tensor
    }
}
