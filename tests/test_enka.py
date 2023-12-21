import json
import logging
import os

from python_genshin_artifact import get_damage_analysis, get_transformative_damage
from python_genshin_artifact.enka.enka_parser import enka_parser
from python_genshin_artifact import CalculatorConfig
from python_genshin_artifact import SkillInterface

PATH = os.path.dirname(os.path.abspath(__file__))

logger = logging.getLogger(__name__)
enka_test_file = os.path.join(PATH, "data", "enka.json")
with open(enka_test_file, "r", encoding="utf-8") as file:
    enka_data = json.load(file)


def test_enka_parser():
    character, weapon, artifacts = enka_parser(enka_data, 10000046)
    logger.info(character)
    logger.info(weapon)
    logger.info(artifacts)
    s = {"index": 7, "config": {"HuTao": {"after_e": True}}}
    artifact_config = {"config_crimson_witch_of_flames": {"level": 1}}
    skill = SkillInterface(**s)
    calculator_config = CalculatorConfig(
        character=character, weapon=weapon, artifacts=artifacts, skill=skill, artifact_config=artifact_config
    )
    damage_analysis = get_damage_analysis(calculator_config)
    logger.info(damage_analysis)
    transformative_damage = get_transformative_damage(calculator_config)
    logger.info(transformative_damage)
