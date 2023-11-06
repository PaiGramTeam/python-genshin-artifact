use pyo3::exceptions::PyValueError;
use pyo3::{pyclass, PyErr};

#[pyclass(extends=PyValueError)]
#[derive(Debug, Clone)]
pub struct ValidationError  {}

impl ValidationError  {
    pub(crate) fn new_err() -> PyErr {
        PyErr::new::<Self, ()>(())
    }
}