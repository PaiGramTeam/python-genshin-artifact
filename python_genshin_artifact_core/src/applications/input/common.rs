use anyhow::Context;
use mona::character::{CharacterConfig, CharacterName};
use mona_wasm::applications::common::CharacterInterface as MonaCharacterInterface;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::str::FromStr;

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
    // todo : 自定义 CharacterConfig 并用 proc macro 实现 Into<Mona::character::CharacterInfo>
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
        if let Some(params) = self.params {
            Python::with_gil(|py| {
                // PyDict 无法转换为 CharacterConfig 类
                // todo : 自定义 CharacterConfig 并用 proc macro 实现 Into<Mona::character::CharacterConfig>
                let _dict: &PyDict = params.as_ref(py);
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
            params: CharacterConfig::NoConfig,
        })
    }
}
