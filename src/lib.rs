#![allow(unused_imports, unused_variables, unused_mut, unused_unsafe, dead_code)]
mod node;
mod resize_image;
mod tensor;
mod attributes;

use crate::node::CustomNode;
use crate::resize_image::ResizeImage;
use candle_core::backend::BackendDevice;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::{Python, pymodule};
use rayon::prelude::*;
use resize::px::RGB;
use resize::{Pixel, PixelFormat, Type};

// #[pyclass]
// #[derive(Default)]
// struct Example;
// 
// #[pymethods]
// impl Example {
//     #[new]
//     fn new() -> Self {
//         Example
//     }
// 
//     #[classmethod]
//     #[pyo3(name = "INPUT_TYPES")]
//     fn input_types<'a>(cls: &Bound<'a, PyType>) -> Bound<'a, PyDict> {
//         let py = cls.py();
//         let out = PyDict::new(py);
//         let required = PyDict::new(py);
// 
//         // image
//         required.set_item("image", ("IMAGE",)).unwrap();
// 
//         // width
//         let width = PyDict::new(py);
//         width.set_item("default", 1024).unwrap();
//         width.set_item("min", 0).unwrap();
//         // width.set_item("max", 1024).unwrap();
//         width.set_item("step", 1).unwrap();
// 
//         required.set_item("width", ("INT", width)).unwrap();
// 
//         // width
//         let height = PyDict::new(py);
//         height.set_item("default", 1024).unwrap();
//         height.set_item("min", 0).unwrap();
//         // height.set_item("max", 1024).unwrap();
//         height.set_item("step", 1).unwrap();
// 
//         required.set_item("height", ("INT", height)).unwrap();
// 
//         out.set_item("required", required).unwrap();
// 
//         out
//     }
// 
//     #[classattr]
//     #[pyo3(name = "DESCRIPTION")]
//     fn description() -> &'static str {
//         "test"
//     }
// 
//     #[classattr]
//     #[pyo3(name = "RETURN_TYPES")]
//     fn return_types() -> (&'static str, &'static str) {
//         ("INT", "IMAGE")
//     }
// 
//     #[classattr]
//     #[pyo3(name = "FUNCTION")]
//     fn function() -> &'static str {
//         "test_new"
//     }
// 
//     #[classattr]
//     #[pyo3(name = "CATEGORY")]
//     fn category() -> &'static str {
//         "Example"
//     }
// 
//     #[classmethod]
//     fn test_new<'a>(
//         py: &'a Bound<PyType>,
//         image: Bound<'a, PyAny>,
//         height: usize,
//         width: usize,
//     ) -> (usize, Bound<'a, PyAny>) {
//         let device = Device::Cpu;
//         let tensor: TensorWrapper<f32> = TensorWrapper::new(&image, &device);
//         let (batch, orig_h, orig_w, channels) = tensor.tensor.dims4().unwrap();
//         assert_eq!(channels, 3, "Only 3-channel (RGB) images supported");
// 
//         let output_pixels_per_image = height * width * channels;
// 
//         // 🔥 Parallel: each thread returns its own resized Vec<f32>
//         let all_resized_chunks: Vec<Vec<f32>> = py.py().detach(|| {
//             (0..batch)
//                 .into_par_iter()
//                 .map(|b| {
//                     let img_tensor = tensor.tensor.i(b).unwrap();
//                     let img_data: Vec<f32> =
//                         img_tensor.flatten_all().unwrap().to_vec1::<f32>().unwrap();
// 
//                     resize_image_fast(
//                         &img_data,
//                         orig_w,
//                         orig_h,
//                         width,
//                         height,
//                         output_pixels_per_image,
//                     )
//                 })
//                 .collect()
//         });
// 
//         // Concatenate all chunks into one Vec — sequential, but fast
//         let total_output_elements = batch * output_pixels_per_image;
//         let mut all_resized_data = Vec::with_capacity(total_output_elements);
//         for chunk in all_resized_chunks {
//             all_resized_data.extend_from_slice(&chunk);
//         }
// 
//         // Build output tensor
//         let new_tensor =
//             Tensor::from_vec(all_resized_data, &[batch, height, width, channels], &device).unwrap();
// 
//         let result_tensor = TensorWrapper::<f32>::from_tensor(new_tensor);
// 
//         (
//             orig_w + orig_h,
//             result_tensor.into_pyobject(py.py()).unwrap(),
//         )
//     }
// }

fn register_node<'a, T: CustomNode<'a>>(
    python: Python,
    module: &'a Bound<'a, PyModule>,
) -> PyResult<()> {
    module.add_class::<T>()?;

    // NODE_CLASS_MAPPINGS
    let node_class_mappings = PyDict::new(python);
    node_class_mappings.set_item("Example", python.get_type::<T>())?;

    // NODE_DISPLAY_NAME_MAPPINGS
    let node_display_name_mappings = PyDict::new(python);
    node_display_name_mappings.set_item("Demo", "Demo Node")?;

    module.add("NODE_CLASS_MAPPINGS", node_class_mappings)?;
    module.add("NODE_DISPLAY_NAME_MAPPINGS", node_display_name_mappings)?;

    Ok(())
}

#[pymodule]
fn god_nodes(python: Python, module: &Bound<'_, PyModule>) -> PyResult<()> {
    register_node::<ResizeImage>(python, module)?;
    Ok(())
}

fn resize_image_fast(
    input: &[f32],
    orig_w: usize,
    orig_h: usize,
    new_w: usize,
    new_h: usize,
    output_pixels_per_image: usize,
) -> Vec<f32> {
    let mut output = vec![0.0f32; output_pixels_per_image];

    assert_eq!(input.len() % 3, 0, "Input length must be divisible by 3");
    assert_eq!(output.len() % 3, 0, "Output length must be divisible by 3");

    let mut resizer = resize::new(
        orig_w,
        orig_h,
        new_w,
        new_h,
        Pixel::RGBF32,
        Type::Lanczos3, // or Type::Nearest for speed
    )
    .unwrap();

    let input_rgb: &[RGB<f32>] =
        unsafe { std::slice::from_raw_parts(input.as_ptr() as *const RGB<f32>, input.len() / 3) };

    let output_rgb: &mut [RGB<f32>] = unsafe {
        std::slice::from_raw_parts_mut(output.as_mut_ptr() as *mut RGB<f32>, output.len() / 3)
    };

    resizer.resize(input_rgb, output_rgb).unwrap();

    output
}
