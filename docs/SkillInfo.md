# SkillInfo 类说明文档

## 类定义

SkillInfo 是一个主要用于存放所需要分析的技能。它具有以下属性：

- `index`：技能索引。
- `config`：技能配置。

## index 技能索引

可以通过 `genshin_artifact/mona_core/src/character/characters` 寻找角色索引，索引是从 0 开始。

以胡桃为例，改角色的配置文件在
`genshin_artifact/mona_core/src/character/characters/pyro/hu_tao.rs` 中。

其中 `SKILL_MAP` 定义了**能够计算**技能的相关数据。

```rust
    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal1 as usize, text: hit_n_dmg!(1) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal2 as usize, text: hit_n_dmg!(2) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal3 as usize, text: hit_n_dmg!(3) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal4 as usize, text: hit_n_dmg!(4) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal51 as usize, text: hit_n_dmg!(5, 1) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal52 as usize, text: hit_n_dmg!(5, 2) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Normal6 as usize, text: hit_n_dmg!(6) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Charged as usize, text: charged_dmg!() },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Plunging1 as usize, text: plunging_dmg!(1) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Plunging2 as usize, text: plunging_dmg!(2) },
            CharacterSkillMapItem { index: HuTaoDamageEnum::Plunging3 as usize, text: plunging_dmg!(3) },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: HuTaoDamageEnum::ElementalSkillBloodBlossom as usize, text: locale!(zh_cn: "血梅香伤害", en: "Blood Blossom DMG") }
        ]),
        skill3: Some(&[
            CharacterSkillMapItem { index: HuTaoDamageEnum::ElementalBurst1 as usize, text: locale!(zh_cn: "技能伤害", en: "Skill DMG") },
            CharacterSkillMapItem { index: HuTaoDamageEnum::ElementalBurstLow1 as usize, text: locale!(zh_cn: "低血量时技能伤害", en: "Low HP Skill DMG") },
        ]),
    };
```

从代码可以看出，如果我们需要获得胡桃在开启大招是低血量时技能伤害，
我们锁定其 `HuTaoDamageEnum::ElementalBurstLow1` 从上往下数，这是第 13 个，因为索引是从0开始，故赋值的时候需要减去 1 。

```python
index = 12
```

**注意：莫娜计算器并不计算相关命座的技能等级加成，这需要前端作出相应的计算 ~~（人话：者用户自己解决）~~**

## config 技能配置

这个参数设定了一些角色当前的状态，比如说胡桃是否E技能是否开启，如果开启伤害会提高。

在 `genshin_artifact/mona_core/src/character/skill_config.rs` 文件我们可以看见各个角色的技能参数

```rust
#[derive(Serialize, Deserialize, Debug)]
pub enum CharacterSkillConfig {
    Albedo { fatal_count: usize },
    Aloy { coil_count: usize },
    AratakiItto { after_q: bool },
    Diluc { pyro: bool },
    Eula { lightfall_stack: usize },
    Ganyu { talent1_rate: f64 },
    HuTao { after_e: bool },
    KaedeharaKazuha { after_e_or_q: bool },
    KamisatoAyaka { #[serde(default = "default_true")] after_dash: bool, #[serde(default = "default_false")] use_c6: bool },
    KamisatoAyato { e_stack: usize, in_q: bool },
    Keqing { after_e: bool },
    Noelle { after_q: bool },
    RaidenShogun { under_e: bool, resolve_stack: usize },
    SangonomiyaKokomi { after_q: bool },
    Xiao { after_q: bool, talent1_stack: f64, talent2_stack: f64 },
    Xingqiu { c4: bool },
    Xinyan { shield_rate: f64 },
    Yanfei { after_q: bool },
    Yoimiya { after_e: bool },
    Dori { c6: bool },
    Candace { crown: bool },
    Cyno { under_judication: bool },
    Nahida { q_bonus: bool, q_bonus_count: usize },
    Wanderer { e_enabled: bool, e_hydro: bool, sdpoints: f64 },
    Faruzan { talent2_ratio: f64 },
    Alhaitham { under_e: bool },
    Dehya { c2_rate: f64, c6_stack: f64 },
    Kaveh { after_q: bool },
    Freminet { talent2_rate: f64 },
    Lyney { prop_stack: f64, under_pyro: bool, pyro_count: usize, },
    Neuvillette { talent1_stack: usize },
    Wriothesley { under_chilling_penalty: bool },
    NoConfig,
}
```

以胡桃为例，如果要设置当前E技能状态生效，我们传入的参数是

```python
params = {"after_e": True}
```

如不进行任何设置，保持该参数不会被任何代码修改，或者设置为 `NoConfig`

```python
config = "NoConfig"
```
