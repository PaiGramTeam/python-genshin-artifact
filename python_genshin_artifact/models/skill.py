

from pydantic import BaseModel


class SkillInfo(BaseModel):
    index: str
    config: str = "NoConfig"
