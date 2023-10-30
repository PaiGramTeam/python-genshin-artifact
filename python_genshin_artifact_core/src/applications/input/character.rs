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
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub level: usize,
    #[pyo3(get, set)]
    pub ascend: bool,
    #[pyo3(get, set)]
    pub constellation: i32,
    #[pyo3(get, set)]
    pub skill1: usize,
    #[pyo3(get, set)]
    pub skill2: usize,
    #[pyo3(get, set)]
    pub skill3: usize,
    #[pyo3(get, set)]
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
            let result: Result<(), anyhow::Error> = Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                params = depythonize(_dict).context("Failed to convert PyDict to CharacterConfig")?;
                Ok(())
            });
            result?;
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


#[cfg(test)]
mod tests {

    use mona::attribute::{Attribute, AttributeName, ComplicatedAttributeGraph};
    use mona::character::Character;
    use super::*;

    const TEST_CHARACTER_NAME: &str = "HuTao";
    const TEST_LEVEL: usize = 90;
    const TEST_CONSTELLATION: i32 = 6;
    const TEST_SKILL_VALUE: usize = 12;
    const TEST_PARAM_VALUE: &str = "true";

    #[test]
    fn test_character_interface() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let inner_dict = PyDict::new(py);
            inner_dict.set_item("le_50", TEST_PARAM_VALUE).unwrap();

            let outer_dict = PyDict::new(py);
            outer_dict.set_item(TEST_CHARACTER_NAME, inner_dict).unwrap();

            let py_character_interface = PyCharacterInterface {
                name: TEST_CHARACTER_NAME.to_string(),
                level: TEST_LEVEL,
                ascend: true,
                constellation: TEST_CONSTELLATION,
                skill1: TEST_SKILL_VALUE,
                skill2: TEST_SKILL_VALUE,
                skill3: TEST_SKILL_VALUE,
                params: Some(Py::from(outer_dict)),
            };

            assert_eq!(py_character_interface.name, TEST_CHARACTER_NAME);
            assert_eq!(py_character_interface.level, TEST_LEVEL);
            assert!(py_character_interface.ascend);
            assert_eq!(py_character_interface.constellation, TEST_CONSTELLATION);
            assert_eq!(py_character_interface.skill1, TEST_SKILL_VALUE);
            assert_eq!(py_character_interface.skill2, TEST_SKILL_VALUE);
            assert_eq!(py_character_interface.skill3, TEST_SKILL_VALUE);

            match &py_character_interface.params {
                Some(value) => {
                    let py_dict = value.as_ref(py);
                    let hutao_dict = py_dict.get_item("HuTao").unwrap().downcast::<PyDict>().unwrap();
                    assert_eq!(hutao_dict.get_item("le_50").unwrap().extract::<&str>().unwrap(), TEST_PARAM_VALUE);
                }
                None => panic!("Expected PyDict, got None"),
            };

            let mona_character_interface: MonaCharacterInterface = py_character_interface.try_into().unwrap();

            assert_eq!(mona_character_interface.name, TEST_CHARACTER_NAME);
            assert_eq!(mona_character_interface.level, TEST_LEVEL);
            assert!(mona_character_interface.ascend);
            assert_eq!(mona_character_interface.constellation, TEST_CONSTELLATION);
            assert_eq!(mona_character_interface.skill1, TEST_SKILL_VALUE);
            assert_eq!(mona_character_interface.skill2, TEST_SKILL_VALUE);
            assert_eq!(mona_character_interface.skill3, TEST_SKILL_VALUE);


            let character: Character<ComplicatedAttributeGraph> = mona_character_interface.to_character();
            assert_eq!(character.common_data.name, CharacterName::HuTao);

            match character.character_effect {
                Some(effect) => {
                    let mut attribute = ComplicatedAttributeGraph::default();
                    effect.change_attribute(&mut attribute);
                    let value = attribute.get_value(AttributeName::BonusPyro);
                    assert_eq!(value, 0.33);
                }
                None => panic!("Expected character.character_effect, got None"),
            }
            println!("PyCharacterInterface 测试成功 遥遥领先！");
        });
    }
}

