use crate::applications::input::character::PyCharacterInterface;
use crate::applications::input::weapon::PyWeaponInterface;
use crate::applications::input::buff::PyBuffInterface;
use pyo3::prelude::*;

#[pyclass]
pub struct CalculatorConfig {
    pub character: PyCharacterInterface,
    pub weapon: PyWeaponInterface,
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
        Ok(Self { character, weapon, buffs })
    }
}
