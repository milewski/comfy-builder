use candle_core::{Device, Tensor, WithDType};
use numpy::{Element, PyArray, PyArrayDyn, PyArrayMethods, PyUntypedArrayMethods};
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::PyAnyMethods;
use pyo3::{Bound, IntoPyObject, PyAny, PyErr, PyResult, Python};
use std::marker::PhantomData;
use std::ops::Deref;

#[derive(Clone, Debug)]
pub struct TensorWrapper<T = f32>
where
    T: Element + WithDType,
{
    pub tensor: Tensor,
    _marker: PhantomData<T>,
}

impl<T> TensorWrapper<T>
where
    T: Element + WithDType,
{
    pub fn new<'py>(py_any: &Bound<'py, PyAny>, device: &Device) -> Self {
        let tensor = Self::torch_to_candle(py_any, device);

        Self {
            tensor,
            _marker: PhantomData,
        }
    }

    fn torch_to_candle<'py>(torch_tensor: &Bound<'py, PyAny>, device: &Device) -> Tensor {
        let mut np = torch_tensor.call_method0("numpy").unwrap();

        let mut arr = np.downcast::<PyArrayDyn<T>>().unwrap();

        if !arr.is_contiguous() {
            np = np.call_method0("copy").unwrap();
            arr = np.downcast::<PyArrayDyn<T>>().unwrap();
        }

        let shape = arr.shape().to_vec();
        let data = arr.to_vec().unwrap();

        let tensor = Tensor::from_vec(data, shape, device).unwrap();

        tensor
    }

    /// The dimension size for this tensor on each axis.
    pub fn dims(&self) -> &[usize] {
        self.tensor.dims()
    }

    pub fn from_tensor(tensor: Tensor) -> Self {
        Self {
            tensor,
            _marker: PhantomData,
        }
    }

    pub fn into_tensor(self) -> Tensor {
        self.tensor
    }
}

impl<'py, T> IntoPyObject<'py> for TensorWrapper<T>
where
    T: Element + WithDType,
{
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        let tensor = self.into_tensor();
        let shape = tensor.dims();

        let data: Vec<T> = tensor
            .flatten_all()
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?
            .to_vec1::<_>()
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;

        let array = PyArray::from_iter(py, data)
            .reshape(shape)
            .map_err(|e| PyErr::new::<PyRuntimeError, _>(e.to_string()))?;

        let torch = py.import("torch")?;
        let tensor = torch.getattr("tensor")?.call1((array,))?;

        Ok(tensor)
    }
}

// impl<T> TensorWrapper<T>
// where
//     T: Element + WithDType,
// {
//     pub fn to_py_tensor(self, py: Python) -> PyResult<Bound<PyAny>> {
//         let data = self.into_pyobject(py)?;
//
//         let torch = py.import("torch")?;
//         torch.getattr("tensor")?.call1((data,))
//     }
// }

impl Deref for TensorWrapper<f32> {
    type Target = Tensor;

    fn deref(&self) -> &Self::Target {
        &self.tensor
    }
}
