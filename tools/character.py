from typing import List

from python_genshin_artifact.assets import Assets

assets = Assets()

character = assets.character.get("Yelan")
locale = assets.locale.get("zh-cn")

print(f"name {locale[character.get('name_locale')]}")

for skill_map1 in character.get("skill_map1"):
    index = skill_map1.get("index")
    locale_index = skill_map1.get("locale_index")
    print(f"skill_map1 index {index} locale {locale[locale_index]}")


for skill_map2 in character.get("skill_map2"):
    index = skill_map2.get("index")
    locale_index = skill_map2.get("locale_index")
    print(f"skill_map2 index {index} locale {locale[locale_index]}")

for skill_map3 in character.get("skill_map3"):
    index = skill_map3.get("index")
    locale_index = skill_map3.get("locale_index")
    print(f"skill_map3 index {index} locale {locale[locale_index]}")

print(locale.index("夜兰"))


