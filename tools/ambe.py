import httpx

from python_genshin_artifact.enka.characters import characters_map
from python_genshin_artifact.enka.weapon import weapon_name_map, test_weapon_name_map
from python_genshin_artifact.enka.artifacts import artifacts_name_map

response = httpx.get("https://api.ambr.top/v2/chs/weapon")

items: dict = response.json()["data"]["items"]
need_add = {}

for key, value in items.items():
    key_id = int(key)
    if key_id in weapon_name_map:
        continue
    if key_id in test_weapon_name_map:
        continue
    name = value["route"].replace(" ", "")
    need_add.setdefault(int(key), name)

print(need_add)

response = httpx.get("https://api.ambr.top/v2/chs/avatar")

items: dict = response.json()["data"]["items"]
need_add = {}

for key, value in items.items():
    try:
        key_id = int(key)
    except ValueError as exc:
        print(exc)
        continue
    if key_id in characters_map:
        continue
    name = value["route"].replace(" ", "")
    need_add.setdefault(int(key), name)

print(need_add)


response = httpx.get("https://api.ambr.top/v2/chs/reliquary")

items: dict = response.json()["data"]["items"]
need_add = {}

for key, value in items.items():
    key_id = int(key)
    if key_id in artifacts_name_map:
        continue
    name = value["route"].replace(" ", "")
    need_add.setdefault(int(key), name)

print(need_add)
