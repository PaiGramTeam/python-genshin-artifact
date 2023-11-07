from typing import Dict, Set

fight_map: Dict[str, str] = {
    "FIGHT_PROP_ATTACK": "ATKFixed",
    "FIGHT_PROP_DEFENSE": "DEFFixed",
    "FIGHT_PROP_HP": "HPFixed",
    "FIGHT_PROP_ATTACK_PERCENT": "ATKPercentage",
    "FIGHT_PROP_DEFENSE_PERCENT": "DEFPercentage",
    "FIGHT_PROP_HP_PERCENT": "HPPercentage",
    "FIGHT_PROP_CRITICAL": "CriticalRate",
    "FIGHT_PROP_CRITICAL_HURT": "CriticalDamage",
    "FIGHT_PROP_CHARGE_EFFICIENCY": "Recharge",
    "FIGHT_PROP_HEAL_ADD": "HealingBonus",
    "FIGHT_PROP_ELEMENT_MASTERY": "ElementalMastery",
    "FIGHT_PROP_PHYSICAL_ADD_HURT": "PhysicalBonus",
    "FIGHT_PROP_FIRE_ADD_HURT": "PyroBonus",
    "FIGHT_PROP_ELEC_ADD_HURT": "ElectroBonus",
    "FIGHT_PROP_WATER_ADD_HURT": "HydroBonus",
    "FIGHT_PROP_WIND_ADD_HURT": "AnemoBonus",
    "FIGHT_PROP_ICE_ADD_HURT": "CryoBonus",
    "FIGHT_PROP_ROCK_ADD_HURT": "GeoBonus",
    "FIGHT_PROP_GRASS_ADD_HURT": "DendroBonus",
}

fixed: Set[str] = {
    "FIGHT_PROP_ATTACK",
    "FIGHT_PROP_DEFENSE",
    "FIGHT_PROP_HP",
    "FIGHT_PROP_ELEMENT_MASTERY",
}


def to_float(prop_id: str, pc: float) -> float:
    return pc if prop_id in fixed else (pc / 100)
