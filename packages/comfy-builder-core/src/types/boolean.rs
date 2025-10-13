use crate::ComfyDataTypes;
use crate::types::{ComfyNativeType, IntoDict};

pub struct Boolean;

impl<'py> ComfyNativeType<'py> for Boolean {}

impl<'py> IntoDict<'py> for Boolean {
    fn to_native_type() -> ComfyDataTypes {
        ComfyDataTypes::Boolean
    }
}
