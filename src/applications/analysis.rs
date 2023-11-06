use anyhow::Context;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use mona::artifacts::{Artifact, ArtifactList};
use mona::artifacts::effect_config::{ArtifactConfigInterface, ArtifactEffectConfig};
use mona::attribute::{AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use mona::buffs::Buff;
use mona::character::Character;
use mona::damage::DamageContext;
use mona::enemies::Enemy;
use mona_wasm::applications::common::{CharacterInterface, WeaponInterface};
use mona_wasm::CalculatorInterface;
use pyo3::types::PyDict;
use pythonize::depythonize;

use crate::applications::input::calculator::PyCalculatorConfig;
use crate::applications::output::damage_analysis::PyDamageAnalysis;
use crate::applications::output::transformative_damage::PyTransformativeDamage;

#[pyfunction]
pub fn get_damage_analysis(calculator_config: PyCalculatorConfig) -> PyResult<PyDamageAnalysis> {
    let character_result: CharacterInterface = calculator_config.character.try_into()
        .map_err(|e: anyhow::Error| PyValueError::new_err(e.to_string()))?;

    let character: Character<ComplicatedAttributeGraph> = character_result.to_character();

    let weapon_result: WeaponInterface = calculator_config.weapon.try_into()
        .map_err(|e: anyhow::Error| PyValueError::new_err(e.to_string()))?;

    let weapon = weapon_result.to_weapon(&character);


    let buffs_result: Result<Vec<Box<dyn Buff<ComplicatedAttributeGraph>>>, PyValueError> =
        calculator_config.buffs.into_iter()
            .map(|buff| {
                let _buff = buff.try_into()
                    .map_err(|e: anyhow::Error| PyValueError::new_err(e.to_string()))?;
                Ok(_buff.to_buff())
            })
            .collect();

    let buffs = buffs_result?;


    let skill_config = calculator_config.skill.try_into()
        .map_err(|e| PyValueError::new_err(e.to_string()))?;


    let result = CalculatorInterface::get_damage_analysis_internal(
        &character,
        &weapon,
        &buffs,
        artifacts,
        &artifact_config,
        skill_config.index,
        &skill_config.config,
        &enemy,
        None,
    );

    Ok(PyDamageAnalysis::from(result))
}
