# TransformativeDamage 类说明文档

## 类定义

DamageAnalysis 是一个主要用于存放剧变反应伤害。它具有以下属性：

- `swirl_cryo`：扩散（冰）伤害值，浮点型。
- `swirl_hydro`：扩散（水）伤害值，浮点型。
- `swirl_pyro`：扩散（火）伤害值，浮点型。
- `swirl_electro`：扩散（雷）伤害值，浮点型。
- `overload`：超载伤害值，浮点型。
- `electro_charged`：感电伤害值，浮点型。
- `shatter`：碎冰伤害值，浮点型。
- `super_conduct`：超导伤害值，浮点型。
- `bloom`：绽放伤害值，浮点型。
- `hyper_bloom`：烈绽放伤害值，浮点型。
- `burgeon`：超绽放伤害值，浮点型。
- `burning`：燃烧伤害值，浮点型。
- `crystallize`：结晶盾伤害值，浮点型。

## JSON

```json
{
    "swirl_cryo": 1348.1636027931086,
    "swirl_hydro": 1348.1636027931086,
    "swirl_pyro": 1348.1636027931086,
    "swirl_electro": 1348.1636027931086,
    "overload": 5535.613207227028,
    "electro_charged": 2696.327205586217,
    "shatter": 3370.4090069827716,
    "super_conduct": 1123.469668994257,
    "bloom": 4493.878675977028,
    "hyper_bloom": 6740.818013965543,
    "burgeon": 8303.419810840544,
    "burning": 691.9516509033785,
    "crystallize": 2373.8421050673833
}
```