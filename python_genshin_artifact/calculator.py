from genshin_artifact_function import get_damage_analysis as _get_damage_analysis

from python_genshin_artifact.models.characterInfo import CharacterInfo
from python_genshin_artifact.models.damage.analysis import DamageAnalysis


def get_damage_analysis(value: CharacterInfo) -> DamageAnalysis:
    ret = _get_damage_analysis(value=value.json())
    return DamageAnalysis.parse_raw(ret)
