mod node;

use image::RgbImage;
use image::imageops::FilterType;
use pyo3::prelude::*;
use pyo3::types::{PyByteArray, PyDict, PyTuple, PyType};
use pyo3::{IntoPyObjectExt, Python, pymodule};
use pyo3_tch::PyTensor;
use pyo3_tch::tch::{Device, Kind, Tensor};
use std::borrow::Cow;
use zerocopy::FromZeros;
use pyo3_tch::tch::IndexOp;

struct Int {
    default: isize,
    min: isize,
    max: isize,
}

#[pyclass]
#[derive(Default)]
struct Example;

#[pymethods]
impl Example {
    #[new]
    fn new() -> Self {
        Example
    }

    #[classmethod]
    fn INPUT_TYPES<'a>(cls: &Bound<'a, PyType>) -> pyo3::Bound<'a, PyDict> {
        let py = cls.py();
        let out = PyDict::new(py);
        let required = PyDict::new(py);

        // image
        required.set_item("image", ("IMAGE",)).unwrap();

        // width
        let width = PyDict::new(py);
        width.set_item("default", 1024.0).unwrap();
        width.set_item("min", 0).unwrap();
        width.set_item("max", 1024).unwrap();
        width.set_item("step", 5).unwrap();

        required.set_item("width", ("INT", width)).unwrap();

        // width
        let height = PyDict::new(py);
        height.set_item("default", 1024.0).unwrap();
        height.set_item("min", 0).unwrap();
        height.set_item("max", 1024).unwrap();
        height.set_item("step", 5).unwrap();

        required.set_item("height", ("INT", height)).unwrap();

        out.set_item("required", required).unwrap();

        out
    }

    #[classattr]
    fn DESCRIPTION() -> &'static str {
        "test"
    }

    #[classattr]
    fn RETURN_TYPES() -> (&'static str, &'static str) {
        ("INT", "IMAGE")
    }

    #[classattr]
    fn FUNCTION() -> &'static str {
        "test"
    }

    #[classattr]
    fn CATEGORY() -> &'static str {
        "Example"
    }

    #[classmethod]
    fn example(py: &Bound<PyType>) {}

    #[classmethod]
    fn test<'a>(
        py: &'a Bound<PyType>,
        image: PyTensor,
        width: usize,
        height: usize,
    ) -> (usize, PyTensor) {
        println!("RECEIVED: W {:?}, H {:?}", width, height);

        // // image.to_device(Device::Cpu).to_kind(Kind::Float);
        // let flat: Vec<u8> = image.contiguous().view(-1).try_into().unwrap();
        //
        // let image_rgb = RgbImage::from_raw(width as u32, height as u32, flat).unwrap();
        //
        // let resized: RgbImage =
        //     image::imageops::resize(&image_rgb, width as u32, height as u32, FilterType::CatmullRom);
        //
        // let (w, h) = resized.dimensions();
        // let mut data: Vec<f32> = Vec::with_capacity((w * h * 3) as usize);
        //
        // // CHW order: channels first
        // for c in 0..3 {
        //     for y in 0..h {
        //         for x in 0..w {
        //             let pixel = resized.get_pixel(x, y);
        //             let value = pixel[c] as f32 / 255.0; // normalize 0..1
        //             data.push(value);
        //         }
        //     }
        // }
        //
        // let tensor = Tensor::from_slice(&data)
        //     .view([1, h as i64, w as i64]);

        let tensor = image.to_device(Device::Cpu).to_kind(Kind::Uint8);
        let shape = tensor.size();
        let (c, h, w) = (shape[0] as u32, shape[1] as u32, shape[2] as u32);

        // 2. CHW → HWC → RgbImage
        let flat: Vec<u8> = tensor
            .permute(&[1, 2, 0]) // CHW → HWC
            .contiguous()
            .view([-1])
            .to_kind(Kind::Uint8)
            .try_into()
            .unwrap();

        let image_rgb = RgbImage::from_raw(w, h, flat).unwrap();

        // 3. Resize
        let resized: RgbImage = image::imageops::resize(&image_rgb, width as u32, height as u32, FilterType::CatmullRom);

        // 4. HWC → CHW → Vec<f32>
        let (w, h) = resized.dimensions();
        let mut data: Vec<f32> = Vec::with_capacity((w*h*3) as usize);
        for y in 0..h {
            for x in 0..w {
                let pixel = resized.get_pixel(x, y);
                data.push(pixel[0] as f32 / 255.0);
                data.push(pixel[1] as f32 / 255.0);
                data.push(pixel[2] as f32 / 255.0);
            }
        }
        // 5. Vec<f32> → Tensor
        let tensor_out = Tensor::from(&data[..]).view([3, h as i64, w as i64]);

        (width + height, PyTensor(tensor_out))
    }
}

#[pymodule]
fn super_node(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Example>()?;

    // NODE_CLASS_MAPPINGS
    let node_class_mappings = PyDict::new(py);
    node_class_mappings.set_item("Example", py.get_type::<Example>())?;
    m.add("NODE_CLASS_MAPPINGS", node_class_mappings)?;

    // NODE_DISPLAY_NAME_MAPPINGS
    let node_display_name_mappings = PyDict::new(py);
    node_display_name_mappings.set_item("Demo", "Demo Node")?;
    m.add("NODE_DISPLAY_NAME_MAPPINGS", node_display_name_mappings)?;

    Ok(())
}
