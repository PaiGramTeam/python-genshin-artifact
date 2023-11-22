use crate::applications::output::damage_result::PyDamageResult;
use mona::damage::DamageAnalysis as MonaDamageAnalysis;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

#[pyclass(name = "DamageAnalysis")]
#[derive(Clone)]
pub struct PyDamageAnalysis {
    #[pyo3(get, set)]
    pub atk: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub atk_ratio: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub hp: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub hp_ratio: HashMap<String, f64>,
    #[pyo3(get, set, name = "defense")]
    pub def: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub def_ratio: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub em: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub em_ratio: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub extra_damage: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub bonus: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub critical: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub critical_damage: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub melt_enhance: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub vaporize_enhance: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub healing_bonus: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub shield_strength: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub spread_compose: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub aggravate_compose: HashMap<String, f64>,

    #[pyo3(get, set)]
    pub def_minus: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub def_penetration: HashMap<String, f64>,
    #[pyo3(get, set)]
    pub res_minus: HashMap<String, f64>,

    #[pyo3(get, set)]
    pub element: String,
    #[pyo3(get, set)]
    pub is_heal: bool,
    #[pyo3(get, set)]
    pub is_shield: bool,

    #[pyo3(get, set)]
    pub normal: PyDamageResult,
    #[pyo3(get, set)]
    pub melt: Option<PyDamageResult>,
    #[pyo3(get, set)]
    pub vaporize: Option<PyDamageResult>,
    #[pyo3(get, set)]
    pub spread: Option<PyDamageResult>,
    #[pyo3(get, set)]
    pub aggravate: Option<PyDamageResult>,
}

#[pymethods]
impl PyDamageAnalysis {
    #[getter]
    fn __dict__(&self, py: Python) -> PyResult<PyObject> { // skipcq: RS-R1000
        let dict = PyDict::new(py);

        fn insert_hashmap(
            dict: &PyDict,
            py: Python,
            key: &str,
            hashmap: &HashMap<String, f64>,
        ) -> PyResult<()> {
            let hashmap_dict = PyDict::new(py);
            for (k, &v) in hashmap.iter() {
                hashmap_dict.set_item(k, v)?;
            }
            dict.set_item(key, hashmap_dict)?;
            Ok(())
        }

        insert_hashmap(dict, py, "atk", &self.atk)?;
        insert_hashmap(dict, py, "atk_ratio", &self.atk_ratio)?;
        insert_hashmap(dict, py, "hp", &self.hp)?;
        insert_hashmap(dict, py, "hp_ratio", &self.hp_ratio)?;
        insert_hashmap(dict, py, "defense", &self.def)?;
        insert_hashmap(dict, py, "def_ratio", &self.def_ratio)?;
        insert_hashmap(dict, py, "em", &self.em)?;
        insert_hashmap(dict, py, "em_ratio", &self.em_ratio)?;
        insert_hashmap(dict, py, "extra_damage", &self.extra_damage)?;
        insert_hashmap(dict, py, "bonus", &self.bonus)?;
        insert_hashmap(dict, py, "critical", &self.critical)?;
        insert_hashmap(dict, py, "critical_damage", &self.critical_damage)?;
        insert_hashmap(dict, py, "melt_enhance", &self.melt_enhance)?;
        insert_hashmap(dict, py, "vaporize_enhance", &self.vaporize_enhance)?;
        insert_hashmap(dict, py, "healing_bonus", &self.healing_bonus)?;
        insert_hashmap(dict, py, "shield_strength", &self.shield_strength)?;
        insert_hashmap(dict, py, "spread_compose", &self.spread_compose)?;
        insert_hashmap(dict, py, "aggravate_compose", &self.aggravate_compose)?;
        insert_hashmap(dict, py, "def_minus", &self.def_minus)?;
        insert_hashmap(dict, py, "def_penetration", &self.def_penetration)?;
        insert_hashmap(dict, py, "res_minus", &self.res_minus)?;

        dict.set_item("element", &self.element)?;
        dict.set_item("is_heal", self.is_heal)?;
        dict.set_item("is_shield", self.is_shield)?;

        dict.set_item("normal", self.normal.__dict__(py)?)?;
        if let Some(melt) = self.melt.as_ref().map(|e| e.__dict__(py)).transpose()? {
            dict.set_item("melt", melt)?;
        } else {
            dict.set_item("melt", py.None())?;
        }
        if let Some(vaporize) = self.vaporize.as_ref().map(|e| e.__dict__(py)).transpose()? {
            dict.set_item("vaporize", vaporize)?;
        } else {
            dict.set_item("vaporize", py.None())?;
        }
        if let Some(spread) = self.spread.as_ref().map(|e| e.__dict__(py)).transpose()? {
            dict.set_item("spread", spread)?;
        } else {
            dict.set_item("spread", py.None())?;
        }
        if let Some(aggravate) = self
            .aggravate
            .as_ref()
            .map(|e| e.__dict__(py))
            .transpose()?
        {
            dict.set_item("aggravate", aggravate)?;
        } else {
            dict.set_item("aggravate", py.None())?;
        }

        Ok(dict.into())
    }
}

impl From<MonaDamageAnalysis> for PyDamageAnalysis {
    fn from(damage_analysis: MonaDamageAnalysis) -> Self {
        let element = damage_analysis.element.to_string();
        let normal = PyDamageResult::from(damage_analysis.normal);
        let melt = damage_analysis.melt.map(PyDamageResult::from);
        let vaporize = damage_analysis.vaporize.map(PyDamageResult::from);
        let spread = damage_analysis.spread.map(PyDamageResult::from);
        let aggravate = damage_analysis.aggravate.map(PyDamageResult::from);
        Self {
            atk: damage_analysis.atk,
            atk_ratio: damage_analysis.atk_ratio,
            hp: damage_analysis.hp,
            hp_ratio: damage_analysis.hp_ratio,
            def: damage_analysis.def,
            def_ratio: damage_analysis.def_ratio,
            em: damage_analysis.em,
            em_ratio: damage_analysis.em_ratio,
            extra_damage: damage_analysis.extra_damage,
            bonus: damage_analysis.bonus,
            critical: damage_analysis.critical,
            critical_damage: damage_analysis.critical_damage,
            melt_enhance: damage_analysis.melt_enhance,
            vaporize_enhance: damage_analysis.vaporize_enhance,
            healing_bonus: damage_analysis.healing_bonus,
            shield_strength: damage_analysis.shield_strength,
            spread_compose: damage_analysis.spread_compose,
            aggravate_compose: damage_analysis.aggravate_compose,
            def_minus: damage_analysis.def_minus,
            def_penetration: damage_analysis.def_penetration,
            res_minus: damage_analysis.res_minus,
            element,
            is_heal: damage_analysis.is_heal,
            is_shield: damage_analysis.is_shield,
            normal,
            melt,
            vaporize,
            spread,
            aggravate,
        }
    }
}
