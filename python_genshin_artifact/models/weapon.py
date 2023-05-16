from typing import Union

from pydantic import BaseModel


class WeaponInfo(BaseModel):
    name: str
    level: int
    ascend: bool
    refine: int
    params: Union[str, dict] = "NoConfig"
