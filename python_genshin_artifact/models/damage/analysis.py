from typing import Dict, Optional

from pydantic import BaseModel, Field

from python_genshin_artifact.models.damage.result import DamageResult


class DamageAnalysis(BaseModel):
    atk: Dict[str, float]
    atk_ratio: Dict[str, float]
    hp: Dict[str, float]
    hp_ratio: Dict[str, float]
    def_: Dict[str, float] = Field(alias="def")
    def_ratio: Dict[str, float]
    em: Dict[str, float]
    em_ratio: Dict[str, float]
    extra_damage: Dict[str, float]
    bonus: Dict[str, float]
    critical: Dict[str, float]
    critical_damage: Dict[str, float]
    melt_enhance: Dict[str, float]
    vaporize_enhance: Dict[str, float]
    healing_bonus: Dict[str, float]
    shield_strength: Dict[str, float]
    spread_compose: Dict[str, float]
    aggravate_compose: Dict[str, float]

    def_minus: Dict[str, float]
    def_penetration: Dict[str, float]
    res_minus: Dict[str, float]

    element: str
    is_heal: bool
    is_shield: bool

    normal: DamageResult
    melt: Optional[DamageResult]
    vaporize: Optional[DamageResult]
    spread: Optional[DamageResult]
    aggravate: Optional[DamageResult]


class TransformativeDamage(BaseModel):
    swirl_cryo: float
    swirl_hydro: float
    swirl_pyro: float
    swirl_electro: float
    overload: float
    electro_charged: float
    shatter: float
    super_conduct: float = Field(alias="superconduct")
    bloom: float
    hyper_bloom: float = Field(alias="hyperbloom")
    burgeon: float
    burning: float
    crystallize: float
