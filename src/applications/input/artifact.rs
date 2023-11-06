use anyhow::anyhow;
use mona::artifacts::{Artifact as MonaArtifact, ArtifactSetName, ArtifactSlotName};
use mona::common::StatName;
use pyo3::prelude::*;
use pyo3::types::PyString;
use pythonize::depythonize;

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
}

impl TryInto<MonaArtifact> for PyArtifact {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MonaArtifact, Self::Error> {
        let name: ArtifactSetName = Python::with_gil(|py| {
            let _string: &PyString = self.set_name.as_ref(py);
            depythonize(_string)
                .map_err(|err| anyhow!("Failed to deserialize artifact set name: {}", err))
        })?;

        let slot: ArtifactSlotName = Python::with_gil(|py| {
            let _string: &PyString = self.slot.as_ref(py);
            depythonize(_string)
                .map_err(|err| anyhow!("Failed to deserialize artifact slot name: {}", err))
        })?;

        let main_stat_name: StatName = Python::with_gil(|py| {
            depythonize(self.main_stat.0.as_ref(py))
                .map_err(|err| anyhow!("Failed to deserialize main stat name: {}", err))
        })?;

        Ok(MonaArtifact {
            set_name: name,
            slot,
            level: self.level,
            star: self.star,
            sub_stats: vec![],
            main_stat: (main_stat_name, self.main_stat.1),
            id: self.id,
        })
    }
}
