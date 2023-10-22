import json
from pathlib import Path

import pytest
from genshin_artifact_core import (
    get_damage_analysis as _get_damage_analysis
)
from python_genshin_artifact.models.calculator import CalculatorConfig
from python_genshin_artifact.models.damage.analysis import DamageAnalysis, TransformativeDamage


TEST_DATA_DIR = Path(__file__).resolve().parent / 'input'


def get_damage_analysis(value: CalculatorConfig) -> DamageAnalysis:
    ret = _get_damage_analysis(value.json())
    return DamageAnalysis.parse_raw(ret)


def test_damage_analysis_exception():
    with open(TEST_DATA_DIR / 'invalid_enka_name.json') as f:
        config = CalculatorConfig(**json.loads(f.read()))
    with pytest.raises(RuntimeError, match='Failed to deserialize json'):
        get_damage_analysis(config)