import enum


class Category(enum.Enum):
    BUILDING_BLOCKS = 0
    COLORED_BLOCKS = enum.auto()
    NATURAL_BLOCKS = enum.auto()
    FUNCTIONAL_BLOCKS = enum.auto()
    REDSTONE_BLOCKS = enum.auto()
    TOOLS_AND_UTILITIES = enum.auto()
    COMBAT = enum.auto()
    FOOD_AND_DRINKS = enum.auto()
    INGREDIENTS = enum.auto()
    SPAWN_EGGS = enum.auto()
    OTHER = enum.auto()
    UNSORTED = enum.auto()
