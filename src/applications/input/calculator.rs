use crate::applications::input::artifact::PyArtifact;
use crate::applications::input::buff::PyBuffInterface;
use crate::applications::input::character::PyCharacterInterface;
use crate::applications::input::enemy::PyEnemyInterface;
use crate::applications::input::skill::PySkillInterface;
use crate::applications::input::weapon::PyWeaponInterface;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

#[pyclass(name = "CalculatorConfig")]
#[derive(Clone)]
pub struct PyCalculatorConfig {
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
impl PyCalculatorConfig {
    #[new]
    #[pyo3(signature=(character, weapon, skill, buffs = None, artifacts = None, artifact_config = None, enemy = None))]
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

    #[getter]
    pub fn __dict__(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("character", self.character.__dict__(py)?)?;
        dict.set_item("weapon", self.weapon.__dict__(py)?)?;
        let buffs = self
            .buffs
            .iter()
            .map(|b| b.__dict__(py))
            .collect::<Result<Vec<PyObject>, PyErr>>()?;
        dict.set_item("buffs", PyList::new(py, buffs))?;
        let artifacts = self
            .artifacts
            .iter()
            .map(|ar| ar.__dict__(py))
            .collect::<Result<Vec<PyObject>, PyErr>>()?;
        dict.set_item("artifacts", PyList::new(py, artifacts))?;
        if let Some(artifact_config) = self.artifact_config.as_ref().map(|c| c.as_ref(py)) {
            dict.set_item("artifact_config", artifact_config)?;
        } else {
            dict.set_item("artifact_config", py.None())?;
        }
        dict.set_item("skill", self.skill.__dict__(py)?)?;
        if let Some(enemy) = self.enemy.as_ref().map(|e| e.__dict__(py)).transpose()? {
            dict.set_item("enemy", enemy)?;
        } else {
            dict.set_item("enemy", py.None())?;
        }
        Ok(dict.into())
    }
}
