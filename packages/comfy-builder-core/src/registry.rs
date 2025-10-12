use crate::node::NodeFunctionProvider;
use pyo3::prelude::PyAnyMethods;
use pyo3::types::{PyCFunction, PyDict, PyDictMethods};
use pyo3::{Bound, PyAny, PyResult, Python};

type FactoryFn = for<'py> fn(
    python: Python<'py>,
) -> PyResult<(Bound<'py, PyCFunction>, Bound<'py, PyCFunction>)>;

#[derive(Debug)]
pub struct NodeRegistration {
    factory: FactoryFn,
}

impl NodeRegistration {
    pub const fn new<T: NodeFunctionProvider>() -> Self {
        Self {
            factory: |python| Ok((T::define_fn(python)?, T::execute_fn(python)?)),
        }
    }

    pub fn create_node<'a, 'py>(
        &self,
        python: Python<'py>,
        decorator: &'a Bound<'py, PyAny>,
        type_fn: &'a Bound<'py, PyAny>,
        comfy_node: &'a Bound<'py, PyAny>,
        module_name: &'static str,
    ) -> PyResult<Bound<'py, PyAny>> {
        let (define, execute) = (self.factory)(python)?;
        let methods = PyDict::new(python);

        methods.set_item("define_schema", decorator.call1((define,))?)?;
        methods.set_item("execute", decorator.call1((execute,))?)?;

        type_fn.call1((format!("{}_node", module_name), (comfy_node,), methods))
    }
}

inventory::collect!(NodeRegistration);
