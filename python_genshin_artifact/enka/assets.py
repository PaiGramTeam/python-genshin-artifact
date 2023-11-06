import json
import os
from typing import Dict, Optional

PATH = os.path.dirname(os.path.abspath(__file__))


class Assets:
    _instance: "Assets" = None
    character: Dict[str, dict] = {}

    def __new__(cls):
        if cls._instance is None:
            cls._instance = super().__new__(cls)
            cls._instance.reload_assets()
        return cls._instance

    def reload_assets(self) -> None:
        self.__load_assets_data()

    def __load_assets_data(self) -> None:
        path = os.path.join(PATH, "assets", "characters.json")
        with open(path, "r", encoding="utf-8") as file:
            self.character = json.load(file)

    def get_character(self, character_id: int) -> Optional[dict]:
        return self.character.get(str(character_id))
