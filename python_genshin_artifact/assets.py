import json
from typing import Dict, Tuple, List

from python_genshin_artifact import (
    gen_character_meta_as_json,
    gen_weapon_meta_as_json,
    gen_artifact_meta_as_json,
    gen_generate_locale_as_json,
)


class Assets:
    _instance: "Assets" = None
    character: Dict[str, dict] = {}
    weapon: Dict[str, dict] = {}
    artifact: Dict[str, dict] = {}
    locale: Dict[str, List[str]] = {}
    langs: Tuple[str] = ("zh-cn", "en")

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            cls._instance.reload_assets()
        return cls._instance

    def reload_assets(self) -> None:
        self.__load_assets_data()

    def __load_assets_data(self) -> None:
        character_meta = gen_character_meta_as_json()
        _characters = json.loads(character_meta)
        for _character in _characters:
            name = _character.get("name")
            if name is not None:
                self.character.setdefault(name, _character)
        weapon_meta = gen_weapon_meta_as_json()
        _weapons = json.loads(weapon_meta)
        for _weapon in _weapons:
            name = _weapon.get("name")
            if name is not None:
                self.weapon.setdefault(name, _weapon)
        artifact_meta = gen_artifact_meta_as_json()
        _artifacts = json.loads(artifact_meta)
        for _artifact in _artifacts:
            name = _artifact.get("name")
            if name is not None:
                self.artifact.setdefault(name, _artifact)
        for loc in self.langs:
            locale = gen_generate_locale_as_json(loc)
            self.locale.setdefault(loc, json.loads(locale))
