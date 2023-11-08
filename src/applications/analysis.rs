use anyhow::anyhow;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;

use mona::artifacts::effect_config::{ArtifactConfigInterface, ArtifactEffectConfig};
use mona::artifacts::{Artifact, ArtifactList};
use mona::attribute::{AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use mona::buffs::Buff;
use mona::character::Character;
use mona::damage::DamageContext;
use mona::enemies::Enemy;
use mona_wasm::applications::common::{
    BuffInterface, CharacterInterface, SkillInterface, WeaponInterface,
};
use mona_wasm::CalculatorInterface;
use pythonize::depythonize;

use crate::applications::input::calculator::PyCalculatorConfig;
use crate::applications::output::damage_analysis::PyDamageAnalysis;
use crate::applications::output::transformative_damage::PyTransformativeDamage;

#[pyfunction]
pub fn get_damage_analysis(calculator_config: PyCalculatorConfig) -> PyResult<PyDamageAnalysis> {
    let character: CharacterInterface = calculator_config
        .character
        .try_into()
        .map_err(|e: anyhow::Error| PyValueError::new_err(e.to_string()))?;
    let character: Character<ComplicatedAttributeGraph> = character.to_character();

    let weapon: WeaponInterface = calculator_config
        .weapon
        .try_into()
        .map_err(|e: anyhow::Error| PyValueError::new_err(e.to_string()))?;
    let weapon = weapon.to_weapon(&character);

    let artifacts = calculator_config
        .artifacts
        .into_iter()
        .map(|a| -> Result<Artifact, anyhow::Error> { a.try_into() })
        .collect::<Result<Vec<Artifact>, anyhow::Error>>()?;
    let artifacts = artifacts.iter().collect::<Vec<&Artifact>>();

    let artifact_config_interface: Option<ArtifactConfigInterface> =
        if let Some(artifact_config) = calculator_config.artifact_config {
            Python::with_gil(|py| {
                depythonize(artifact_config.as_ref(py))
                    .map_err(|err| anyhow!("Failed to deserialize artifact config: {}", err))
            })?
        } else {
            None
        };
    let artifact_config = if let Some(artifact_config) = artifact_config_interface {
        artifact_config.to_config()
    } else {
        ArtifactEffectConfig::default()
    };

    let buffs = calculator_config
        .buffs
        .into_iter()
        .map(|buff| -> Result<BuffInterface, anyhow::Error> { buff.try_into() })
        .map(|buff| match buff {
            Ok(b) => Ok(b.to_buff()),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<Box<dyn Buff<ComplicatedAttributeGraph>>>, anyhow::Error>>()?;

    let skill_config: SkillInterface = calculator_config
        .skill
        .try_into()
        .map_err(|e: anyhow::Error| PyValueError::new_err(e.to_string()))?;

    let enemy: Enemy = if let Some(enemy) = calculator_config.enemy {
        enemy.try_into()?
    } else {
        Enemy::default()
    };

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

#[pyfunction]
pub fn get_transformative_damage(
    calculator_config: PyCalculatorConfig,
) -> PyResult<PyTransformativeDamage> {
    let character: CharacterInterface = calculator_config
        .character
        .try_into()
        .map_err(|e: anyhow::Error| PyValueError::new_err(e.to_string()))?;
    let character: Character<SimpleAttributeGraph2> = character.to_character();

    let weapon: WeaponInterface = calculator_config
        .weapon
        .try_into()
        .map_err(|e: anyhow::Error| PyValueError::new_err(e.to_string()))?;
    let weapon = weapon.to_weapon(&character);

    let artifacts = calculator_config
        .artifacts
        .into_iter()
        .map(|a| -> Result<Artifact, anyhow::Error> { a.try_into() })
        .collect::<Result<Vec<Artifact>, anyhow::Error>>()?;
    let artifacts = artifacts.iter().collect::<Vec<&Artifact>>();

    let artifact_config_interface: Option<ArtifactConfigInterface> =
        if let Some(artifact_config) = calculator_config.artifact_config {
            Python::with_gil(|py| {
                depythonize(artifact_config.as_ref(py))
                    .map_err(|err| anyhow!("Failed to deserialize artifact config: {}", err))
            })?
        } else {
            None
        };
    let artifact_config = if let Some(artifact_config) = artifact_config_interface {
        artifact_config.to_config()
    } else {
        ArtifactEffectConfig::default()
    };

    let buffs = calculator_config
        .buffs
        .into_iter()
        .map(|buff| -> Result<BuffInterface, anyhow::Error> { buff.try_into() })
        .map(|buff| match buff {
            Ok(b) => Ok(b.to_buff()),
            Err(e) => Err(e),
        })
        .collect::<Result<Vec<Box<dyn Buff<SimpleAttributeGraph2>>>, anyhow::Error>>()?;

    let enemy: Enemy = if let Some(enemy) = calculator_config.enemy {
        enemy.try_into()?
    } else {
        Enemy::default()
    };

    let attribute = AttributeUtils::create_attribute_from_big_config(
        &ArtifactList {
            artifacts: &artifacts,
        },
        &artifact_config,
        &character,
        &weapon,
        &buffs,
    );

    let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
        character_common_data: &character.common_data,
        enemy: &enemy,
        attribute: &attribute,
    };

    let result = context.transformative();

    Ok(PyTransformativeDamage::from(result))
}
