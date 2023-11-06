use anyhow::Context;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pythonize::depythonize;

use mona::character::skill_config::CharacterSkillConfig;
use mona_wasm::applications::common::SkillInterface as MonaSkillInterface;


#[pyclass(name = "SkillInterface")]
#[derive(Clone)]
pub struct PySkillInterface {
    #[pyo3(get, set)]
    pub index: usize,
    #[pyo3(get, set)]
    pub config: Option<Py<PyDict>>,
}

#[pymethods]
impl PySkillInterface {
    #[new]
    fn new(index: usize, config: Option<Py<PyDict>>) -> PyResult<Self> {
        Ok(Self {
            index,
            config,
        })
    }
}


impl TryInto<MonaSkillInterface> for PySkillInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaSkillInterface, Self::Error> {
        let config: CharacterSkillConfig = if let Some(value) = self.config {
            Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                depythonize(_dict).context("Failed to convert PyDict to CharacterConfig")
            })?
        } else {
            CharacterSkillConfig::NoConfig
        };
        Ok(MonaSkillInterface {
            index: self.index,
            config,
        })
    }
}

