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


#[cfg(test)]
mod tests {

    use mona::attribute::{Attribute, AttributeName, ComplicatedAttributeGraph};
    use mona::character::Character;
    use super::*;

    #[test]
    fn test_character_interface() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let inner_dict = PyDict::new(py);
            inner_dict.set_item("le_50", "true").unwrap();

            let outer_dict = PyDict::new(py);
            outer_dict.set_item("HuTao", inner_dict).unwrap();

            let py_character_interface = PyCharacterInterface {
                name: "HuTao".to_string(),
                level: 90,
                ascend: true,
                constellation: 6,
                skill1: 12,
                skill2: 12,
                skill3: 12,
                params: Some(Py::from(outer_dict)),
            };

            assert_eq!(py_character_interface.get_name().unwrap(), "HuTao");
            assert_eq!(py_character_interface.get_level().unwrap(), 90);
            assert_eq!(py_character_interface.get_ascend().unwrap(), true);
            assert_eq!(py_character_interface.get_constellation().unwrap(), 6);
            assert_eq!(py_character_interface.get_skill1().unwrap(), 12);
            assert_eq!(py_character_interface.get_skill2().unwrap(), 12);
            assert_eq!(py_character_interface.get_skill3().unwrap(), 12);

            let params = py_character_interface.get_params().unwrap();
            match params {
                Some(value) => {
                    let py_dict = value.as_ref(py);
                    let hutao_dict = py_dict.get_item("HuTao").unwrap().downcast::<PyDict>().unwrap();
                    assert_eq!(hutao_dict.get_item("le_50").unwrap().extract::<&str>().unwrap(), "true");
                }
                None => panic!("Expected PyDict, got None"),
            };

            let mona_character_interface: MonaCharacterInterface = py_character_interface.try_into().unwrap();
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

