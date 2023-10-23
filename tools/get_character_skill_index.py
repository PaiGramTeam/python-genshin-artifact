from python_genshin_artifact.assets import Assets

assets = Assets()


def main():
    name = input("Enter character name: ").strip()
    character = assets.character.get(name)

    if character is None:
        print(f"Character {name} not found")
        print("Available characters:")
        print(", ".join(assets.character.keys()))
        return

    locale = assets.locale.get("zh-cn")

    print(f"Character name {locale[character.get('name_locale')]}")

    skill1_name_index = character.get("skill1_name_index")
    print(f"Skill A {locale[skill1_name_index]}")

    for skill_map1 in character.get("skill_map1"):
        index = skill_map1.get("index")
        locale_index = skill_map1.get("locale_index")
        print(f"index {index} {locale[locale_index]}")

    skill2_name_index = character.get("skill2_name_index")
    print(f"Skill E {locale[skill2_name_index]}")

    for skill_map2 in character.get("skill_map2"):
        index = skill_map2.get("index")
        locale_index = skill_map2.get("locale_index")
        print(f"index {index} {locale[locale_index]}")

    skill3_name_index = character.get("skill3_name_index")
    print(f"Skill Q {locale[skill3_name_index]}")

    for skill_map3 in character.get("skill_map3"):
        index = skill_map3.get("index")
        locale_index = skill_map3.get("locale_index")
        print(f"index {index} {locale[locale_index]}")


if __name__ == "__main__":
    main()
