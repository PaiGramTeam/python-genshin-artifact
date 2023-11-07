import sys
from typing import List, Optional, Tuple, TYPE_CHECKING, Dict, final

if sys.version_info < (3, 11):
    from typing_extensions import Literal
else:
    from typing import Literal

if TYPE_CHECKING:
    StatName = Literal[
        "ATKFixed",
        "ATKPercentage",
        "HealingBonus",
        "HPFixed",
        "HPPercentage",
        "DEFFixed",
        "DEFPercentage",
        "CriticalRate",
        "CriticalDamage",
        "ElementalMastery",
        "Recharge",
        "ElectroBonus",
        "PyroBonus",
        "HydroBonus",
        "CryoBonus",
        "AnemoBonus",
        "GeoBonus",
        "DendroBonus",
        "PhysicalBonus",
    ]
else:
    StatName = str

def get_damage_analysis(calculator_config: "CalculatorConfig") -> "DamageAnalysis": ...
def get_transformative_damage(calculator_config: "CalculatorConfig") -> "TransformativeDamage": ...
def gen_character_meta_as_json() -> str: ...
def gen_weapon_meta_as_json() -> str: ...
def gen_artifact_meta_as_json() -> str: ...
def gen_generate_locale_as_json(loc: str) -> str: ...
@final
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

    def __new__(
        cls,
        swirl_cryo: float,
        swirl_hydro: float,
        swirl_pyro: float,
        swirl_electro: float,
        overload: float,
        electro_charged: float,
        shatter: float,
        super_conduct: float,
        bloom: float,
        hyper_bloom: float,
        burgeon: float,
        burning: float,
        crystallize: float,
    ) -> "TransformativeDamage": ...

@final
class DamageResult:
    critical: float
    non_critical: float
    expectation: float
    is_heal: bool
    is_shield: bool

    def __new__(
        cls, critical: float, non_critical: float, expectation: float, is_heal: bool, is_shield: bool
    ) -> "DamageResult": ...

@final
class DamageAnalysis:
    atk: Dict[str, float]
    atk_ratio: Dict[str, float]
    hp: Dict[str, float]
    hp_ratio: Dict[str, float]
    defense: Dict[str, float]
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

@final
class CharacterInterface:
    name: str
    level: int
    ascend: bool
    constellation: int
    skill1: int
    skill2: int
    skill3: int
    params: Optional[dict] = None

    def __new__(
        cls,
        name: str,
        level: int,
        ascend: bool,
        constellation: int,
        skill1: int,
        skill2: int,
        skill3: int,
        params: Optional[dict] = None,
    ) -> "CharacterInterface": ...

@final
class WeaponInterface:
    name: str
    level: int
    ascend: bool
    refine: int
    params: Optional[dict] = None

    def __new__(
        cls, name: str, level: int, ascend: bool, refine: int, params: Optional[dict] = None
    ) -> "WeaponInterface": ...

@final
class BuffInterface:
    name: str
    config: Optional[dict] = None

    def __new__(cls, name: str, config: Optional[dict] = None) -> "BuffInterface": ...

@final
class Artifact:
    set_name: str
    slot: str
    level: int
    star: int
    sub_stats: List[Tuple[StatName, float]]
    main_stat: Tuple[StatName, float]
    id: int

    def __new__(
        cls,
        set_name: str,
        slot: str,
        level: int,
        star: int,
        sub_stats: List[Tuple[StatName, float]],
        main_stat: Tuple[StatName, float],
        id: int,
    ) -> "Artifact": ...

@final
class SkillInterface:
    index: int
    config: Optional[dict] = None

    def __new__(cls, index: int, config: Optional[dict] = None) -> "SkillInterface": ...

@final
class EnemyInterface:
    level: int
    electro_res: float
    pyro_res: float
    hydro_res: float
    cryo_res: float
    geo_res: float
    anemo_res: float
    dendro_res: float
    physical_res: float

    def __new__(
        cls,
        level: int,
        electro_res: float,
        pyro_res: float,
        hydro_res: float,
        cryo_res: float,
        geo_res: float,
        anemo_res: float,
        dendro_res: float,
        physical_res: float,
    ) -> "EnemyInterface": ...

@final
class CalculatorConfig:
    character: CharacterInterface
    weapon: WeaponInterface
    buffs: List[BuffInterface] = []
    artifacts: List[Artifact] = []
    artifact_config: Optional[dict] = None
    skill: SkillInterface
    enemy: Optional[EnemyInterface] = None

    def __new__(
        cls,
        character: CharacterInterface,
        weapon: WeaponInterface,
        skill: SkillInterface,
        buffs: Optional[List[BuffInterface]] = None,
        artifacts: Optional[List[Artifact]] = None,
        artifact_config: Optional[dict] = None,
        enemy: Optional[EnemyInterface] = None,
    ) -> "CalculatorConfig": ...
