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

    #[getter]
    pub fn get_name(&self) -> PyResult<Py<PyString>> {
        Ok(self.name.clone())
    }

    #[setter]
    pub fn set_name(&mut self, name: Py<PyString>) {
        self.name = name;
    }

    #[getter]
    pub fn get_level(&self) -> PyResult<i32> {
        Ok(self.level)
    }

    #[setter]
    pub fn set_level(&mut self, level: i32) {
        self.level = level;
    }

    #[getter]
    pub fn get_ascend(&self) -> PyResult<bool> {
        Ok(self.ascend)
    }

    #[setter]
    pub fn set_ascend(&mut self, ascend: bool) {
        self.ascend = ascend;
    }

    #[getter]
    pub fn get_refine(&self) -> PyResult<i32> {
        Ok(self.refine)
    }

    #[setter]
    pub fn set_refine(&mut self, refine: i32) {
        self.refine = refine;
    }

    #[getter]
    pub fn get_params(&self) -> PyResult<Option<Py<PyDict>>> {
        Ok(self.params.clone())
    }

    #[setter]
    pub fn set_params(&mut self, params: Option<Py<PyDict>>) {
        self.params = params;
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
