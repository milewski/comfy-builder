use crate::{ComfyDataTypes, EnumVariants};
use crate::types::{ComfyNativeType, IntoDict};
use pyo3::types::{PyDict, PyDictMethods};
use pyo3::{Bound, PyAny, PyResult};
use std::marker::PhantomData;

pub struct Enum<T: EnumVariants> {
    inner: PhantomData<T>,
}

impl<'py, T: EnumVariants> ComfyNativeType<'py> for Enum<T> {}

impl<'py, T: EnumVariants> IntoDict<'py> for Enum<T> {
    fn into_dict(dict: &mut Bound<'py, PyDict>, _: &Bound<'py, PyAny>) -> PyResult<()> {
        dict.set_item("options", T::variants())?;

        Ok(())
    }

    fn to_native_type() -> ComfyDataTypes {
        ComfyDataTypes::Enum
    }
}
