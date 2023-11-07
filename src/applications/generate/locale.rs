use mona_generate::gen_meta::gen_locale::generate_locale_vec;
use pyo3::prelude::*;

#[pyfunction]
pub fn gen_generate_locale_as_json(loc: String) -> PyResult<String> {
    let json = serde_json::to_string_pretty(&generate_locale_vec(&loc)).unwrap();
    Ok(json)
}
