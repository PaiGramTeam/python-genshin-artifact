from json import JSONDecodeError

from genshin_artifact_core import (
    get_damage_analysis as _get_damage_analysis,
    get_transformative_damage as _get_transformative_damage,
)

from python_genshin_artifact.error import JsonParseException
from python_genshin_artifact.models.calculator import CalculatorConfig
from python_genshin_artifact.models.damage.analysis import (
    DamageAnalysis,
    TransformativeDamage,
)


def get_damage_analysis(value: CalculatorConfig) -> DamageAnalysis:
    try:
        ret = _get_damage_analysis(value.json())
    except JSONDecodeError as exc:
        raise JsonParseException from exc
    return DamageAnalysis.parse_raw(ret)


def get_transformative_damage(value: CalculatorConfig) -> TransformativeDamage:
    try:
        ret = _get_transformative_damage(value.json())
    except JSONDecodeError as exc:
        raise JsonParseException from exc
    return TransformativeDamage.parse_raw(ret)
