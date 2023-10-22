# CharacterInfo 类说明文档

## 类定义

CharacterInfo 是一个主要用于存放角色数据。它具有以下属性：

- `name`：角色名称。
- `level`：角色等级。
- `ascend`：是否为突破等级，如80级突破时该选项为 `true`。
- `constellation`：命座。
- `skill1`：A技能等级。
- `skill2`：E技能等级。
- `skill3`：Q技能等级。
- `params`：角色参数。

## skill 等级说明 

因为莫娜的代码，实际传递给计算器的传递的技能等级，为

```
传递的技能等级 = 角色技能等级 - 1
```

信息来源：

`genshin_artifact/src/composables/character.ts` L51 - L63

```typescript
    const characterInterface = computed(() => {
        let i = {
            name: characterName.value,
            level: characterLevelNumber.value,
            ascend: characterAscend.value,
            constellation: characterConstellation.value,
            skill1: characterSkill1.value - 1,
            skill2: characterSkill2.value - 1,
            skill3: characterSkill3.value - 1,
            params: characterConfig.value
        }
        return i
    })

```

## params 参数说明

这个参数设定了一些角色当前的状态，比如说胡桃是否半血状态，如果是半血状态会激活胡桃的血之灶火天赋，使其在生效期间火元素伤害增加 33% 。

在 `genshin_artifact/mona_core/src/character/character_config.rs` 文件我们可以看见各个角色的参数

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum CharacterConfig {
    Ganyu { talent2_rate: f64 },
    HuTao { le_50: bool },
    KamisatoAyaka { talent1_rate: f64, talent2_rate: f64 },
    Keqing { talent2_rate: f64 },
    KukiShinobu { hp_le_50: bool, use_c6: bool },
    Ningguang { talent2_rate: f64 },
    Rosaria { e_from_behind: bool },
    Razor { e_stack: f64, talent2_ratio: f64 },
    Yelan { team_element_count: usize },
    Yoimiya { talent1_level: f64 },
    Collei { background: bool },
    Tighnari { talent1_ratio: f64, c2_ratio: f64 },
    Cyno { c2_stack: f64, after_q: bool },
    Nilou { golden_rate: f64 },
    Candace { c2_rate: f64 },
    Nahida { c4_e_count: usize },
    Wanderer { e_pyro: bool, e_cryo: bool },
    Faruzan { q_ratio: f64 },
    Yaoyao { c4_rate: f64 },
    Alhaitham { c2_stack: f64, c4_stack: f64, c6_rate: f64 },
    Kaveh { talent2_stack: f64, c2_rate: f64 },
    Baizhu { hp_below_50: bool },
    Lynette { talent1_rate: f64, talent1_count: usize, talent2_rate: f64 },
    Freminet { c4_stack: f64, c6_stack: f64 },
    Lyney { c2_stack: f64, c4_rate: f64 },
    Neuvillette { current_hp: usize },
    Wriothesley { talent2_stack: f64 },
    NoConfig,
}
```

以胡桃为例，如果要设置当前角色的状态为半血，我们传入的参数是

```python
params = {"le_50": True}
```

如不进行任何设置，保持该参数不会被任何代码修改，或者设置为 `NoConfig`

```python
params = "NoConfig"
```


