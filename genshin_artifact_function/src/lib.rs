extern crate core;
mod applications;

use applications::generate::artifact::gen_artifact_meta_as_json;
use applications::generate::character::gen_character_meta_as_json;
use applications::generate::locale::gen_generate_locale_as_json;
use applications::generate::weapon::gen_weapon_meta_as_json;
use applications::wasm::{get_damage_analysis, get_transformative_damage};
use pyo3::prelude::*;

#[pymodule]
fn genshin_artifact_function(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_damage_analysis, m)?)?;
    m.add_function(wrap_pyfunction!(get_transformative_damage, m)?)?;
    m.add_function(wrap_pyfunction!(gen_character_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_weapon_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_artifact_meta_as_json, m)?)?;
    m.add_function(wrap_pyfunction!(gen_generate_locale_as_json, m)?)?;
    Ok(())
}
