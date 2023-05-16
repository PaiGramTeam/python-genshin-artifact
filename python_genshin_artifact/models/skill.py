from typing import Union
from pydantic import BaseModel


class SkillInfo(BaseModel):
    index: int
    config: Union[str, dict] = "NoConfig"
