import json
from python_genshin_artifact import (
    get_damage_analysis,
    gen_character_meta_as_json,
    gen_weapon_meta_as_json,
    gen_artifact_meta_as_json,
    gen_generate_locale_as_json,
)

data = {
    "character": {
        "name": "HuTao",
        "level": 90,
        "ascend": True,
        "constellation": 0,
        "skill1": 10,
        "skill2": 10,
        "skill3": 10,
        "params": "NoConfig",
    },
    "weapon": {
        "name": "StaffOfHoma",
        "level": 90,
        "ascend": True,
        "refine": 1,
        "params": "NoConfig",
    },
    "buffs": [],
    "artifacts": [],
    "artifact_config": None,
    "skill": {"index": 1, "config": {"HuTao": {"after_e": True}}},
    "enemy": None,
}

# character_meta = gen_character_meta_as_json()
# with open("character.json", mode="w+", encoding="utf-8") as f:
#     f.write(character_meta)
# print(character_meta)
#
# weapon_meta = gen_weapon_meta_as_json()
# with open("weapon.json", mode="w+", encoding="utf-8") as f:
#     f.write(weapon_meta)
#
# artifact_meta = gen_artifact_meta_as_json()
# with open("artifact.json", mode="w+", encoding="utf-8") as f:
#     f.write(artifact_meta)
#
# locale_meta = gen_generate_locale_as_json("zh-cn")
# with open("locale_zh_cn.json", mode="w+", encoding="utf-8") as f:
#     f.write(locale_meta)


damage_analysis = get_damage_analysis(json.dumps(data))
data: dict = json.loads(damage_analysis)
for key, value in data.items():
    if value:
        print(key)
        print(value)
