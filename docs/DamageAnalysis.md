# DamageAnalysis 类说明文档

## 类定义

DamageAnalysis 是一个主要用于存放计算伤害结果的类。它具有以下属性：

- `atk`：总攻击力数值。
- `atk_ratio`：攻击力倍率。
- `hp`：总生命值数值。
- `hp_ratio`：生命值倍率。
- `def`：总防御力数值。
- `def_ratio`：防御力倍率。
- `em`：总元素精通数值。
- `em_ratio`：元素精通倍率。
- `extra_damage`：基础伤害区加成。
- `bonus`：增伤区加成。
- `critical`：暴击率加成。
- `critical_damage`：暴击伤害加成。
- `melt_enhance`：融化反应伤害提高。
- `vaporize_enhance`：蒸发反应伤害提高。
- `healing_bonus`：治疗效果加成。
- `shield_strength`：护盾强效加成。
- `spread_compose`：扩散反应加成。
- `aggravate_compose`：剧变反应加成。
- `def_minus`：百分比减防。
- `def_penetration`：无视防御。
- `res_minus`：减抗。
- `element`：伤害类型，元素/物理，字符串。
- `is_heal`：是否为治疗效果，布尔值类型。
- `is_shield`：是否为护盾效果，布尔值类型。
- `normal`：普通伤害结果，DamageResult 类型。
- `melt`：融化伤害结果，DamageResult 类型，可选。
- `vaporize`：蒸发伤害结果，DamageResult 类型，可选。
- `spread`：扩散伤害结果，DamageResult 类型，可选。
- `aggravate`：加剧伤害结果，DamageResult 类型，可选。

每个面板数值的键值对的形式，键为加成的描述，值为加成的数值，如无特别说明，则为浮点数类型，百分比加成基准为 1。

## JSON

```json
{
    "atk": {
        "胡桃：彼岸蝶舞": 1224.2534400000002,
        "武器基础攻击": 608.0,
        "护摩之杖被动等效": 149.2992,
        "角色基础攻击": 106.0
    },
    "atk_ratio": {
        "技能倍率": 2.5647
    },
    "hp": {
        "角色基础生命": 15552.0,
        "护摩之杖被动": 3110.4
    },
    "hp_ratio": {},
    "def": {
        "角色基础防御": 876.0
    },
    "def_ratio": {},
    "em": {},
    "em_ratio": {},
    "extra_damage": {},
    "bonus": {},
    "critical": {
        "初始值": 0.05
    },
    "critical_damage": {
        "角色副属性": 0.384,
        "武器副词条": 0.662,
        "初始值": 0.5
    },
    "melt_enhance": {},
    "vaporize_enhance": {},
    "healing_bonus": {},
    "shield_strength": {},
    "spread_compose": {},
    "aggravate_compose": {},
    "def_minus": {},
    "def_penetration": {},
    "res_minus": {},
    "element": "Pyro",
    "is_heal": false,
    "is_shield": false,
    "normal": {
        "critical": 6134.016225279227,
        "non_critical": 2409.2758151136004,
        "expectation": 2595.512835621881,
        "is_heal": false,
        "is_shield": false
    },
    "melt": {
        "critical": 12268.032450558454,
        "non_critical": 4818.551630227201,
        "expectation": 5191.025671243762,
        "is_heal": false,
        "is_shield": false
    },
    "vaporize": {
        "critical": 9201.02433791884,
        "non_critical": 3613.9137226704006,
        "expectation": 3893.2692534328216,
        "is_heal": false,
        "is_shield": false
    },
    "spread": null,
    "aggravate": null
}
```