from python_genshin_artifact import CharacterInterface


def test_character_interface():
    params = {"HuTao": {"le_50": True}}
    character = CharacterInterface(
        name="HuTao", level=90, ascend=False, constellation=6, skill1=12, skill2=12, skill3=12, params=params
    )
    assert character.name == "HuTao"
    assert character.level == 90
    assert character.ascend is False
    assert character.constellation == 6
    assert character.skill1 == 12
    assert character.skill2 == 12
    assert character.skill3 == 12
    assert character.params == params
