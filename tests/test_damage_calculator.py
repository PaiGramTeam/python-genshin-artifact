from python_genshin_artifact import (
    CalculatorConfig,
    get_damage_analysis,
    CharacterInterface,
    SkillInterface,
    WeaponInterface,
)


def test_damage_analysis():
    character = CharacterInterface(
        name="HuTao", level=90, ascend=False, constellation=6, skill1=12, skill2=12, skill3=12
    )
    skill = SkillInterface(index=1)
    weapon = WeaponInterface(name="StaffOfHoma", level=90, ascend=False, refine=4)
    calculator_config = CalculatorConfig(character=character, weapon=weapon, skill=skill)
    damage_analysis = get_damage_analysis(calculator_config)
    assert damage_analysis.is_heal is False
