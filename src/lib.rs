extern crate core;
mod applications;

use pyo3::import_exception;
use pyo3::prelude::*;

use crate::applications::errors::ValidationError;
use applications::analysis::{get_damage_analysis, get_transformative_damage};
use applications::generate::artifact::gen_artifact_meta_as_json;
use applications::generate::character::gen_character_meta_as_json;
use applications::generate::locale::gen_generate_locale_as_json;
use applications::generate::weapon::gen_weapon_meta_as_json;

use crate::applications::input::artifact::PyArtifact;
use crate::applications::input::buff::PyBuffInterface;
use crate::applications::input::calculator::PyCalculatorConfig;
use crate::applications::input::character::PyCharacterInterface;
use crate::applications::input::enemy::PyEnemyInterface;
use crate::applications::input::skill::PySkillInterface;
use crate::applications::input::weapon::PyWeaponInterface;
use crate::applications::output::damage_analysis::PyDamageAnalysis;
use crate::applications::output::damage_result::PyDamageResult;
use crate::applications::output::transformative_damage::PyTransformativeDamage;

import_exception!(json, JSONDecodeError);

#[pymodule]
fn _python_genshin_artifact(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add("JSONDecodeError", py.get_type::<JSONDecodeError>())?;
    m.add_function(wrap_pyfunction!(get_damage_analysis, m)?)?;
    m.add_function(wrap_pyfunction!(get_transformative_damage, m)?)?;
    m.add_function(wrap_pyfunction!(gen_character_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_weapon_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_artifact_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_generate_locale_as_json, m)?)?;
    m.add_class::<PyCalculatorConfig>()?;
    m.add_class::<PyCharacterInterface>()?;
    m.add_class::<PyBuffInterface>()?;
    m.add_class::<PyWeaponInterface>()?;
    m.add_class::<PyTransformativeDamage>()?;
    m.add_class::<PySkillInterface>()?;
    m.add_class::<PyEnemyInterface>()?;
    m.add_class::<PyArtifact>()?;
    m.add_class::<PyDamageResult>()?;
    m.add_class::<PyDamageAnalysis>()?;
    m.add_class::<ValidationError>()?;
    Ok(())
}
