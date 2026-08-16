from typing import NamedTuple

from category import Category


class Family(NamedTuple):
    category: Category
    identifier: str
    order: int
