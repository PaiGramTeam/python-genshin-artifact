use crate::applications::input::artifact::PyArtifact;
use crate::applications::input::buff::PyBuffInterface;
use crate::applications::input::character::PyCharacterInterface;
use crate::applications::input::enemy::PyEnemyInterface;
use crate::applications::input::skill::PySkillInterface;
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
    #[pyo3(get, set)]
    pub artifacts: Vec<PyArtifact>,
    #[pyo3(get, set)]
    pub skill: PySkillInterface,
    #[pyo3(get, set)]
    pub enemy: Option<PyEnemyInterface>,
}

#[pymethods]
impl CalculatorConfig {
    #[new]
    #[args(
        buffs = "None",
        artifacts = "None",
        enemy = "None"
    )]
    pub fn py_new(
        character: PyCharacterInterface,
        weapon: PyWeaponInterface,
        skill: PySkillInterface,
        buffs: Option<Vec<PyBuffInterface>>,
        artifacts: Option<Vec<PyArtifact>>,
        enemy: Option<PyEnemyInterface>,
    ) -> PyResult<Self> {
        Ok(Self {
            character,
            weapon,
            buffs: buffs.unwrap_or_default(),
            artifacts: artifacts.unwrap_or_default(),
            skill,
            enemy,
        })
    }
}
