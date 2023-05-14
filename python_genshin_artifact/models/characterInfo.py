from pydantic import BaseModel


class CharacterInfo(BaseModel):
    name: str
    level: int
    ascend: bool
    constellation: int
    skill1: int
    skill2: int
    skill3: int
    params: str = "NoConfig"
