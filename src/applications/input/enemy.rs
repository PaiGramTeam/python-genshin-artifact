use pyo3::prelude::*;

use mona::enemies::Enemy as MomaEnemy;


#[pyclass(name = "EnemyInterface")]
#[derive(Clone)]
pub struct PyEnemyInterface {
    #[pyo3(get, set)]
    pub level: usize,
    #[pyo3(get, set)]
    pub electro_res: f64,
    #[pyo3(get, set)]
    pub pyro_res: f64,
    #[pyo3(get, set)]
    pub hydro_res: f64,
    #[pyo3(get, set)]
    pub cryo_res: f64,
    #[pyo3(get, set)]
    pub geo_res: f64,
    #[pyo3(get, set)]
    pub anemo_res: f64,
    #[pyo3(get, set)]
    pub dendro_res: f64,
    #[pyo3(get, set)]
    pub physical_res: f64,
}

#[pymethods]
impl PyEnemyInterface {
    #[new]
    fn py_new(
        level: usize,
        electro_res: f64,
        pyro_res: f64,
        hydro_res: f64,
        cryo_res: f64,
        geo_res: f64,
        anemo_res: f64,
        dendro_res: f64,
        physical_res: f64,
    ) -> PyResult<Self> {
        Ok( Self {
            level,
            electro_res,
            pyro_res,
            hydro_res,
            cryo_res,
            geo_res,
            anemo_res,
            dendro_res,
            physical_res,
        })
    }
}

impl TryInto<MomaEnemy> for PyEnemyInterface {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<MomaEnemy, Self::Error> {
        Ok(MomaEnemy {
            level: self.level as i32,
            electro_res: self.electro_res,
            pyro_res: self.pyro_res,
            hydro_res: self.hydro_res,
            cryo_res: self.cryo_res,
            geo_res: self.geo_res,
            anemo_res: self.anemo_res,
            dendro_res: self.dendro_res,
            physical_res: self.physical_res
        })
    }
}

