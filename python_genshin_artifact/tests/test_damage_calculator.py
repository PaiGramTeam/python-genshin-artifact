import json
from json import JSONDecodeError
from pathlib import Path

import pytest
from python_genshin_artifact.calculator import get_damage_analysis
from python_genshin_artifact.error import JsonParseException
from python_genshin_artifact.models.calculator import CalculatorConfig


TEST_DATA_DIR = Path(__file__).resolve().parent / "input"


def test_damage_analysis_exception():
    """Test damage analysis raises expected exception on invalid input"""
    with open(TEST_DATA_DIR / "invalid_enka_name.json") as f:
        config = CalculatorConfig(**json.load(f))
    with pytest.raises(JsonParseException) as exc_info:
        get_damage_analysis(config)
        assert type(exc_info.__cause__) is JSONDecodeError
