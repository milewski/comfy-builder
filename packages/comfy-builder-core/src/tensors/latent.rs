use crate::tensors::image::Image;
use candle_core::{Device, WithDType};
use numpy::Element;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::PyDict;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, PyResult, Python};

#[derive(Clone, Debug)]
pub struct Latent<T: Element + WithDType = f32> {
    samples: Image<T>,
    noise_mask: Option<Image<T>>,
}

impl<T: Element + WithDType> Latent<T> {
    pub fn new(any: Bound<PyAny>) -> PyResult<Self> {
        let dict = any.downcast::<PyDict>()?;

        let samples = dict
            .get_item("samples")
            .map(|samples| Image::<T>::new(samples, &Device::Cpu))??;

        let noise_mask = dict
            .get_item("noise_mask")
            .map(|noise| Image::<T>::new(noise, &Device::Cpu))?
            .ok();

        Ok(Self {
            samples,
            noise_mask,
        })
    }
}

impl<'py, T: Element + WithDType> IntoPyObject<'py> for Latent<T> {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let dic = PyDict::new(py);

        dic.set_item("samples", self.samples.into_pyobject(py)?)?;

        if let Some(noise) = self.noise_mask {
            dic.set_item("noise_mask", noise)?;
        }

        Ok(dic.into_any())
    }
}

impl<'py, T: Element + WithDType> FromPyObject<'py> for Latent<T> {
    fn extract_bound(object: &Bound<'py, PyAny>) -> PyResult<Self> {
        Latent::new(object.extract::<Bound<'py, PyAny>>()?)
    }
}

impl<T: Element + WithDType> From<Image<T>> for Latent<T> {
    fn from(tensor: Image<T>) -> Self {
        Latent {
            samples: tensor,
            noise_mask: None,
        }
    }
}
