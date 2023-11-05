use anyhow::anyhow;
use mona::weapon::{WeaponConfig, WeaponName};
use mona_wasm::applications::common::WeaponInterface as MonaWeaponInterface;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use pythonize::depythonize;

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
    use super::*;
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
                        .downcast::<PyDict>()
                        .unwrap();
                    assert_eq!(
                        params_dict
                            .get_item("be50_rate")
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
}
