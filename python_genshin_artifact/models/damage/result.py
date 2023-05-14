from pydantic import BaseModel


class DamageResult(BaseModel):
    critical: float
    non_critical: float
    expectation: float

    is_heal: bool
    is_shield: bool
