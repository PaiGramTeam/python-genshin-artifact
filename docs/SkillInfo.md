# SkillInfo 类说明文档

## 类定义

SkillInfo 是一个主要用于存放所需要分析的技能的类。它具有以下属性：

- `index`：技能索引。
- `config`：技能配置。

## index 技能索引

可以运行 `genshin_genshin_artifact/tools/get_character_skill_index.py` 寻找角色索引。

```
K:\PycharmProjects\python_genshin_artifact\venv\Scripts\python.exe K:\PycharmProjects\python_genshin_artifact\tools\get_character_skill_index.py 
Enter character name: HuTao
Character name 胡桃
Skill A 普通攻击·往生秘传枪法
index 0 一段伤害
index 1 二段伤害
index 2 三段伤害
index 3 四段伤害
index 4 五段伤害-1
index 5 五段伤害-2
index 6 六段伤害
index 7 重击伤害
index 8 下坠期间伤害
index 9 低空坠地冲击伤害
index 10 高空坠地冲击伤害
Skill E 蝶引来生
index 11 血梅香伤害
Skill Q 安神秘法
index 12 技能伤害
index 13 低血量时技能伤害

Process finished with exit code 0
```

从输出可以看出，如果我们需要获得胡桃在低血量时开启大招的技能伤害，索引应该设置为 13。

```python
index = 13
```

**注意：莫娜计算器并不计算相关命座的技能等级加成，这需要前端作出相应的计算 ~~（人话：用户自己解决）~~**

## config 技能配置

这个参数设定了一些角色当前的状态，比如说胡桃 E 技能是否开启，如果开启会基于生命值获得额外攻击力，打出更高的伤害。

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

以胡桃为例，如果要设置当前 E 技能状态生效，我们传入的参数是

```python
params = {"after_e": True}
```

如不进行任何设置，保持该参数不会被任何代码修改，或者设置为 `NoConfig`

```python
config = "NoConfig"
```
