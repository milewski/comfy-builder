use crate::node::{Node, DataType, InputPort, NodeResult, OutputPort};
use crate::tensor::Tensor;

use candle_core::backend::BackendDevice;
use candle_core::{Device, IndexOp};
use comfyui_macro::{node, Enumerates, InputDerive, OutputPort as OutputPortDerive};
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{Bound, IntoPyObject, PyAny, PyErr, PyResult, Python};
use rayon::prelude::*;
use resize::px::RGB;
use resize::{Pixel, Type};
use std::ops::Deref;

#[derive(Debug, Default, Clone, Enumerates)]
enum Interpolation {
    #[default]
    Lanczos3,
    Point,
    Triangle,
    Catrom,
    Mitchell,
    BSpline,
    Gaussian,
}

impl From<Interpolation> for Type {
    fn from(value: Interpolation) -> Self {
        match value {
            Interpolation::Lanczos3 => Type::Lanczos3,
            Interpolation::Point => Type::Point,
            Interpolation::Triangle => Type::Triangle,
            Interpolation::Catrom => Type::Catrom,
            Interpolation::Mitchell => Type::Mitchell,
            Interpolation::BSpline => Type::BSpline,
            Interpolation::Gaussian => Type::Gaussian,
        }
    }
}

#[derive(Debug, InputDerive)]
pub struct Input {
    #[attribute(min = 0, step = 1, default = 1024)]
    width: usize,

    #[attribute(min = 0, step = 1, default = 1024)]
    height: usize,

    image: Tensor,

    #[attribute(enum)]
    interpolation: Interpolation,
}

#[derive(OutputPortDerive)]
pub struct Output {
    width: usize,
    height: usize,
    image: Tensor,
}

#[node]
pub struct ResizeImage;

impl<'a> Node<'a> for ResizeImage {
    type In = Input;
    type Out = Output;

    const CATEGORY: &'static str = "God Nodes / Image";

    const DESCRIPTION: &'static str = r#"
        A full descriptive description about `what` this node is supposed to do.
        This node is extremely versatile you can do whatever you want it is kind magical
    "#;

    fn execute(&self, input: Self::In) -> NodeResult<'a, Self> {
        let device = Device::Cpu;
        let (batch, height, width, channels) = input.image.dims4()?;

        assert_eq!(channels, 3, "Only 3-channel (RGB) input.images supported");

        let output_pixels_per_image = input.width * input.height * channels;

        let resized_data: Vec<Vec<f32>> = (0..batch)
            .into_par_iter()
            .flat_map(|index| input.image.i(index))
            .flat_map(|tensor| tensor.flatten_all().and_then(|data| data.to_vec1()))
            .flat_map(|data| {
                self.resize(
                    &data,
                    width,
                    height,
                    input.width,
                    input.height,
                    output_pixels_per_image,
                    input.interpolation.clone().into(),
                )
            })
            .collect();

        let mut data = Vec::with_capacity(batch * output_pixels_per_image);

        for chunk in resized_data {
            data.extend_from_slice(&chunk);
        }

        Ok(Output {
            image: Tensor::from_raw(data, &[batch, input.height, input.width, channels], &device)?,
            height: input.height,
            width: input.width,
        })
    }
}

impl ResizeImage {
    fn resize(
        &self,
        input: &[f32],
        origin_width: usize,
        origin_height: usize,
        target_width: usize,
        target_height: usize,
        output_pixels_per_image: usize,
        r#type: Type,
    ) -> resize::Result<Vec<f32>> {
        let mut output = vec![0.0f32; output_pixels_per_image];

        assert_eq!(input.len() % 3, 0, "Input length must be divisible by 3");
        assert_eq!(output.len() % 3, 0, "Output length must be divisible by 3");

        let mut resizer = resize::new(
            origin_width,
            origin_height,
            target_width,
            target_height,
            Pixel::RGBF32,
            r#type,
        )?;

        let input_rgb: &[RGB<f32>] = unsafe {
            std::slice::from_raw_parts(input.as_ptr() as *const RGB<f32>, input.len() / 3)
        };

        let output_rgb: &mut [RGB<f32>] = unsafe {
            std::slice::from_raw_parts_mut(output.as_mut_ptr() as *mut RGB<f32>, output.len() / 3)
        };

        resizer.resize(input_rgb, output_rgb)?;

        Ok(output)
    }
}
