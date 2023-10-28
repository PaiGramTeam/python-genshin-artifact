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

    #[getter]
    pub fn get_character(&self) -> PyResult<PyCharacterInterface> {
        Ok(self.character.clone())
    }

    #[setter]
    pub fn set_character(&mut self, character: PyCharacterInterface) -> PyResult<()> {
        self.character = character;
        Ok(())
    }

    #[getter]
    pub fn get_weapon(&self) -> PyResult<PyWeaponInterface> {
        Ok(self.weapon.clone())
    }

    #[setter]
    pub fn set_weapon(&mut self, weapon: PyWeaponInterface) -> PyResult<()> {
        self.weapon = weapon;
        Ok(())
    }

    #[getter]
    pub fn get_buffs(&self) -> PyResult<Vec<PyBuffInterface>> {
        Ok(self.buffs.clone())
    }

    #[setter]
    pub fn set_buffs(&mut self, buffs: Vec<PyBuffInterface>) -> PyResult<()> {
        self.buffs = buffs;
        Ok(())
    }
}
