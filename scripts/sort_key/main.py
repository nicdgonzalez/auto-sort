#!/usr/bin/env python3

import argparse
import pathlib
from typing import cast

from constants import FAMILIES, VARIANTS
from family import Family
from sort_key import SortKey
from variant import Affix, Variant


def main() -> None:
    args = build_cli().parse_args()
    # `parse_args` returns a namespace with no type information, so we need
    # to `cast` if we want the Language Server to know what it's looking at.
    materials = cast(pathlib.Path, args.input).read_text().splitlines()

    # Move longer identifiers to the front so, for example, `STONE_BRICK`
    # matches before `STONE`, and `CHEST_BOAT` matches before `BOAT`.
    FAMILIES.sort(key=lambda f: len(f.identifier), reverse=True)
    VARIANTS.sort(key=lambda v: len(v.identifier), reverse=True)

    sort_keys: dict[str, SortKey] = {}

    for material in materials:
        for variant in VARIANTS:
            if (family := get_family(material, variant)) is not None:
                sort_keys[material] = SortKey(
                    family.category.value,
                    family.order,
                    variant.order,
                )
                break
            else:
                pass
        else:
            raise RuntimeError(f"family not found for {material!r}")


def get_family(material: str, variant: Variant) -> Family | None:
    family_id: str | None = None

    for family in FAMILIES:
        if material == family.identifier:
            family_id = material

    if family_id is None:
        if (family_id := get_family_id(material, variant)) is None:
            return None

    for family in FAMILIES:
        if family_id == family.identifier:
            return family

    return None


def get_family_id(material: str, variant: Variant) -> str | None:
    substring = variant.substring()

    match variant.affix:
        case Affix.PREFIX:
            if material.startswith(substring):
                start = len(substring)
                return material[start:]
        case Affix.SUFFIX:
            if material.endswith(substring):
                end = material.find(substring)
                assert end != -1
                return material[:end]

    return None


def build_cli() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser()

    parser.add_argument(
        "-i",
        "--input",
        type=pathlib.Path,
        help="Path to file containing a newline-terminated material list",
        required=True,
    )

    return parser


if __name__ == "__main__":
    main()
