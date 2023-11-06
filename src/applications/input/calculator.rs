use crate::applications::input::artifact::PyArtifact;
use crate::applications::input::buff::PyBuffInterface;
use crate::applications::input::character::PyCharacterInterface;
use crate::applications::input::enemy::PyEnemyInterface;
use crate::applications::input::skill::PySkillInterface;
use crate::applications::input::weapon::PyWeaponInterface;

use pyo3::prelude::*;
use pyo3::types::PyDict;

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
    pub artifact_config: Option<Py<PyDict>>,
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
        artifact_config = "None",
        enemy = "None"
    )]
    pub fn py_new(
        character: PyCharacterInterface,
        weapon: PyWeaponInterface,
        skill: PySkillInterface,
        buffs: Option<Vec<PyBuffInterface>>,
        artifacts: Option<Vec<PyArtifact>>,
        artifact_config: Option<Py<PyDict>>,
        enemy: Option<PyEnemyInterface>,
    ) -> PyResult<Self> {
        Ok(Self {
            character,
            weapon,
            buffs: buffs.unwrap_or_default(),
            artifacts: artifacts.unwrap_or_default(),
            artifact_config,
            skill,
            enemy,
        })
    }
}
