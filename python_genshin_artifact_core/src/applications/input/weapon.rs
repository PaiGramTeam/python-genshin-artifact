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
