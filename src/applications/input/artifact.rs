use anyhow::anyhow;

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList, PyString};
use pythonize::depythonize;

use mona::artifacts::{Artifact as MonaArtifact, ArtifactSetName, ArtifactSlotName};
use mona::common::StatName;

#[pyclass(name = "Artifact")]
#[derive(Clone)]
pub struct PyArtifact {
    #[pyo3(get, set)]
    pub set_name: Py<PyString>,
    #[pyo3(get, set)]
    pub slot: Py<PyString>,
    #[pyo3(get, set)]
    pub level: i32,
    #[pyo3(get, set)]
    pub star: i32,
    #[pyo3(get, set)]
    pub sub_stats: Vec<(Py<PyString>, f64)>,
    #[pyo3(get, set)]
    pub main_stat: (Py<PyString>, f64),
    #[pyo3(get, set)]
    pub id: u64,
}

#[pymethods]
impl PyArtifact {
    #[new]
    pub fn py_new(
        set_name: Py<PyString>,
        slot: Py<PyString>,
        level: i32,
        star: i32,
        sub_stats: Vec<(Py<PyString>, f64)>,
        main_stat: (Py<PyString>, f64),
        id: u64,
    ) -> PyResult<Self> {
        Ok(Self {
            set_name,
            slot,
            level,
            star,
            sub_stats,
            main_stat,
            id,
        })
    }

    pub fn __repr__(&self, py: Python) -> PyResult<String> {
        let set_name = self.set_name.as_ref(py).to_str()?;
        let slot = self.slot.as_ref(py).to_str()?;
        let main_stat = self.main_stat.0.as_ref(py).to_str()?;
        let main_stat_value = self.main_stat.1;
        Ok(format!(
            "PyArtifact(set_name='{}', slot='{}', level={}, star={}, main_stat=({}, {}), id={})",
            set_name, slot, self.level, self.star, main_stat, main_stat_value, self.id
        ))
    }

    #[getter]
    pub fn __dict__(&self, py: Python) -> PyResult<PyObject> {
        let dict = PyDict::new(py);
        dict.set_item("set_name", self.set_name.as_ref(py))?;
        dict.set_item("slot", self.slot.as_ref(py))?;
        dict.set_item("level", self.level)?;
        dict.set_item("star", self.star)?;
        let sub_stats_pylist = PyList::new(
            py,
            self.sub_stats.iter().map(|(s, v)| {
                let stat_str = s.as_ref(py).to_str().unwrap();
                (stat_str, *v)
            }),
        );
        dict.set_item("sub_stats", sub_stats_pylist)?;
        let main_stat_tuple = (self.main_stat.0.as_ref(py), self.main_stat.1);
        dict.set_item("main_stat", main_stat_tuple)?;
        dict.set_item("id", self.id)?;

        Ok(dict.into())
    }
}

impl TryInto<MonaArtifact> for PyArtifact {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaArtifact, Self::Error> {
        let name: ArtifactSetName = Python::with_gil(|py| {
            let _string: &PyString = self.set_name.as_ref(py);
            depythonize(_string).map_err(|err| {
                let serialized_data = format!("{:?}", _string);
                anyhow!(
                    "Failed to deserialize name into mona::artifacts::ArtifactSetName: {}. Serialized data: \n{}",
                    err,
                    serialized_data
                )
            })
        })?;

        let slot: ArtifactSlotName = Python::with_gil(|py| {
            let _string: &PyString = self.slot.as_ref(py);
            depythonize(_string).map_err(|err| {
                let serialized_data = format!("{:?}", _string);
                anyhow!(
                    "Failed to deserialize slot name into mona::artifacts::ArtifactSlotName: {}. Serialized data: \n{}",
                    err,
                    serialized_data
                )
            })
        })?;

        let main_stat_name: StatName = Python::with_gil(|py| {
            let main_stat = self.main_stat.0.as_ref(py);
            depythonize(self.main_stat.0.as_ref(py)).map_err(|err| {
                let serialized_data = format!("{:?}", main_stat);
                anyhow!(
                    "Failed to deserialize main stat into mona::artifacts::StatName: {}. Serialized data: \n{}",
                    err,
                    serialized_data
                )
            })
        })?;

        let sub_stats = Python::with_gil(|py| {
            self.sub_stats
                .iter()
                .map(|s| {
                    let sub_stats = s.0.as_ref(py);
                    let name: Result<StatName, anyhow::Error> = depythonize(s.0.as_ref(py)).map_err(|err| {
                        let serialized_data = format!("{:?}", sub_stats);
                        anyhow!(
                            "Failed to deserialize sub stats into mona::artifacts::StatName: {}. Serialized data: \n{}",
                            err,
                            serialized_data
                        )
                    });
                    match name {
                        Ok(n) => Ok((n, s.1)),
                        Err(e) => Err(e),
                    }
                })
                .collect::<Result<Vec<(StatName, f64)>, anyhow::Error>>()
        })?;

        Ok(MonaArtifact {
            set_name: name,
            slot,
            level: self.level,
            star: self.star,
            sub_stats,
            main_stat: (main_stat_name, self.main_stat.1),
            id: self.id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Context;

    #[test]
    fn test_artifact_set_name() -> PyResult<()> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let module = PyModule::import(py, "python_genshin_artifact.enka.artifacts")?;
            let artifacts_name_map = module.getattr("artifacts_name_map")?.downcast::<PyDict>()?;
            for (_, value) in artifacts_name_map.iter() {
                let artifacts_name_str = value.extract::<String>()?;
                let res: Result<ArtifactSetName, anyhow::Error> = depythonize(&value).context(
                    format!("Artifact name '{}' does not exist", artifacts_name_str),
                );
                if res.is_err() {
                    println!("{:?}", res.err().map(|e| e.to_string()));
                }
            }
            Ok(())
        })
    }
}
