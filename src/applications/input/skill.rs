use anyhow::anyhow;
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
        Ok(Self { index, config })
    }
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "SkillInterface(index: {}, config: {:?})",
            self.index, self.config
        ))
    }

    #[getter]
    pub fn __dict__(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("index", self.index)?;
        if let Some(config) = &self.config {
            dict.set_item("config", config.as_ref(py))?;
        } else {
            dict.set_item("config", py.None())?;
        }
        Ok(dict.into())
    }
}

impl TryInto<MonaSkillInterface> for PySkillInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaSkillInterface, Self::Error> {
        let config: CharacterSkillConfig = if let Some(value) = self.config {
            Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                depythonize(_dict).map_err(|err| {
                    let serialized_data = format!("{:?}", _dict);
                    anyhow!("Failed to deserialize config into mona::character::skill_config::CharacterSkillConfig: {}. Serialized data: \n{}", err, serialized_data)
                })
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
