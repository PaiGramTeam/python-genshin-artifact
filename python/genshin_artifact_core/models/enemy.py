from pydantic import BaseModel


class EnemyInfo(BaseModel):
    level: int
    electro_res: float
    pyro_res: float
    hydro_res: float
    cryo_res: float
    geo_res: float
    anemo_res: float
    dendro_res: float
    physical_res: float
