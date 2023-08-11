from genshin_artifact_core import get_damage_analysis as _get_damage_analysis

from python_genshin_artifact.models.calculator import CalculatorConfig
from python_genshin_artifact.models.damage.analysis import DamageAnalysis


def get_damage_analysis(value: CalculatorConfig) -> DamageAnalysis:
    ret = _get_damage_analysis(value.json())
    return DamageAnalysis.parse_raw(ret)
