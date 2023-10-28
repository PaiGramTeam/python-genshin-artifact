use anyhow::anyhow;
use mona_wasm::applications::common::WeaponInterface as MonaWeaponInterface;
use pyo3::prelude::*;
use pythonize::depythonize;
use mona::weapon::{WeaponConfig, WeaponName};
use pyo3::types::{PyDict, PyString};

#[pyclass(name = "WeaponInterface")]
#[derive(Clone)]
pub struct PyWeaponInterface {
    pub name: Py<PyString>,
    pub level: i32,
    pub ascend: bool,
    pub refine: i32,
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

    #[getter]
    pub fn get_name(&self) -> PyResult<Py<PyString>> {
        Ok(self.name.clone())
    }

    #[setter]
    pub fn set_name(&mut self, name: Py<PyString>) {
        self.name = name;
    }

    #[getter]
    pub fn get_level(&self) -> PyResult<i32> {
        Ok(self.level)
    }

    #[setter]
    pub fn set_level(&mut self, level: i32) {
        self.level = level;
    }

    #[getter]
    pub fn get_ascend(&self) -> PyResult<bool> {
        Ok(self.ascend)
    }

    #[setter]
    pub fn set_ascend(&mut self, ascend: bool) {
        self.ascend = ascend;
    }

    #[getter]
    pub fn get_refine(&self) -> PyResult<i32> {
        Ok(self.refine)
    }

    #[setter]
    pub fn set_refine(&mut self, refine: i32) {
        self.refine = refine;
    }

    #[getter]
    pub fn get_params(&self) -> PyResult<Option<Py<PyDict>>> {
        Ok(self.params.clone())
    }

    #[setter]
    pub fn set_params(&mut self, params: Option<Py<PyDict>>) {
        self.params = params;
    }
}


impl TryInto<MonaWeaponInterface> for PyWeaponInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaWeaponInterface, Self::Error> {
        let name: WeaponName = Python::with_gil(|py| {
            let _string: &PyString = self.name.as_ref(py);
            depythonize(_string).map_err(|err| anyhow!("Failed to deserialize name: {}", err))
        })?;

        let params: WeaponConfig = if let Some(value) = self.params {
            Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                depythonize(_dict).map_err(|err| anyhow!("Failed to deserialize params: {}", err))
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
    use mona::attribute::ComplicatedAttributeGraph;
    use mona::character::{Character, CharacterConfig, CharacterName};
    use mona::weapon::Weapon;
    use super::*;


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

            assert_eq!(py_weapon_interface.get_name().as_ref().unwrap().to_string(), "StaffOfHoma");
            assert_eq!(py_weapon_interface.get_level().unwrap(), 90);
            assert!(py_weapon_interface.get_ascend().unwrap());
            assert_eq!(py_weapon_interface.get_refine().unwrap(), 5);

            let params = py_weapon_interface.get_params().unwrap();
            match params {
                Some(value) => {
                    let py_dict = value.as_ref(py);
                    let params_dict = py_dict.get_item("StaffOfHoma").unwrap().downcast::<PyDict>().unwrap();
                    assert_eq!(params_dict.get_item("be50_rate").unwrap().extract::<f64>().unwrap(), 1.0);
                }
                None => panic!("Expected PyDict, got None"),
            };

            let mona_weapon_interface: MonaWeaponInterface = py_weapon_interface.try_into().unwrap();


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


            let weapon: Weapon<ComplicatedAttributeGraph> = mona_weapon_interface.to_weapon(&character);

            assert_eq!(weapon.common_data.name, WeaponName::StaffOfHoma);

            match weapon.effect {
                Some(effect) => {
                    let mut attribute = ComplicatedAttributeGraph::default();
                    effect.apply(&weapon.common_data, &mut attribute);
                    assert!(attribute.edges.iter().any(|item| item.key == "护摩之杖被动"), "Expected to find key '护摩之杖被动'");
                    assert!(attribute.edges.iter().any(|item| item.key == "护摩之杖被动等效"), "Expected to find key '护摩之杖被动等效'");
                }
                None => panic!("Expected weapon.effect, got None"),
            }
            println!("PyWeaponInterface 测试成功 遥遥领先！");
        });
    }
}