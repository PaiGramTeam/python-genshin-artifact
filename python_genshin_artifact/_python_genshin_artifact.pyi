from typing import List, Optional, Tuple, Literal, TYPE_CHECKING

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

class WeaponInterface:
    name: str
    level: int
    ascend: bool
    refine: int
    params: Optional[dict] = None

    def __new__(
        cls, name: str, level: int, ascend: bool, refine: int, params: Optional[dict] = None
    ) -> "WeaponInterface": ...

class BuffInterface:
    name: str
    config: Optional[dict] = None
    def __new__(cls, name: str, config: Optional[dict] = None) -> "BuffInterface": ...

class Artifact:
    set_name: str
    slot: str
    level: int
    star: int
    sub_stats: List[Tuple["StatName", float]]
    main_stat: Tuple["StatName", float]
    id: int

    def __new__(
        cls,
        set_name: str,
        slot: str,
        level: int,
        star: int,
        sub_stats: List[Tuple["StatName", float]],
        main_stat: Tuple["StatName", float],
        id: int,
    ) -> "Artifact": ...

class SkillInterface:
    index: int
    config: Optional[dict] = None
    def __new__(cls, index: int, config: Optional[dict] = None) -> "SkillInterface": ...

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
