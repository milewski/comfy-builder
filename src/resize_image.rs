use crate::node::CustomNode;
use crate::tensor::TensorWrapper;
use candle_core::Device;
use pyo3::types::{PyAnyMethods, PyDict, PyDictMethods, PyTuple, PyType};
use pyo3::{Bound, IntoPyObject, PyAny, PyErr, PyResult, Python, pyclass, pymethods};

#[derive(Debug)]
pub struct Input {
    width: usize,
    height: usize,
    image: TensorWrapper,
}

#[derive(Debug)]
pub struct Output {
    width: usize,
    height: usize,
    image: TensorWrapper,
}

#[pyclass]
pub struct ResizeImage {
    device: Device,
}

#[pymethods]
impl ResizeImage {
    #[new]
    fn initialize() -> Self {
        Self::new()
    }

    #[classattr]
    #[pyo3(name = "DESCRIPTION")]
    fn description() -> &'static str {
        Self::DESCRIPTION
    }

    #[classattr]
    #[pyo3(name = "FUNCTION")]
    fn function() -> &'static str {
        "run"
    }

    #[classmethod]
    #[pyo3(name = "INPUT_TYPES")]
    fn input_types<'a>(cls: &Bound<'a, PyType>) -> Bound<'a, PyDict> {
        let py = cls.py();
        let out = PyDict::new(py);
        let required = PyDict::new(py);

        // image
        required.set_item("image", ("IMAGE",)).unwrap();

        // width
        let width = PyDict::new(py);
        width.set_item("default", 1024).unwrap();
        width.set_item("min", 0).unwrap();
        // width.set_item("max", 1024).unwrap();
        width.set_item("step", 1).unwrap();

        required.set_item("width", ("INT", width)).unwrap();

        // width
        let height = PyDict::new(py);
        height.set_item("default", 1024).unwrap();
        height.set_item("min", 0).unwrap();
        // height.set_item("max", 1024).unwrap();
        height.set_item("step", 1).unwrap();

        required.set_item("height", ("INT", height)).unwrap();

        out.set_item("required", required).unwrap();

        out
    }

    #[classattr]
    #[pyo3(name = "RETURN_TYPES")]
    fn return_types(py: Python) -> PyResult<Bound<PyTuple>> {
        ("INT", "IMAGE").into_pyobject(py)
    }

    #[classattr]
    #[pyo3(name = "CATEGORY")]
    fn category() -> &'static str {
        Self::CATEGORY
    }

    #[classmethod]
    #[pyo3(signature = (**kwargs))]
    pub fn run<'a>(py: &'a Bound<PyType>, kwargs: Option<&Bound<PyDict>>) -> impl IntoPyObject<'a> {
        println!("GOT {:?}", kwargs.unwrap().keys());
        let instance = Self::new();
        let output = instance.execute(instance.initialize_input(kwargs));

        output.into_pyobject(py.py()).unwrap()
    }
}

/// A full descriptive description about `what` this node is supposed to do.
/// This node is extremely versatile you can do whatever you want it is kind magical
/// Category: God Nodes / Image
impl<'a> CustomNode<'a> for ResizeImage {
    type In = Input;
    type Out = Output;

    const CATEGORY: &'static str = "God Nodes / Image";

    const DESCRIPTION: &'static str = r#"
        A full descriptive description about `what` this node is supposed to do.
        This node is extremely versatile you can do whatever you want it is kind magical
    "#;

    fn new() -> Self {
        Self {
            device: Device::Cpu,
        }
    }

    fn execute(&self, input: Self::In) -> Self::Out {
        println!("GOT {:?}", input);

        Output {
            image: input.image,
            width: input.width,
            height: input.height,
        }
    }
}

impl<'py> IntoPyObject<'py> for Output {
    type Target = PyTuple;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        (
            self.height.into_pyobject(py)?,
            self.image.into_pyobject(py)?,
        )
            .into_pyobject(py)
    }
}

impl<'a> From<&'a Bound<'a, PyDict>> for Input {
    fn from(kwargs: &'a Bound<'a, PyDict>) -> Self {
        Self {
            image: kwargs
                .get_item("image")
                .unwrap()
                .and_then(|v| v.extract::<Bound<PyAny>>().ok())
                .map(|v| TensorWrapper::new(&v, &Device::Cpu))
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'image'"))
                .unwrap(),

            width: kwargs
                .get_item("width")
                .unwrap()
                .and_then(|v| v.extract::<usize>().ok())
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'width'"))
                .unwrap(),

            height: kwargs
                .get_item("height")
                .unwrap()
                .and_then(|v| v.extract::<usize>().ok())
                .ok_or_else(|| pyo3::exceptions::PyKeyError::new_err("missing or invalid 'height'"))
                .unwrap(),
        }
    }
}
