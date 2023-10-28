use anyhow::Context;
use mona::character::{CharacterConfig, CharacterName};
use mona_wasm::applications::common::CharacterInterface as MonaCharacterInterface;
use pyo3::prelude::*;
use pythonize::depythonize;
use std::str::FromStr;
use pyo3::types::PyDict;

#[pyclass(name = "CharacterInterface")]
#[derive(Clone)]
pub struct PyCharacterInterface {
    pub name: String,
    pub level: usize,
    pub ascend: bool,
    pub constellation: i32,
    pub skill1: usize,
    pub skill2: usize,
    pub skill3: usize,
    pub params: Option<Py<PyDict>>,
}

#[pymethods]
impl PyCharacterInterface {
    #[new]
    pub fn py_new(
        name: String,
        level: usize,
        ascend: bool,
        constellation: i32,
        skill1: usize,
        skill2: usize,
        skill3: usize,
        params: Option<Py<PyDict>>,
    ) -> PyResult<Self> {
        Ok(Self {
            name,
            level,
            ascend,
            constellation,
            skill1,
            skill2,
            skill3,
            params,
        })
    }
}

impl TryInto<MonaCharacterInterface> for PyCharacterInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaCharacterInterface, Self::Error> {
        let name = CharacterName::from_str(&self.name).context("Failed to deserialize json")?;
        let mut params: CharacterConfig = CharacterConfig::NoConfig;
        if let Some(value) = self.params {
            Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                params = depythonize(_dict).unwrap();;
            })
        }
        Ok(MonaCharacterInterface {
            name,
            level: self.level,
            ascend: self.ascend,
            constellation: self.constellation,
            skill1: self.skill1,
            skill2: self.skill2,
            skill3: self.skill3,
            params,
        })
    }
}
