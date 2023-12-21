use anyhow::{anyhow, Context};
use std::str::FromStr;

use pyo3::prelude::*;
use pyo3::types::PyDict;
use pythonize::depythonize;

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

    pub fn __repr__(&self) -> PyResult<String> {
        let params_repr = match &self.params {
            Some(params) => format!("{:?}", params),
            None => "None".to_string(),
        };

        Ok(format!(
            "CharacterInterface(name='{}', level={}, ascend={}, constellation={}, skill1={}, skill2={}, skill3={}, params={})",
            self.name, self.level, self.ascend, self.constellation, self.skill1, self.skill2, self.skill3, params_repr
        ))
    }

    pub fn __dict__(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);

        dict.set_item("name", &self.name)?;
        dict.set_item("level", self.level)?;
        dict.set_item("ascend", self.ascend)?;
        dict.set_item("constellation", self.constellation)?;
        dict.set_item("skill1", self.skill1)?;
        dict.set_item("skill2", self.skill2)?;
        dict.set_item("skill3", self.skill3)?;

        if let Some(params) = &self.params {
            dict.set_item("params", params)?;
        } else {
            dict.set_item("params", py.None())?;
        }

        Ok(dict.into())
    }
}

impl TryInto<MonaCharacterInterface> for PyCharacterInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaCharacterInterface, Self::Error> {
        let name = CharacterName::from_str(&self.name)
            .context("Failed to name params into mona::character::CharacterName")?;
        let params: CharacterConfig = if let Some(value) = self.params {
            Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                depythonize(_dict).map_err(|err| {
                    let serialized_data = format!("{:?}", _dict);
                    anyhow!(
                        "Failed to deserialize params into mona::character::CharacterConfig: {}. Serialized data: \n{}",
                        err,
                        serialized_data
                    )
                })
            })?
        } else {
            CharacterConfig::NoConfig
        };
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
    use super::*;
    use mona::attribute::{Attribute, AttributeName, ComplicatedAttributeGraph};
    use mona::character::Character;

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

            assert_eq!(py_character_interface.name, "HuTao");
            assert_eq!(py_character_interface.level, 90);
            assert!(py_character_interface.ascend);
            assert_eq!(py_character_interface.constellation, 6);
            assert_eq!(py_character_interface.skill1, 12);
            assert_eq!(py_character_interface.skill2, 12);
            assert_eq!(py_character_interface.skill3, 12);

            match &py_character_interface.params {
                Some(value) => {
                    let py_dict = value.as_ref(py);
                    let hutao_dict = py_dict
                        .get_item("HuTao")
                        .unwrap()
                        .unwrap()
                        .downcast::<PyDict>()
                        .unwrap();
                    assert_eq!(
                        hutao_dict
                            .get_item("le_50")
                            .unwrap()
                            .unwrap()
                            .extract::<&str>()
                            .unwrap(),
                        "true"
                    );
                }
                None => panic!("Expected PyDict, got None"),
            };

            let mona_character_interface: MonaCharacterInterface =
                py_character_interface.try_into().unwrap();

            assert_eq!(mona_character_interface.name, CharacterName::HuTao);
            assert_eq!(mona_character_interface.level, 90);
            assert!(mona_character_interface.ascend);
            assert_eq!(mona_character_interface.constellation, 6);
            assert_eq!(mona_character_interface.skill1, 12);
            assert_eq!(mona_character_interface.skill2, 12);
            assert_eq!(mona_character_interface.skill3, 12);

            let character: Character<ComplicatedAttributeGraph> =
                mona_character_interface.to_character();
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

    #[test]
    fn test_character_name() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let module = PyModule::import(py, "python_genshin_artifact.enka.characters")?;
            let characters_map = module.getattr("characters_map")?.downcast::<PyDict>()?;
            for (_, value) in characters_map.iter() {
                let character_name_str = value.extract::<String>()?;
                let res = CharacterName::from_str(&character_name_str).context(format!(
                    "Character name '{}' does not exist",
                    character_name_str
                ));
                if res.is_err() {
                    println!("{:?}", res.err().map(|e| e.to_string()));
                }
            }
            Ok(())
        })
    }
}
