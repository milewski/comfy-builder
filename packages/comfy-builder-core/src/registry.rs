use pyo3::prelude::{PyAnyMethods, PyModule};
use pyo3::types::{PyCFunction, PyDict, PyDictMethods, PyModuleMethods};
use pyo3::{
    Bound, FromPyObject, Py, PyAny, PyClass, PyResult, Python, pyfunction, wrap_pyfunction,
};
use std::marker::PhantomData;
use crate::{ExtractNodeFunctions, Node};

pub trait Registerable {}

#[derive(Debug)]
pub struct NodeRegistration {
    #[allow(clippy::type_complexity)]
    inner: fn(
        python: Python,
        // module: &Bound<PyModule>,
        // class: &Bound<PyDict>,
        // display: &Bound<PyDict>,
    ) -> PyResult<(Bound<PyCFunction>, Bound<PyCFunction>)>,
}

impl NodeRegistration {
    pub const fn new<'a, T: ExtractNodeFunctions>() -> Self {
        Self {
            inner: |python: Python| {
                let define_schema = T::define_function(python)?;
                let execute = T::run_function(python)?;

                Ok((define_schema, execute))
            },
        }
    }

    // pub fn register(
    //     &self,
    //     python: Python,
    //     module: &Bound<PyModule>,
    //     class: &Bound<PyDict>,
    //     display: &Bound<PyDict>,
    // ) -> PyResult<()> {
    //     (self.inner)(python, module, class, display)
    // }

    pub fn register_v2<'a>(
        &self,
        python: Python<'a>,
        decorator: &Bound<'a, PyAny>,
        type_fn: &Bound<'a, PyAny>,
        comfy_node: &Bound<'a, PyAny>,
    ) -> PyResult<Bound<'a, PyAny>> {
        let (define, execute) = (self.inner)(python)?;

        let methods = PyDict::new(python);
        let define_schema_function = decorator.call1((define,))?;
        let execute_function = decorator.call1((execute,))?;

        methods.set_item("define_schema", define_schema_function)?;
        methods.set_item("execute", execute_function)?;

        type_fn.call1(("RustNode", (comfy_node,), methods))
    }
}

inventory::collect!(NodeRegistration);
