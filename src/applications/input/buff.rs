use anyhow::anyhow;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyString};
use pythonize::depythonize;

use mona::buffs::buff_name::BuffName;
use mona::buffs::BuffConfig;

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
    pub fn py_new(name: Py<PyString>, config: Option<Py<PyDict>>) -> PyResult<Self> {
        Ok(Self { name, config })
    }

    pub fn __repr__(&self, py: Python) -> PyResult<String> {
        let name = self.name.as_ref(py).to_str()?;
        let config_repr = match &self.config {
            Some(config) => config.as_ref(py).repr()?.to_str()?.to_string(),
            None => "None".to_string(),
        };
        Ok(format!(
            "BuffInterface(name={}, config={})",
            name, config_repr
        ))
    }

    #[getter]
    pub fn __dict__(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        let name_str = self.name.as_ref(py).to_str()?;
        dict.set_item("name", name_str)?;
        if let Some(config) = &self.config {
            dict.set_item("config", config.as_ref(py))?;
        } else {
            dict.set_item("config", py.None())?;
        }
        Ok(dict.into())
    }
}

impl TryInto<MonaBuffInterface> for PyBuffInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaBuffInterface, Self::Error> {
        let name: BuffName = Python::with_gil(|py| {
            let _string: &PyString = self.name.as_ref(py);
            depythonize(_string).map_err(|err| {
                let serialized_data = format!("{:?}", _string);
                anyhow!(
                    "Failed to deserialize name into mona::buffs::buff_name::BuffName: {}. Serialized data: \n{}",
                    err,
                    serialized_data
                )
            })
        })?;

        let config: BuffConfig = if let Some(value) = self.config {
            Python::with_gil(|py| {
                let _dict: &PyDict = value.as_ref(py);
                depythonize(_dict).map_err(|err| {
                    let serialized_data = format!("{:?}", _dict);
                    anyhow!(
                        "Failed to deserialize config into mona::buffs::BuffConfig: {}. Serialized data: \n{}",
                        err,
                        serialized_data
                    )
                })
            })?
        } else {
            BuffConfig::NoConfig
        };

        Ok(MonaBuffInterface { name, config })
    }
}
