#!/usr/bin/env python3

"""
Scrapes the Spigot documentation to get a list of `Material` constants.

## Requirements

Create a virtual environment and install the required dependencies:

```bash
python3 -m venv .venv
bash ./.venv/bin/activate
pip install requests bs4
```

## Usage

This script outputs a new-line-terminated list of Material` constants
to stdout. To save it to a file, redirect the stream to the target path:

```bash
./scripts/materials.py > ./materials.txt
```
"""

import itertools
from typing import Iterator

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
    return 200 <= response.status_code <= 299


def is_client_error(response: requests.Response) -> bool:
    """Returns `True` if the status code is between 400 and 499."""
    return 400 <= response.status_code <= 499


def into_rows(
    children: Iterator[bs4.PageElement],
) -> Iterator[tuple[bs4.PageElement, bs4.PageElement]]:
    """Groups a table's children into logical rows."""
    # For some reason, there is a blank element before each column.
    children = strip_extra_children(children)
    return itertools.batched(children, 2)


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
    """Returns `True` if a table row is not required for our project."""
    # The `AIR` material is filtered out in our plugin code to align items
    # to the top left corner of an inventory. We could leave it in, but
    # we would have to create an extra category for it at the end so it always
    # sorts to the back. It is easier to just filter it out on both ends.
    return (
        constant.startswith("LEGACY_")
        or constant.startswith("POTTED_")
        or constant.endswith("_WALL_SKULL")
        or constant.endswith("_WALL_HEAD")
        or constant == "AIR"
    )


if __name__ == "__main__":
    main()
