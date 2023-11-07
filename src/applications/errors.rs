use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::pyclass;
use std::fmt;

#[pyclass(extends = PyValueError)]
#[derive(Debug, Clone)]
pub struct ValidationError {
    #[pyo3(get)]
    message: String,
}

#[pymethods]
impl ValidationError {
    #[new]
    pub fn new_err(message: String) -> Self {
        Self { message }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ValidationError: {}", self.message)
    }
}
