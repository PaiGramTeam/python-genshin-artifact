class GenshinArtifactException(Exception):
    """GenshinArtifactException"""


class JsonParseException(GenshinArtifactException):
    """JSON 解析错误"""


class EnkaParseException(GenshinArtifactException):
    """EnkaParse 解析错误"""
