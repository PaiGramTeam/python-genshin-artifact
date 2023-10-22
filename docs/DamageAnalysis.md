# DamageAnalysis 类说明文档

## 类定义

DamageAnalysis 是一个基于 BaseModel 的类，主要用于分析和计算伤害。它具有以下属性：

- `atk`：攻击力相关的加成。
- `atk_ratio`：攻击力比例加成。
- `hp`：生命值相关的加成。
- `hp_ratio`：生命值比例加成。
- `def`：防御力相关的加成。
- `def_ratio`：防御力比例加成。
- `em`：元素精通相关的加成。
- `em_ratio`：元素精通比例加成。
- `extra_damage`：额外伤害加成。
- `bonus`：元素/物理伤害加成。
- `critical`：暴击率加成。
- `critical_damage`：暴击伤害加成。
- `melt_enhance`：融化反应伤害加成。
- `vaporize_enhance`：蒸发反应伤害加成。
- `healing_bonus`：治疗效果加成。
- `shield_strength`：护盾强效加成。
- `spread_compose`：扩散反应加成。
- `aggravate_compose`：剧变反应加成。
- `def_minus`：防御力减少加成。
- `def_penetration`：防御穿透加成。
- `res_minus`：抗性减少加成。
- `element`：元素类型。
- `is_heal`：是否为治疗效果，布尔值类型。
- `is_shield`：是否为护盾效果，布尔值类型。
- `normal`：普通伤害结果。
- `melt`：融化伤害结果，可选。
- `vaporize`：蒸发伤害结果，可选。
- `spread`：扩散伤害结果，可选。
- `aggravate`：加剧伤害结果，可选。

每个面板数值的键值对的形式，键为加成的描述，值为加成的数值。

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