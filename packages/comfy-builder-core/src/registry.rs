use pyo3::prelude::PyModule;
use pyo3::types::{PyDict, PyDictMethods, PyModuleMethods};
use pyo3::{Bound, PyClass, PyResult, Python};

pub trait Registerable: PyClass {}

#[derive(Debug)]
pub struct NodeRegistration {
    #[allow(clippy::type_complexity)]
    inner: fn(
        python: Python,
        module: &Bound<PyModule>,
        class: &Bound<PyDict>,
        display: &Bound<PyDict>,
    ) -> PyResult<()>,
}

impl NodeRegistration {
    pub const fn new<T: Registerable>() -> Self {
        Self {
            inner: |python, module, class, display| {
                module.add_class::<T>()?;

                let type_name = std::any::type_name::<T>();
                let class_name = type_name.split("::").last().unwrap_or(type_name);

                class.set_item(class_name, python.get_type::<T>())?;
                display.set_item(class_name, class_name)?;

                Ok(())
            },
        }
    }

    pub fn register(
        &self,
        python: Python,
        module: &Bound<PyModule>,
        class: &Bound<PyDict>,
        display: &Bound<PyDict>,
    ) -> PyResult<()> {
        (self.inner)(python, module, class, display)
    }
}

inventory::collect!(NodeRegistration);
