use std::collections::HashMap;
use pyo3::prelude::*;
use crate::applications::output::damage_result::PyDamageResult;
use mona::damage::DamageAnalysis as MonaDamageAnalysis;

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
    #[pyo3(get, set)]
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
