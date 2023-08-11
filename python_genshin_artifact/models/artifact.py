from typing import List, Tuple

from pydantic import BaseModel


class ArtifactInfo(BaseModel):
    set_name: str
    slot: str
    level: int
    star: int
    sub_stats: List[Tuple[str, float]]
    main_stat: Tuple[str, float]
    id: int
