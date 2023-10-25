use crate::applications::input::common::PyCharacterInterface;
use pyo3::prelude::*;

#[pyclass]
pub struct CalculatorConfig {
    pub character: PyCharacterInterface,
}

#[pymethods]
impl CalculatorConfig {
    #[new]
    pub fn py_new(character: PyCharacterInterface) -> PyResult<Self> {
        Ok(Self { character })
    }
}
