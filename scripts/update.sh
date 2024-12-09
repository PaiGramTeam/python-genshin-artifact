#!/bin/bash

wget -O characters.json https://raw.githubusercontent.com/EnkaNetwork/API-docs/refs/heads/master/store/characters.json
mv -f characters.json python_genshin_artifact/enka/assets/characters.json
