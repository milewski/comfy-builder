use crate::types::comfy_type::{ComfyType, AsInput};

impl<'py> AsInput<'py> for bool {
    fn comfy_type() -> ComfyType {
        ComfyType::Boolean
    }
}
