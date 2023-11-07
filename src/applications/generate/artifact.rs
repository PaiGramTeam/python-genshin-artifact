use anyhow::Context;
use mona::artifacts::artifact_trait::ArtifactMetaData;
use mona::artifacts::ArtifactSetName;
use mona::common::item_config_type::ItemConfig;
use mona_generate::gen_meta::gen_locale::get_index_mapping;
use mona_generate::utils::config_to_json;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ArtifactMeta {
    name_locale: usize,
    name: String,
    name_mona: String,
    min_star: usize,
    max_star: usize,
    effect1: Option<usize>,
    effect2: Option<usize>,
    effect3: Option<usize>,
    effect4: Option<usize>,
    effect5: Option<usize>,
    config4: Vec<String>,

    flower: Option<usize>,
    feather: Option<usize>,
    sand: Option<usize>,
    goblet: Option<usize>,
    head: Option<usize>,

    flower_icon: String,
    feather_icon: String,
    sand_icon: String,
    goblet_icon: String,
    head_icon: String,
}

#[pyfunction]
pub fn gen_artifact_meta_as_json() -> PyResult<String> {
    let mut data = Vec::new();
    let index_map = get_index_mapping();

    for i in 1_usize..ArtifactSetName::LEN {
        let e: ArtifactSetName = num::FromPrimitive::from_usize(i).unwrap();
        let meta: ArtifactMetaData = e.get_meta();
        let config4: Option<&'static [ItemConfig]> = e.get_config4();

        data.push(ArtifactMeta {
            name_locale: *index_map.get(&meta.name_locale).unwrap(),
            name: meta.name.to_string(),
            name_mona: String::from(meta.name_mona),
            min_star: meta.star.0,
            max_star: meta.star.1,
            effect1: if let Some(ref x) = meta.effect1 {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            effect2: if let Some(ref x) = meta.effect2 {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            effect3: if let Some(ref x) = meta.effect3 {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            effect4: if let Some(ref x) = meta.effect4 {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            effect5: if let Some(ref x) = meta.effect5 {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            config4: config4
                .unwrap_or(&[])
                .iter()
                .map(|x| config_to_json(x))
                .collect(),
            flower: if let Some(ref x) = meta.flower {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            feather: if let Some(ref x) = meta.feather {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            sand: if let Some(ref x) = meta.sand {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            goblet: if let Some(ref x) = meta.goblet {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            head: if let Some(ref x) = meta.head {
                Some(*index_map.get(x).unwrap())
            } else {
                None
            },
            flower_icon: if meta.flower.is_some() {
                format!("UI_RelicIcon_{}_4", meta.internal_id)
            } else {
                String::default()
            },
            feather_icon: if meta.feather.is_some() {
                format!("UI_RelicIcon_{}_2", meta.internal_id)
            } else {
                String::default()
            },
            sand_icon: if meta.sand.is_some() {
                format!("UI_RelicIcon_{}_5", meta.internal_id)
            } else {
                String::default()
            },
            goblet_icon: if meta.goblet.is_some() {
                format!("UI_RelicIcon_{}_1", meta.internal_id)
            } else {
                String::default()
            },
            head_icon: if meta.head.is_some() {
                format!("UI_RelicIcon_{}_3", meta.internal_id)
            } else {
                String::default()
            },
        })
    }

    let result_str = serde_json::to_string(&data).context("Failed to serialize json")?;
    Ok(result_str)
}
