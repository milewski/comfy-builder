pub use crate::types::IntoDict;
use pyo3::IntoPyObject;
use std::ops::Deref;
use types::comfy_type::ToComfyType;

pub mod node;
pub mod prelude;
pub mod registry;
pub mod types;

pub trait ComfyInput<'py>: ToComfyType<'py> + IntoDict<'py> {}

// impl<'a> TryFrom<Kwargs<'a>> for () {
//     type Error = PyErr;
//
//     fn try_from(value: Kwargs<'a>) -> Result<Self, Self::Error> {
//        Ok(())
//     }
// }
//
// impl<'py> In<'py> for () {
//     fn blueprints(python: Python<'py>, _: &Bound<PyAny>) -> PyResult<Bound<'py, PyList>> {
//         Ok(PyList::empty(python))
//     }
//
//     fn is_list() -> bool {
//         false
//     }
// }

#[macro_export]
macro_rules! set_defaults {
    ($dict:expr, $( $key:expr => $value:expr ),* $(,)?) => {
        $(
            if let Err(_) = $dict.get_item($key) {
                $dict.set_item($key, $value)?;
            }
        )*
    };
}

#[macro_export]
macro_rules! run_node {
    ($node:ident, $input:expr) => {{
        match $node::new().execute($input) {
            Ok(output) => output,
            Err(error) => panic!("`{}` node failed: {}", stringify!($node), error),
        }
    }};
}