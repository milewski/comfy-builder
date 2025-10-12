use crate::types::{ComfyNativeType, IntoDict};

pub struct String;

impl<'py> ComfyNativeType<'py> for String {}

impl<'py> IntoDict<'py> for String {}
