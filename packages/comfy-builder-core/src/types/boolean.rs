use crate::types::{ComfyNativeType, IntoDict};
use crate::{ComfyDataTypes, ToComfyType};

impl<'py> ComfyNativeType<'py> for bool {}

impl<'py> IntoDict<'py> for bool {

}

impl ToComfyType for bool {
    fn comfy_type() -> ComfyDataTypes {
        ComfyDataTypes::Boolean
    }
}
