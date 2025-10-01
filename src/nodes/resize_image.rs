use crate::node::{DataType, InputPort, Node, NodeResult, OutputPort};
use crate::tensor::{Mask, Tensor};

use candle_core::backend::BackendDevice;
use candle_core::shape::ShapeWithOneHole;
use candle_core::{Device, IndexOp};
use comfyui_macro::{Enumerates, InputDerive, OutputPort as OutputPortDerive, node};
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{Bound, IntoPyObject, PyAny, PyErr, PyResult, Python};
use rayon::prelude::*;
use resize::Pixel::{GrayF32, RGBF32};
use resize::{PixelFormat, Type};
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

    mask: Option<Mask>,

    #[attribute(enum)]
    interpolation: Interpolation,
}

#[derive(OutputPortDerive)]
pub struct Output {
    image: Tensor,
    mask: Option<Mask>,
    width: usize,
    height: usize,
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

        let mask = if let Some(mask) = input.mask {
            let (batch, mask_height, mask_width) = mask.dims3()?;

            let mask = self.resize_parallel::<1, Mask, _, _, _>(
                &mask,
                batch,
                mask_width,
                mask_height,
                input.width,
                input.height,
                input.interpolation.clone(),
                || GrayF32,
                |batch, width, height, _| (batch, height, width),
            )?;

            Some(mask)
        } else {
            None
        };

        let (batch, height, width, channels) = input.image.dims4()?;

        let image = self.resize_parallel::<3, Tensor, _, _, _>(
            &input.image,
            batch,
            width,
            height,
            input.width,
            input.height,
            input.interpolation.clone(),
            || RGBF32,
            |batch, width, height, channels| (batch, height, width, channels),
        )?;

        Ok(Output {
            image,
            mask,
            height: input.height,
            width: input.width,
        })
    }
}

impl ResizeImage {
    fn resize_parallel<'a, const CHANNELS: usize, Output, Input, Format, Shape>(
        &self,
        image: &Input,
        batch: usize,
        width: usize,
        height: usize,
        target_width: usize,
        target_height: usize,
        interpolation: Interpolation,
        format: fn() -> Format,
        get_shape: fn(batch: usize, width: usize, height: usize, channels: usize) -> Shape,
    ) -> Result<Output, candle_core::Error>
    where
        Input: IndexOp<usize> + Send + Sync,
        Output: Deref<Target = Input>,
        Output: TryFrom<(Vec<f32>, Shape, &'a Device), Error = candle_core::Error>,
        Format: PixelFormat,
        Shape: ShapeWithOneHole,
    {
        let output_pixels_per_image = target_width * target_height * CHANNELS;

        let resized_data: Vec<Vec<f32>> = (0..batch)
            .into_par_iter()
            .flat_map(|batch| image.i(batch))
            .flat_map(|tensor| tensor.flatten_all().and_then(|data| data.to_vec1()))
            .flat_map(|data| {
                self.resize::<CHANNELS, Format>(
                    &data,
                    width,
                    height,
                    target_width,
                    target_height,
                    interpolation.clone().into(),
                    format(),
                )
            })
            .collect();

        let mut data = Vec::with_capacity(batch * output_pixels_per_image);

        for chunk in resized_data {
            data.extend_from_slice(&chunk);
        }

        Output::try_from((
            data,
            get_shape(batch, target_height, target_width, CHANNELS),
            &Device::Cpu,
        ))
    }

    fn resize<const CHANNELS: usize, Format: PixelFormat>(
        &self,
        input: &[f32],
        origin_width: usize,
        origin_height: usize,
        target_width: usize,
        target_height: usize,
        r#type: Type,
        format: Format,
    ) -> resize::Result<Vec<f32>> {
        let output_pixels_per_image = target_width * target_height * CHANNELS;
        let mut output = vec![0.0f32; output_pixels_per_image];

        let mut resizer = resize::new(
            origin_width,
            origin_height,
            target_width,
            target_height,
            format,
            r#type,
        )?;

        let input_rgb: &[Format::InputPixel] = unsafe {
            std::slice::from_raw_parts(
                input.as_ptr() as *const Format::InputPixel,
                input.len() / CHANNELS,
            )
        };

        let output_rgb: &mut [Format::OutputPixel] = unsafe {
            std::slice::from_raw_parts_mut(
                output.as_mut_ptr() as *mut Format::OutputPixel,
                output.len() / CHANNELS,
            )
        };

        resizer.resize(input_rgb, output_rgb)?;

        Ok(output)
    }
}
