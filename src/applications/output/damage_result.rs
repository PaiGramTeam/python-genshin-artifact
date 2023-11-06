use pyo3::prelude::*;
use mona::damage::damage_result::DamageResult as MonaDamageResult;

#[pyclass(name = "DamageResult")]
#[derive(Clone)]
pub struct PyDamageResult {
    #[pyo3(get, set)]
    pub critical: f64,
    #[pyo3(get, set)]
    pub non_critical: f64,
    #[pyo3(get, set)]
    pub expectation: f64,
    #[pyo3(get, set)]
    pub is_heal: bool,
    #[pyo3(get, set)]
    pub is_shield: bool,
}

#[pymethods]
impl PyDamageResult {
    #[new]
    fn py_new(
        critical: f64,
        non_critical: f64,
        expectation: f64,
        is_heal: bool,
        is_shield: bool
    ) -> PyResult<Self> {
        Ok(
            Self { critical, non_critical, expectation, is_heal, is_shield }
        )
    }
}

impl From<MonaDamageResult> for PyDamageResult {
    fn from(damage_result: MonaDamageResult) -> Self {
        Self {
            critical: damage_result.critical,
            non_critical: damage_result.non_critical,
            expectation: damage_result.expectation,
            is_heal: damage_result.is_heal,
            is_shield: damage_result.is_shield
        }
    }
}