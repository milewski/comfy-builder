use crate::tensors::image::Image;
use candle_core::shape::ShapeWithOneHole;
use candle_core::{Device, WithDType};
use numpy::Element;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyDict;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct Latent<T: Element + WithDType = f32> {
    samples: Image<T>,
    noise_mask: Option<Image<T>>,
}

impl<T: Element + WithDType> Latent<T> {
    pub fn new(any: Bound<PyAny>) -> PyResult<Self> {
        let dict = any.downcast::<PyDict>()?;
        let samples = dict.get_item("samples")?;

        let image = Image::<T>::new(&samples, &Device::Cpu);

        Ok(Self {
            samples: image,
            noise_mask: None,
        })
    }
}

// impl<T: Element + WithDType> Deref for Latent<T> {
//     type Target = CandleTensor;
//
//     fn deref(&self) -> &Self::Target {
//         self.0.inner_tensor()
//     }
// }

impl<'py, T: Element + WithDType> IntoPyObject<'py> for Latent<T> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dic = PyDict::new(py);

        dic.set_item("samples", self.samples.into_pyobject(py)?)?;

        Ok(dic.into_any())
    }
}

impl<T: Element + WithDType, S: ShapeWithOneHole> TryFrom<(Vec<T>, S, &Device)> for Latent {
    type Error = candle_core::Error;

    fn try_from(value: (Vec<T>, S, &Device)) -> Result<Self, Self::Error> {
        todo!()
        // Ok(Latent(Image::from_raw(value.0, value.1, value.2)?))
    }
}

impl<'py, T: Element + WithDType> FromPyObject<'py> for Latent<T> {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        Latent::new(object.extract::<Bound<'py, PyAny>>()?)
    }
}

impl<T: Element + WithDType> From<Image<T>> for Latent<T> {
    fn from(tensor: Image<T>) -> Self {
        todo!()
        // Latent(tensor)
    }
}
