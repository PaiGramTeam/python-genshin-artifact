from typing import List, Optional

from pydantic import BaseModel

from python_genshin_artifact.models.artifact import ArtifactInfo
from python_genshin_artifact.models.buff import BuffInfo
from python_genshin_artifact.models.characterInfo import CharacterInfo
from python_genshin_artifact.models.enemy import EnemyInfo
from python_genshin_artifact.models.skill import SkillInfo
from python_genshin_artifact.models.weapon import WeaponInfo


class CalculatorConfig(BaseModel):
    character: CharacterInfo
    weapon: WeaponInfo
    buffs: List[BuffInfo] = []
    artifacts: List[ArtifactInfo] = []
    artifact_config: Optional[dict] = None
    skill: SkillInfo
    enemy: Optional[EnemyInfo] = None
