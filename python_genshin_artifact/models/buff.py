from pydantic import BaseModel


class BuffInfo(BaseModel):
    name: str
    config: str
    star: int
