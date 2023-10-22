# CalculatorConfig 类说明文档

## 类定义

CalculatorConfig 是一个主要用于存放计算所需要的数据给伤害计算器进行分析。它具有以下属性：

- `index`：角色信息。
- `weapon`：武器信息。
- `buffs`：BUFF，如角色引发的BUFF、武器引发的BUFF、圣遗物引发的BUFF、元素共鸣、自定义BUFF。
- `artifacts`：圣遗物列表。
- `artifact_config`：圣遗物效果配置，如魔女四件套的效果。
- `skill`：需要计算的技能。
- `enemy`：怪物信息。

## artifact_config 

圣遗物遗器效果来源于 
`genshin_artifact/mona_core/src/artifacts/effect_config.rs`
在 `ArtifactEffectConfig` 结构体中表示了各个圣遗物的效果配置

```rust
#[derive(Default, Debug, Clone)]
#[derive(Serialize, Deserialize)]
pub struct ArtifactEffectConfig {
    pub config_archaic_petra: ConfigArchaicPetra,
    pub config_berserker: ConfigRate,
    pub config_blizzard_strayer: ConfigBlizzardStrayer,
    pub config_bloodstained_chivalry: ConfigRate,
    pub config_brave_heart: ConfigRate,
    pub config_crimson_witch_of_flames: ConfigLevel,
    pub config_heart_of_depth: ConfigRate,
    pub config_husk_of_opulent_dreams: ConfigLevel,
    pub config_instructor: ConfigRate,
    pub config_lavawalker: ConfigRate,
    pub config_martial_artist: ConfigRate,
    pub config_noblesse_oblige: ConfigRate,
    pub config_pale_flame: ConfigPaleFlame,
    pub config_retracing_bolide: ConfigRate,
    pub config_shimenawas_reminiscence: ConfigRate,
    pub config_tenacity_of_the_millelith: ConfigRate,
    pub config_thundersoother: ConfigRate,
    pub config_vermillion_hereafter: ConfigVermillionHereafter,
    pub config_echoes_of_an_offering: ConfigEchoesOfAnOffering,
    pub config_deepwood_memories: ConfigRate,
    pub config_gilded_dreams: ConfigGildedDreams,
    pub config_desert_pavilion_chronicle: ConfigRate,
    pub config_flower_of_paradise_lost: ConfigFlowerOfParadiseLost,
    pub config_nymphs_dream: ConfigNymphsDream,
    pub config_vourukashas_glow: ConfigVourukashasGlow,
    pub config_marechaussee_hunter: ConfigMarechausseeHunter,
    pub config_golden_troupe: ConfigRate,
}
```