use crate::types::comfy_type::{AsInput, ComfyType};

impl<'py> AsInput<'py> for bool {
    fn comfy_type() -> ComfyType {
        ComfyType::Boolean
    }
}
