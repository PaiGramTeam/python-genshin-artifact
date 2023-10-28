use anyhow::anyhow;
use pyo3::prelude::*;
use pythonize::depythonize;
use mona::buffs::BuffConfig;
use mona::buffs::buff_name::BuffName;

use pyo3::types::{PyDict, PyString};

use mona_wasm::applications::common::BuffInterface as MonaBuffInterface;

#[pyclass(name = "BuffInterface")]
#[derive(Clone)]
pub struct PyBuffInterface {
    #[pyo3(get, set)]
    pub name: Py<PyString>,
    #[pyo3(get, set)]
    pub config: Option<Py<PyDict>>,
}

#[pymethods]
impl PyBuffInterface {
    #[new]
    pub fn py_new(
        name: Py<PyString>,
        config: Option<Py<PyDict>>,
    ) -> PyResult<Self> {
        Ok(Self {
            name,
            config,
        })
    }
}

impl TryInto<MonaBuffInterface> for PyBuffInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaBuffInterface, Self::Error> {
        let name:BuffName = Python::with_gil(|py| {
            let _string: &PyString = self.name.as_ref(py);
            depythonize(_string).map_err(|err| anyhow!("Failed to deserialize name: {}", err))
        })?;

        let config: BuffConfig = if let Some(value) = self.config {
            Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                depythonize(_dict).map_err(|err| anyhow!("Failed to deserialize config: {}", err))
            })?
        } else {
            BuffConfig::NoConfig
        };

        Ok(MonaBuffInterface {
            name,
            config,
        })
    }
}


