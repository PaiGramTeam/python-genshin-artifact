use pyo3::exceptions::{PyException, PyValueError};
use pyo3::{pyclass, PyErr};

#[pyclass(extends=PyValueError)]
#[derive(Debug, Clone)]
pub struct JSONDecodeError {}

impl JSONDecodeError {
    pub(crate) fn new_err() -> PyErr {
        PyErr::new::<Self, ()>(())
    }
}