from typing import List, Optional

def get_damage_analysis(value_str: str) -> str: ...
def get_transformative_damage(value_str: str) -> str: ...
def gen_character_meta_as_json() -> str: ...
def gen_weapon_meta_as_json() -> str: ...
def gen_artifact_meta_as_json() -> str: ...
def gen_generate_locale_as_json(loc: str) -> str: ...

class TransformativeDamage:
    swirl_cryo: float
    swirl_hydro: float
    swirl_pyro: float
    swirl_electro: float
    overload: float
    electro_charged: float
    shatter: float
    super_conduct: float
    bloom: float
    hyper_bloom: float
    burgeon: float
    burning: float
    crystallize: float

class CharacterInterface:
    name: str
    level: int
    ascend: bool
    constellation: int
    skill1: int
    skill2: int
    skill3: int
    params: Optional[dict] = None

class WeaponInterface:
    name: str
    level: int
    ascend: bool
    refine: int
    params: Optional[dict] = None

class BuffInterface:
    name: str
    config: str

class CalculatorConfig:
    character: CharacterInterface
    weapon: WeaponInterface
    buffs: List[BuffInterface] = []
