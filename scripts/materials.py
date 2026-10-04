#!/usr/bin/env python3

"""
Scrapes the Spigot documentation to get a list of `Material` constants.
"""

import argparse
import itertools
import pathlib
from typing import Iterator, cast

import bs4
import requests

URL = "https://hub.spigotmc.org/javadocs/bukkit/org/bukkit/Material.html"


def main() -> None:
    args = build_cli().parse_args()
    skipped_only = cast(bool, args.skipped_only)

    response = requests.get(URL)
    assert not is_client_error(response), (
        f"invalid request to Spigot API: {response.status_code}"
    )

    if not is_success(response):
        raise RuntimeError(
            f"Spigot API returned an error status code: {response.status_code}"
        )

    # Extract constants from documentation.
    html = response.text

    soup = bs4.BeautifulSoup(html, features="lxml")
    table = soup.select_one("section#enum-constant-summary div.summary-table")

    if table is None:
        raise RuntimeError("table 'Enum Constants' not found")

    rows = into_rows(iter(table.children))
    _ = next(rows)  # Skip table header row.

    constants = (
        constant.text
        for constant, description in rows
        if not is_skippable_row(constant.text, description.text)
        and not skipped_only
    )

    for constant in constants:
        print(constant)


def build_cli() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "--skipped-only",
        action="store_true",
        help="Displays only the skipped items (for appending to the `excludes.txt` file).",
    )

    return parser


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
    children: Iterator[bs4.PageElement], /
) -> Iterator[bs4.PageElement]:
    """Removes the blank entries inserted before each column."""
    return (
        child
        for index, child in enumerate(children)
        if not is_extra_child(index, child)
    )


def is_extra_child(index: int, _element: bs4.PageElement, /) -> bool:
    # A blank element is added before each column, so we can alternate between
    # even and odd indices to determine whether it is intentionally blank,
    # or if it is one of the added elements.
    return is_even(index)


def is_even(n: int) -> bool:
    return n % 2 == 0


def is_skippable_row(constant: str, _description: str, /) -> bool:
    """Returns `True` if the item cannot be held in game."""
    skippable_items = get_skippable_items()
    return constant in skippable_items


def get_skippable_items() -> list[str]:
    current_dir = pathlib.Path(__file__).parent
    file = current_dir.joinpath("exclude.txt")

    with open(file, "r") as f:
        return [line.strip() for line in f.readlines()]


if __name__ == "__main__":
    main()
