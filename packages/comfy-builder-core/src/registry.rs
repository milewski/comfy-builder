use crate::{ExtractNodeFunctions, Node};
use pyo3::prelude::PyAnyMethods;
use pyo3::types::{PyCFunction, PyDict, PyDictMethods};
use pyo3::{Bound, PyAny, PyResult, Python};

pub trait Registerable {}

#[derive(Debug)]
pub struct NodeRegistration {
    inner: fn(python: Python) -> PyResult<(Bound<PyCFunction>, Bound<PyCFunction>)>,
}

impl NodeRegistration {
    pub const fn new<'a, T: ExtractNodeFunctions>() -> Self {
        Self {
            inner: |python: Python| Ok((T::define_function(python)?, T::run_function(python)?)),
        }
    }

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
