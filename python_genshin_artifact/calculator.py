from genshin_artifact_core import (
    get_damage_analysis as _get_damage_analysis,
    get_transformative_damage as _get_transformative_damage,
)

from python_genshin_artifact.models.calculator import CalculatorConfig
from python_genshin_artifact.models.damage.analysis import DamageAnalysis, TransformativeDamage


def get_damage_analysis(value: CalculatorConfig) -> DamageAnalysis:
    ret = _get_damage_analysis(value.json())
    return DamageAnalysis.parse_raw(ret)


def get_transformative_damage(value: CalculatorConfig) -> TransformativeDamage:
    ret = _get_transformative_damage(value.json())
    return TransformativeDamage.parse_raw(ret)
