import enum
from typing import NamedTuple


class Affix(enum.Enum):
    PREFIX = enum.auto()
    SUFFIX = enum.auto()


class Variant(NamedTuple):
    identifier: str
    affix: Affix
    order: int

    def substring(self) -> str:
        match self.affix:
            case Affix.PREFIX:
                return self.identifier + "_"
            case Affix.SUFFIX:
                return "_" + self.identifier
