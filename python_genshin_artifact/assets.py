import json
from typing import Dict, Tuple

from genshin_artifact_function import (
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
    locale: Dict[str, dict] = {}
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
        self.character = json.loads(character_meta)
        weapon_meta = gen_weapon_meta_as_json()
        self.weapon = json.loads(weapon_meta)
        artifact_meta = gen_artifact_meta_as_json()
        self.artifact = json.loads(artifact_meta)
        for loc in self.langs:
            locale = gen_generate_locale_as_json(loc)
            self.locale[loc] = json.loads(locale)
