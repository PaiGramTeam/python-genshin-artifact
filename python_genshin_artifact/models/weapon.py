from pydantic import BaseModel


class WeaponInfo(BaseModel):
    name: str
    level: int
    ascend: bool
    refine: int
    params: str
