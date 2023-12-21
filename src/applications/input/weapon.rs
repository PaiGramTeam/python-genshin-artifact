use anyhow::anyhow;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use pythonize::depythonize;

use mona::weapon::{WeaponConfig, WeaponName};
use mona_wasm::applications::common::WeaponInterface as MonaWeaponInterface;

#[pyclass(name = "WeaponInterface")]
#[derive(Clone)]
pub struct PyWeaponInterface {
    #[pyo3(get, set)]
    pub name: Py<PyString>,
    #[pyo3(get, set)]
    pub level: i32,
    #[pyo3(get, set)]
    pub ascend: bool,
    #[pyo3(get, set)]
    pub refine: i32,
    #[pyo3(get, set)]
    pub params: Option<Py<PyDict>>,
}

#[pymethods]
impl PyWeaponInterface {
    #[new]
    pub fn py_new(
        name: Py<PyString>,
        level: i32,
        ascend: bool,
        refine: i32,
        params: Option<Py<PyDict>>,
    ) -> PyResult<Self> {
        Ok(Self {
            name,
            level,
            ascend,
            refine,
            params,
        })
    }

    pub fn __repr__(&self, py: Python) -> PyResult<String> {
        let name = self.name.as_ref(py).to_str()?;
        let params_repr = match &self.params {
            Some(params) => params.as_ref(py).repr()?.to_str()?.to_string(),
            None => "None".to_string(),
        };

        Ok(format!(
            "WeaponInterface(name='{}', level={}, ascend={}, refine={}, params={})",
            name, self.level, self.ascend, self.refine, params_repr
        ))
    }

    #[getter]
    pub fn __dict__(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("name", self.name.as_ref(py))?;
        dict.set_item("level", self.level)?;
        dict.set_item("ascend", self.ascend)?;
        dict.set_item("refine", self.refine)?;
        if let Some(params) = &self.params {
            dict.set_item("params", params.as_ref(py))?;
        } else {
            dict.set_item("params", py.None())?;
        }
        Ok(dict.into())
    }
}

impl TryInto<MonaWeaponInterface> for PyWeaponInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaWeaponInterface, Self::Error> {
        let name: WeaponName = Python::with_gil(|py| {
            let _string: &PyString = self.name.as_ref(py);
            depythonize(_string).map_err(|err| {
                let serialized_data = format!("{:?}", _string);
                anyhow!(
                    "Failed to deserialize name into mona::weapon::WeaponName: {}. Serialized data: \n{}",
                    err,
                    serialized_data
                )
            })
        })?;

        let params: WeaponConfig = if let Some(value) = self.params {
            Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                depythonize(_dict).map_err(|err| {
                    let serialized_data = format!("{:?}", _dict);
                    anyhow!(
                        "Failed to deserialize params into mona::weapon::WeaponConfig: {}. Serialized data: \n{}",
                        err,
                        serialized_data
                    )
                })
            })?
        } else {
            WeaponConfig::NoConfig
        };
        Ok(MonaWeaponInterface {
            name,
            level: self.level,
            ascend: self.ascend,
            refine: self.refine,
            params,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;
    use mona::attribute::ComplicatedAttributeGraph;
    use mona::character::{Character, CharacterConfig, CharacterName};
    use mona::weapon::Weapon;

    #[test]
    fn test_weapon_interface() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let inner_dict = PyDict::new(py);
            inner_dict.set_item("be50_rate", 1.0).unwrap();

            let params_dict = PyDict::new(py);
            params_dict.set_item("StaffOfHoma", inner_dict).unwrap();

            let name = PyString::new(py, "StaffOfHoma");

            let py_weapon_interface = PyWeaponInterface {
                name: Py::from(name),
                level: 90,
                ascend: true,
                refine: 5,
                params: Some(Py::from(params_dict)),
            };

            assert_eq!(
                py_weapon_interface.name.as_ref(py).to_string(),
                "StaffOfHoma"
            );
            assert_eq!(py_weapon_interface.level, 90);
            assert!(py_weapon_interface.ascend);
            assert_eq!(py_weapon_interface.refine, 5);

            match &py_weapon_interface.params {
                Some(value) => {
                    let py_dict = value.as_ref(py);
                    let params_dict = py_dict
                        .get_item("StaffOfHoma")
                        .unwrap()
                        .unwrap()
                        .downcast::<PyDict>()
                        .unwrap();
                    assert_eq!(
                        params_dict
                            .get_item("be50_rate")
                            .unwrap()
                            .unwrap()
                            .extract::<f64>()
                            .unwrap(),
                        1.0
                    );
                }
                None => panic!("Expected PyDict, got None"),
            };

            let mona_weapon_interface: MonaWeaponInterface =
                py_weapon_interface.try_into().unwrap();

            assert_eq!(mona_weapon_interface.name, WeaponName::StaffOfHoma);
            assert_eq!(mona_weapon_interface.level, 90);
            assert!(mona_weapon_interface.ascend);
            assert_eq!(mona_weapon_interface.refine, 5);

            let character: Character<ComplicatedAttributeGraph> = Character::new(
                CharacterName::HuTao,
                90,
                true,
                6,
                12,
                12,
                12,
                &CharacterConfig::HuTao { le_50: true },
            );

            let weapon: Weapon<ComplicatedAttributeGraph> =
                mona_weapon_interface.to_weapon(&character);

            assert_eq!(weapon.common_data.name, WeaponName::StaffOfHoma);

            match weapon.effect {
                Some(effect) => {
                    let mut attribute = ComplicatedAttributeGraph::default();
                    effect.apply(&weapon.common_data, &mut attribute);
                    assert!(
                        attribute
                            .edges
                            .iter()
                            .any(|item| item.key == "护摩之杖被动"),
                        "Expected to find key '护摩之杖被动'"
                    );
                    assert!(
                        attribute
                            .edges
                            .iter()
                            .any(|item| item.key == "护摩之杖被动等效"),
                        "Expected to find key '护摩之杖被动等效'"
                    );
                }
                None => panic!("Expected weapon.effect, got None"),
            }
            println!("PyWeaponInterface 测试成功 遥遥领先！");
        });
    }

    #[test]
    fn test_weapon_name() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let module = PyModule::import(py, "python_genshin_artifact.enka.weapon")?;
            let weapon_name_map = module.getattr("weapon_name_map")?.downcast::<PyDict>()?;
            for (_, value) in weapon_name_map.iter() {
                let weapon_name_str = value.extract::<String>()?;
                let res: Result<WeaponName, anyhow::Error> = depythonize(&value)
                    .context(format!("Weapon name '{}' does not exist", weapon_name_str));
                if res.is_err() {
                    println!("{:?}", res.err().map(|e| e.to_string()));
                }
            }
            Ok(())
        })
    }
}
