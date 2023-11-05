use crate::applications::input::buff::PyBuffInterface;
use crate::applications::input::character::PyCharacterInterface;
use crate::applications::input::weapon::PyWeaponInterface;
use pyo3::prelude::*;

#[pyclass]
pub struct CalculatorConfig {
    #[pyo3(get, set)]
    pub character: PyCharacterInterface,
    #[pyo3(get, set)]
    pub weapon: PyWeaponInterface,
    #[pyo3(get, set)]
    pub buffs: Vec<PyBuffInterface>,
}

#[pymethods]
impl CalculatorConfig {
    #[new]
    pub fn py_new(
        character: PyCharacterInterface,
        weapon: PyWeaponInterface,
        buffs: Vec<PyBuffInterface>,
    ) -> PyResult<Self> {
        Ok(Self {
            character,
            weapon,
            buffs,
        })
    }
}
