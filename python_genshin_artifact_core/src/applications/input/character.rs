use anyhow::Context;
use pythonize::depythonize;
use std::str::FromStr;
use pyo3::types::PyDict;
use pyo3::prelude::*;

use mona::character::{CharacterConfig, CharacterName};
use mona_wasm::applications::common::CharacterInterface as MonaCharacterInterface;

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

    pub fn get_name(&self) -> PyResult<String> {
        Ok(self.name.clone())
    }

    pub fn get_level(&self) -> PyResult<usize> {
        Ok(self.level)
    }

    pub fn get_ascend(&self) -> PyResult<bool> {
        Ok(self.ascend)
    }

    pub fn get_constellation(&self) -> PyResult<i32> {
        Ok(self.constellation)
    }

    pub fn get_skill1(&self) -> PyResult<usize> {
        Ok(self.skill1)
    }

    pub fn get_skill2(&self) -> PyResult<usize> {
        Ok(self.skill2)
    }

    pub fn get_skill3(&self) -> PyResult<usize> {
        Ok(self.skill3)
    }

    pub fn get_params(&self) -> PyResult<Option<Py<PyDict>>> {
        Ok(self.params.clone())
    }

    pub fn set_name(&mut self, name: String) -> PyResult<()> {
        self.name = name;
        Ok(())
    }

    pub fn set_level(&mut self, level: usize) -> PyResult<()> {
        self.level = level;
        Ok(())
    }

    pub fn set_ascend(&mut self, ascend: bool) -> PyResult<()> {
        self.ascend = ascend;
        Ok(())
    }

    pub fn set_constellation(&mut self, constellation: i32) -> PyResult<()> {
        self.constellation = constellation;
        Ok(())
    }

    pub fn set_skill1(&mut self, skill1: usize) -> PyResult<()> {
        self.skill1 = skill1;
        Ok(())
    }

    pub fn set_skill2(&mut self, skill2: usize) -> PyResult<()> {
        self.skill2 = skill2;
        Ok(())
    }

    pub fn set_skill3(&mut self, skill3: usize) -> PyResult<()> {
        self.skill3 = skill3;
        Ok(())
    }

    pub fn set_params(&mut self, params: Option<Py<PyDict>>) -> PyResult<()> {
        self.params = params;
        Ok(())
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
                params = depythonize(_dict).unwrap();
                ;
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
