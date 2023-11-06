extern crate core;
mod applications;

use pyo3::import_exception;
use pyo3::prelude::*;

use applications::generate::artifact::gen_artifact_meta_as_json;
use applications::generate::character::gen_character_meta_as_json;
use applications::generate::locale::gen_generate_locale_as_json;
use applications::generate::weapon::gen_weapon_meta_as_json;
use applications::wasm::{get_damage_analysis, get_transformative_damage};

use crate::applications::input::buff::PyBuffInterface;
use crate::applications::input::calculator::CalculatorConfig;
use crate::applications::input::character::PyCharacterInterface;
use crate::applications::input::weapon::PyWeaponInterface;
use crate::applications::output::transformative_damage::PyTransformativeDamage;

import_exception!(json, JSONDecodeError);

#[pymodule]
fn genshin_artifact_core(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("JSONDecodeError", py.get_type::<JSONDecodeError>())?;
    m.add_function(wrap_pyfunction!(get_damage_analysis, m)?)?;
    m.add_function(wrap_pyfunction!(get_transformative_damage, m)?)?;
    m.add_function(wrap_pyfunction!(gen_character_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_weapon_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_artifact_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_generate_locale_as_json, m)?)?;
    m.add_class::<CalculatorConfig>()?;
    m.add_class::<PyCharacterInterface>()?;
    m.add_class::<PyBuffInterface>()?;
    m.add_class::<PyWeaponInterface>()?;
    m.add_class::<PyTransformativeDamage>()?;
    Ok(())
}
