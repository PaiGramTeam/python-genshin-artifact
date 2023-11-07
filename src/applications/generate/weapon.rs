use anyhow::Context;
use mona::common::item_config_type::ItemConfig;
use mona::weapon::weapon_name::WeaponName;
use mona::weapon::weapon_static_data::WeaponStaticData;
use mona_generate::gen_meta::gen_locale::get_index_mapping;
use mona_generate::utils::config_to_json;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct WeaponMetaDataForJS {
    name: String,
    internal_name: String,
    name_index: usize,
    star: usize,
    t: String,
    effect: Option<usize>,
    configs: Vec<String>,
}

#[pyfunction]
pub fn gen_weapon_meta_as_json() -> PyResult<String> {
    let mut data: Vec<WeaponMetaDataForJS> = Vec::new();
    let index_map = get_index_mapping();

    for i in 0_usize..WeaponName::LEN {
        let weapon_name: WeaponName = num::FromPrimitive::from_usize(i).unwrap();

        let meta_data: WeaponStaticData = weapon_name.get_static_data();
        let config_data: Option<&[ItemConfig]> = weapon_name.get_config_data();
        let mut configs: Vec<String> = Vec::new();

        if let Some(x) = config_data {
            for config in x.iter() {
                configs.push(config_to_json(&config));
            }
        }

        let my_data = WeaponMetaDataForJS {
            name: weapon_name.to_string(),
            // internal_name: get_internal_weapon_name(weapon_name),
            internal_name: String::from(meta_data.internal_name),
            name_index: *index_map.get(&meta_data.name_locale).unwrap(),
            star: meta_data.star,
            t: meta_data.weapon_type.to_string(),
            effect: if let Some(ref s) = meta_data.effect {
                Some(*index_map.get(s).unwrap())
            } else {
                None
            },
            configs,
        };
        data.push(my_data);
    }

    let result_str = serde_json::to_string(&data).context("Failed to serialize json")?;
    Ok(result_str)
}
