use mona::artifacts::Artifact;
use mona::artifacts::effect_config::ArtifactConfigInterface;
use mona_wasm::applications::common::{BuffInterface, CharacterInterface, EnemyInterface, SkillInterface, WeaponInterface};
use pyo3::prelude::*;

#[derive(Debug, Clone)]
#[pyclass]
pub struct CalculatorConfig {
    pub character: CharacterInterface,
    // pub weapon: WeaponInterface,
    // pub buffs: Vec<BuffInterface>,
    // pub artifacts: Vec<Artifact>,
    // pub artifact_config: Option<ArtifactConfigInterface>,
    // pub skill: SkillInterface,
    // pub enemy: Option<EnemyInterface>,
}

#[pymethods]
impl CalculatorConfig {
    #[new]
    // #[args(buffs="Vec::new()", artifacts="Vec::new()", artifact_config="None", enemy="None")]
    fn new(character: CharacterInterface,
           // weapon: WeaponInterface,
           // buffs: Vec<BuffInterface>,
           // artifacts: Vec<Artifact>,
           // artifact_config: Option<ArtifactConfigInterface>,
           // skill: SkillInterface,
           // enemy: Option<EnemyInterface>,
    ) -> Self {
        CalculatorConfig {
            character,
            // weapon,
            // buffs,
            // artifacts,
            // artifact_config,
            // skill,
            // enemy,
        }
    }
}
