use mona::damage::transformative_damage::TransformativeDamage as MonaTransformativeDamage;
use pyo3::prelude::*;

#[pyclass(name = "TransformativeDamage")]
#[derive(Clone)]
pub struct PyTransformativeDamage {
    #[pyo3(get, set)]
    swirl_cryo: f64,
    #[pyo3(get, set)]
    swirl_hydro: f64,
    #[pyo3(get, set)]
    swirl_pyro: f64,
    #[pyo3(get, set)]
    swirl_electro: f64,
    #[pyo3(get, set)]
    overload: f64,
    #[pyo3(get, set)]
    electro_charged: f64,
    #[pyo3(get, set)]
    shatter: f64,
    #[pyo3(get, set)]
    super_conduct: f64,
    #[pyo3(get, set)]
    bloom: f64,
    #[pyo3(get, set)]
    hyper_bloom: f64,
    #[pyo3(get, set)]
    burgeon: f64,
    #[pyo3(get, set)]
    burning: f64,
    #[pyo3(get, set)]
    crystallize: f64,
}

#[pymethods]
impl PyTransformativeDamage {
    #[new]
    fn py_new(
        swirl_cryo: f64,
        swirl_hydro: f64,
        swirl_pyro: f64,
        swirl_electro: f64,
        overload: f64,
        electro_charged: f64,
        shatter: f64,
        super_conduct: f64,
        bloom: f64,
        hyper_bloom: f64,
        burgeon: f64,
        burning: f64,
        crystallize: f64,
    ) -> PyResult<Self> {
        Ok(PyTransformativeDamage {
            swirl_cryo,
            swirl_hydro,
            swirl_pyro,
            swirl_electro,
            overload,
            electro_charged,
            shatter,
            super_conduct,
            bloom,
            hyper_bloom,
            burgeon,
            burning,
            crystallize,
        })
    }
}

impl From<MonaTransformativeDamage> for PyTransformativeDamage {
    fn from(damage: MonaTransformativeDamage) -> Self {
        Self {
            swirl_cryo: damage.swirl_cryo,
            swirl_hydro: damage.swirl_hydro,
            swirl_pyro: damage.swirl_pyro,
            swirl_electro: damage.swirl_electro,
            overload: damage.overload,
            electro_charged: damage.electro_charged,
            shatter: damage.shatter,
            super_conduct: damage.superconduct,
            bloom: damage.bloom,
            hyper_bloom: damage.hyperbloom,
            burgeon: damage.burgeon,
            burning: damage.burning,
            crystallize: damage.crystallize,
        }
    }
}