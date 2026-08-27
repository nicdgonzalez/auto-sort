#!/usr/bin/env python3

"""
Scrapes the Spigot documentation to get a list of `Material` constants.
"""

import itertools
from typing import Iterator, cast

import bs4
import requests

URL = "https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/Material.html"


def main() -> None:
    response = requests.get(URL)
    assert not is_client_error(response), (
        f"invalid request to Spigot API: {response.status_code}"
    )

    if not is_success(response):
        raise RuntimeError(
            f"Spigot API returned an error status code: {response.status_code}"
        )

    constants = extract_constants_from_documentation(response.text)

    for constant in constants:
        print(constant)


def extract_constants_from_documentation(html: str) -> Iterator[str]:
    soup = bs4.BeautifulSoup(html, features="lxml")
    table = soup.select_one("section#enum-constant-summary div.summary-table")

    if table is None:
        raise RuntimeError("table 'Enum Constants' not found")

    rows = into_rows(iter(table.children))
    _ = next(rows)  # Skip table header row.

    return (
        constant.text
        for constant, description in rows
        if not is_skippable_row(constant.text, description.text)
    )


def is_success(response: requests.Response) -> bool:
    """Returns `True` if the status code is between 200 and 299."""
    assert isinstance(response.status_code, int)
    return 200 <= response.status_code <= 299


def is_client_error(response: requests.Response) -> bool:
    """Returns `True` if the status code is between 400 and 499."""
    assert isinstance(response.status_code, int)
    return 400 <= response.status_code <= 499


def into_rows(
    children: Iterator[bs4.PageElement],
) -> Iterator[tuple[bs4.PageElement, bs4.PageElement]]:
    """Groups a table's children into logical rows."""
    # For some reason, there is a blank element before each column.
    children = strip_extra_children(children)
    return cast(
        Iterator[tuple[bs4.PageElement, bs4.PageElement]],
        itertools.batched(children, 2),
    )


def strip_extra_children(
    children: Iterator[bs4.PageElement],
) -> Iterator[bs4.PageElement]:
    """Removes the blank entries inserted before each column."""
    return (
        element
        for index, element in enumerate(children)
        if not is_extra_child(index, element)
    )


def is_extra_child(index: int, element: bs4.PageElement) -> bool:
    """Checks if the given entry is extraneous."""
    # A blank element is added before each column, so we can alternate between
    # even and odd indices to determine whether it is intentionally blank,
    # or if it is one of the added elements.
    return is_even(index)


def is_even(n: int) -> bool:
    return n % 2 == 0


def is_skippable_row(constant: str, description: str) -> bool:
    """Returns `True` if the table row does not contain a sortable item."""
    return (
        any(
            (
                constant.startswith(s)
                for s in [
                    "LEGACY_",
                    "POTTED_",
                    "COMMAND_BLOCK",
                ]
            )
        )
        or any(
            (
                constant.endswith(s)
                for s in [
                    "_WALL_SKULL",
                    "_WALL_HEAD",
                    "_WALL_SIGN",
                    "_WALL_HANGING_SIGN",
                    "_DRIPLEAF_STEM",
                    "_CANDLE_CAKE",
                    "_WALL_BANNER",
                    "_WALL_FAN",
                    "COMMAND_BLOCK",
                    "WALL_TORCH",
                ]
            )
        )
        or constant
        in [
            "ATTACHED_MELON_STEM",
            "ATTACHED_PUMPKIN_STEM",
            "ENDER_DRAGON_SPAWN_EGG",
            "WITHER_SPAWN_EGG",
            "BAMBOO_SAPLING",
            "PETRIFIED_OAK_SLAB",
            "AIR",
            "CAVE_AIR",
            "VOID_AIR",
            "BARRIER",
            "BEETROOTS",
            "BUBBLE_COLUMN",
            "CANDLE_CAKE",
            "CARROTS",
            "CAVE_VINES",
            "CAVE_VINES_PLANT",
            "COCOA",
            "DEBUG_STICK",
            "END_GATEWAY",
            "END_PORTAL",
            "FILLED_MAP",
            "FIRE",
            "FROSTED_ICE",
            "JIGSAW",
            "KELP_PLANT",
            "KNOWLEDGE_BOOK",
            "LAVA",
            "LAVA_CAULDRON",
            "LIGHT",
            "MELON_STEM",
            "PUMPKIN_STEM",
            "MOVING_PISTON",
            "NETHER_PORTAL",
            "PISTON_HEAD",
            "PITCHER_CROP",
            "POTATOES",
            "POTENT_SULFUR",
            "POWDER_SNOW",
            "POWDER_SNOW_CAULDRON",
            "REDSTONE_WIRE",
            "SOUL_FIRE",
            "STRUCTURE_BLOCK",
            "STRUCTURE_VOID",
            "SWEET_BERRY_BUSH",
            "TALL_SEAGRASS",
            "TEST_BLOCK",
            "TEST_INSTANCE_BLOCK",
            "TORCHFLOWER_CROP",
            "TRIPWIRE",
            "TWISTING_VINES_PLANT",
            "WEEPING_VINES_PLANT",
            "WATER",
            "WATER_CAULDRON",
            "WRITTEN_BOOK",
        ]
    )


if __name__ == "__main__":
    main()
