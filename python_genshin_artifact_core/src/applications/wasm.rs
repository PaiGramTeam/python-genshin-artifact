use mona::artifacts::effect_config::{ArtifactConfigInterface, ArtifactEffectConfig};
use mona::artifacts::{Artifact, ArtifactList};
use mona::attribute::{AttributeUtils, ComplicatedAttributeGraph, SimpleAttributeGraph2};
use mona::buffs::Buff;
use mona::character::Character;
use mona::damage::transformative_damage::TransformativeDamage;
use mona::damage::DamageContext;
use mona::enemies::Enemy;
use mona::utils;
use mona_wasm::applications::common::{
    BuffInterface, CharacterInterface, EnemyInterface, SkillInterface, WeaponInterface,
};
use mona_wasm::CalculatorInterface;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CalculatorConfigInterface {
    pub character: CharacterInterface,
    pub weapon: WeaponInterface,
    pub buffs: Vec<BuffInterface>,
    pub artifacts: Vec<Artifact>,
    pub artifact_config: Option<ArtifactConfigInterface>,
    pub skill: SkillInterface,
    pub enemy: Option<EnemyInterface>,
}

#[derive(Serialize, Deserialize)]
struct TransformativeDamageBridge {
    swirl_cryo: f64,
    swirl_hydro: f64,
    swirl_pyro: f64,
    swirl_electro: f64,
    overload: f64,
    electro_charged: f64,
    shatter: f64,
    superconduct: f64,
    bloom: f64,
    hyperbloom: f64,
    burgeon: f64,
    burning: f64,
    crystallize: f64,
}

impl From<TransformativeDamage> for TransformativeDamageBridge {
    fn from(damage: TransformativeDamage) -> Self {
        Self {
            swirl_cryo: damage.swirl_cryo,
            swirl_hydro: damage.swirl_hydro,
            swirl_pyro: damage.swirl_pyro,
            swirl_electro: damage.swirl_electro,
            overload: damage.overload,
            electro_charged: damage.electro_charged,
            shatter: damage.shatter,
            superconduct: damage.superconduct,
            bloom: damage.bloom,
            hyperbloom: damage.hyperbloom,
            burgeon: damage.burgeon,
            burning: damage.burning,
            crystallize: damage.crystallize,
        }
    }
}

#[pyfunction]
pub fn get_damage_analysis(value_str: String) -> PyResult<String> {
    let input: CalculatorConfigInterface = serde_json::from_str(&*value_str).unwrap();

    let character: Character<ComplicatedAttributeGraph> = input.character.to_character();
    let weapon = input.weapon.to_weapon(&character);

    let buffs: Vec<Box<dyn Buff<ComplicatedAttributeGraph>>> =
        input.buffs.iter().map(|x| x.to_buff()).collect();
    let artifacts: Vec<&Artifact> = input.artifacts.iter().collect();

    let artifact_config = match input.artifact_config {
        Some(x) => x.to_config(),
        None => ArtifactEffectConfig::default(),
    };

    let enemy = if let Some(x) = input.enemy {
        x.to_enemy()
    } else {
        Enemy::default()
    };

    let result = CalculatorInterface::get_damage_analysis_internal(
        &character,
        &weapon,
        &buffs,
        artifacts,
        &artifact_config,
        input.skill.index,
        &input.skill.config,
        &enemy,
        None,
    );

    let result_str = serde_json::to_string(&result).unwrap();
    Ok(result_str)
}

#[pyfunction]
pub fn get_transformative_damage(value_str: String) -> PyResult<String> {
    utils::set_panic_hook();
    let input: CalculatorConfigInterface = serde_json::from_str(&*value_str).unwrap();

    let character: Character<SimpleAttributeGraph2> = input.character.to_character();
    let weapon = input.weapon.to_weapon(&character);

    let buffs: Vec<_> = input.buffs.iter().map(|x| x.to_buff()).collect();
    let artifacts: Vec<&Artifact> = input.artifacts.iter().collect();

    let artifact_config = match input.artifact_config {
        Some(x) => x.to_config(),
        None => ArtifactEffectConfig::default(),
    };

    let enemy = if let Some(x) = input.enemy {
        x.to_enemy()
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
    let bridge = TransformativeDamageBridge::from(result);
    let result_str = serde_json::to_string(&bridge).unwrap();
    Ok(result_str)
}
