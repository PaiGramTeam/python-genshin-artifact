import re
from typing import Tuple, List, Optional

from python_genshin_artifact.enka.artifacts import artifacts_name_map, equip_type_map
from python_genshin_artifact.enka.assets import Assets
from python_genshin_artifact.enka.characters import characters_map
from python_genshin_artifact.enka.fight import fight_map
from python_genshin_artifact.enka.weapon import weapon_name_map
from python_genshin_artifact.models.artifact import ArtifactInfo
from python_genshin_artifact.models.characterInfo import CharacterInfo
from python_genshin_artifact.models.weapon import WeaponInfo

assets = Assets()


def is_ascend(level: int, promote_level: int) -> bool:
    if level < 20:
        return False
    if 20 <= level < 40:
        return promote_level >= 1
    expected_promote_level = level // 10 - 2
    return promote_level >= expected_promote_level


def enka_parser(data: dict, avatar_id: int) -> Tuple[CharacterInfo, WeaponInfo, List[ArtifactInfo]]:
    character_info = assets.get_character(avatar_id)
    if character_info is None:
        raise RuntimeError
    _avatar_info_list = data["avatarInfoList"]
    _avatar_info: dict = {}
    for _value in _avatar_info_list:
        if _value["avatarId"] == avatar_id:
            _avatar_info = _value
            break
    else:
        raise ValueError(f"avatarId={avatar_id} is not found")
    _prop_map = _avatar_info.get("propMap", {})
    level = int(_prop_map["4001"].get("ival", 0)) if "4001" in _prop_map else 0
    talent_id_list = _avatar_info.get("talentIdList", [])  # 命之座 ID 列 如果未解锁任何命之座则此数据不存在
    _skill_level_map: "dict" = _avatar_info["skillLevelMap"]
    skill_info = {"skill1": 1, "skill2": 1, "skill3": 1}
    for _index, _value in enumerate(character_info["ProudMap"]):
        _level = _skill_level_map.get(str(_value))
        _key_name = "skill" + str(_index + 1)
        skill_info[_key_name] = _level - 1  # mona 的角色等级是从 0 开始并非 1 估计是使用了List导致的
    for _index, _value in enumerate(character_info["Consts"]):
        if "UI_Talent_U_" in _value and len(talent_id_list) > _index:
            if _value.endswith("01"):
                skill_info["skill2"] += 3
            if _value.endswith("02"):
                skill_info["skill3"] += 3
    character_name = characters_map.get(avatar_id)
    character = CharacterInfo(
        name=character_name,
        level=level,
        constellation=len(talent_id_list),
        ascend=False,
        skill1=skill_info["skill1"],
        skill2=skill_info["skill2"],
        skill3=skill_info["skill3"],
    )
    _equip_list = _avatar_info["equipList"]
    weapon, artifacts = de_equip_list(_equip_list)

    return character, weapon, artifacts


def de_equip_list(equip_list: list[dict]) -> Tuple[WeaponInfo, List[ArtifactInfo]]:
    weapon: Optional[WeaponInfo] = None
    artifacts: List[ArtifactInfo] = []
    for _equip in equip_list:
        _weapon = _equip.get("weapon")
        _reliquary = _equip.get("reliquary")
        if _reliquary:
            sub_stats: List[Tuple[str, float]] = []
            _flat = _equip["flat"]
            artifact_id = int(re.findall(r"\d+", _flat["icon"])[0])
            set_name = artifacts_name_map.get(artifact_id)
            if set_name is None:
                raise RuntimeError
            _level = _reliquary["level"] - 1
            _reliquary_main_stat = _flat["reliquaryMainstat"]
            _main_prop_id = _reliquary_main_stat["mainPropId"]
            stat_name = fight_map[_main_prop_id]
            stat_value = _reliquary_main_stat["statValue"]
            if "_PERCENT" in _main_prop_id:
                stat_value /= 100
            elif "FIGHT_PROP_CRITICAL" in _main_prop_id:
                stat_value /= 100
            elif "FIGHT_PROP_CRITICAL_HURT" in _main_prop_id:
                stat_value /= 100
            elif "_ADD_HURT" in _main_prop_id:
                stat_value /= 100
            _main_stat = (stat_name, stat_value)
            for _reliquary_sub_stats in _flat["reliquarySubstats"]:
                _append_prop_id = _reliquary_sub_stats["appendPropId"]
                stat_name = fight_map[_append_prop_id]
                stat_value = _reliquary_sub_stats["statValue"]
                if "_PERCENT" in _append_prop_id:
                    stat_value /= 100
                elif "FIGHT_PROP_CRITICAL" in _append_prop_id:
                    stat_value /= 100
                elif "FIGHT_PROP_CRITICAL_HURT" in _append_prop_id:
                    stat_value /= 100
                _sub_stats = (stat_name, stat_value)
                sub_stats.append(_sub_stats)
            slot = equip_type_map[_flat["equipType"]]
            star = _flat["rankLevel"]
            artifacts.append(
                ArtifactInfo(
                    set_name=set_name,
                    id=artifact_id,
                    level=_level,
                    main_stat=_main_stat,
                    sub_stats=sub_stats,
                    slot=slot,
                    star=star,
                )
            )
        if _weapon:
            weapon_id = _equip["itemId"]
            weapon_name = weapon_name_map.get(weapon_id)
            if weapon_name is None:
                raise RuntimeError
            _level = _weapon["level"]
            refinement_level = next(iter(_weapon["affixMap"].values())) + 1
            _promote_level = _weapon["promoteLevel"]
            ascend = is_ascend(_level, _promote_level)
            weapon = WeaponInfo(level=_level, refine=refinement_level, ascend=ascend, name=weapon_name)
    return weapon, artifacts
