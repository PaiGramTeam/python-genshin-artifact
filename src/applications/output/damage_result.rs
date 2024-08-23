use mona::damage::damage_result::DamageResult as MonaDamageResult;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

use serde::{Serialize, Deserialize};
use bincode::{serialize, deserialize};

#[pyclass(module = "python_genshin_artifact", name = "DamageResult")]
#[derive(Clone, Deserialize, Serialize)]
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
        is_shield: bool,
    ) -> PyResult<Self> {
        Ok(Self {
            critical,
            non_critical,
            expectation,
            is_heal,
            is_shield,
        })
    }

    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "DamageResult(critical={}, non_critical={}, expectation={}, is_heal={}, is_shield={})",
            self.critical, self.non_critical, self.expectation, self.is_heal, self.is_shield
        ))
    }

    #[getter]
    pub fn __dict__(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("critical", self.critical)?;
        dict.set_item("non_critical", self.non_critical)?;
        dict.set_item("expectation", self.expectation)?;
        dict.set_item("is_heal", self.is_heal)?;
        dict.set_item("is_shield", self.is_shield)?;
        Ok(dict.into())
    }

    pub fn __setstate__(&mut self, state: &PyBytes) -> PyResult<()> {
        *self = deserialize(state.as_bytes()).unwrap();
        Ok(())
    }

    pub fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<&'py PyBytes> {
        Ok(PyBytes::new(py, &serialize(&self).unwrap()))
    }

    pub fn __getnewargs__(&self) -> PyResult<(f64, f64, f64, bool, bool)> {
        Ok((
            self.critical,
            self.non_critical,
            self.expectation,
            self.is_heal,
            self.is_shield,
        ))
    }
}

impl From<MonaDamageResult> for PyDamageResult {
    fn from(damage_result: MonaDamageResult) -> Self {
        Self {
            critical: damage_result.critical,
            non_critical: damage_result.non_critical,
            expectation: damage_result.expectation,
            is_heal: damage_result.is_heal,
            is_shield: damage_result.is_shield,
        }
    }
}
