use std::str::FromStr;
use mona::character::{CharacterConfig, CharacterName};
use mona_wasm::applications::common::CharacterInterface;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;

#[derive(Clone)]
#[pyclass(name = "CharacterInterface")]
pub struct PyCharacterInterface {
    pub name: String,
    pub level: usize,
    pub ascend: bool,
    pub constellation: i32,
    pub skill1: usize,
    pub skill2: usize,
    pub skill3: usize,
    pub params: Option<PyDict>,
}

#[pymethods]
impl PyCharacterInterface {
    #[new]
    fn new(name: String,
           level: usize,
           ascend: bool,
           constellation: i32,
           skill1: usize,
           skill2: usize,
           skill3: usize,
           params: Option<PyDict>,
    ) -> Self {
        PyCharacterInterface {
            name,
            level,
            ascend,
            constellation,
            skill1,
            skill2,
            skill3,
            params,
        }
    }


}

impl PyCharacterInterface {

    fn py_dict_to_character_config(py_dict: &PyDict) -> PyResult<CharacterConfig> {
        let json_str = py_dict.to_string()?;
        let json_value = serde_json::from_str(&json_str)?;
        let character_config: CharacterConfig = serde_json::from_value(json_value)?;
        Ok(character_config)
    }

    pub fn to_rust(&self) -> CharacterInterface {
        let name = CharacterName::from_str(&self.name).map_err(|e| PyValueError::new_err((e.to_string(), &self.name, 0)))?;
        let params = self.params.as_ref().map_or(Ok(CharacterConfig::NoConfig), |py_dict| {
            self.py_dict_to_character_config(py_dict).map_err(|e| PyValueError::new_err((e.to_string(), &self.params, 0)))?
        })?;
        CharacterInterface {
            name,
            level: self.level.clone(),
            ascend: self.ascend.clone(),
            constellation: self.constellation.clone(),
            skill1: self.skill1.clone(),
            skill2: self.skill2.clone(),
            skill3: self.skill3.clone(),
            params,
        }
    }
}

