#!/usr/bin/env python3
"""Validate that a release dispatch targets the exact Cargo version tag."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

import tomllib

ROOT = Path(__file__).resolve().parents[1]


def cargo_version() -> str:
    """Read the authoritative package version from Cargo.toml."""
    with (ROOT / "Cargo.toml").open("rb") as manifest:
        return str(tomllib.load(manifest)["package"]["version"])


def validate_release_ref(
    ref_type: str, ref_name: str, ref_object_type: str, version: str
) -> str:
    """Return the expected tag or reject a non-identical release ref."""
    if ref_type != "tag":
        raise ValueError(f"release dispatch requires a tag ref, received {ref_type!r}")
    if ref_object_type != "tag":
        raise ValueError(
            "release dispatch requires an annotated Git tag object, "
            f"received {ref_object_type!r}"
        )

    expected = f"v{version}"
    if ref_name != expected:
        raise ValueError(
            f"release ref {ref_name!r} does not match Cargo version tag {expected!r}"
        )
    return expected


def self_test() -> None:
    """Exercise accepted stable/prerelease tags and every authority boundary."""
    assert validate_release_ref("tag", "v0.1.0", "tag", "0.1.0") == "v0.1.0"
    assert validate_release_ref("tag", "v2.0.0-rc.3", "tag", "2.0.0-rc.3") == (
        "v2.0.0-rc.3"
    )

    rejected = [
        ("branch", "master", "commit", "0.1.0"),
        ("tag", "v0.1.0", "commit", "0.1.0"),
        ("tag", "0.1.0", "tag", "0.1.0"),
        ("tag", "v0.1.1", "tag", "0.1.0"),
        ("tag", "v0.1.0-release.1", "tag", "0.1.0"),
    ]
    for ref_type, ref_name, ref_object_type, version in rejected:
        try:
            validate_release_ref(ref_type, ref_name, ref_object_type, version)
        except ValueError:
            continue
        raise AssertionError(
            "expected rejection for "
            f"{ref_type=}, {ref_name=}, {ref_object_type=}, {version=}"
        )


def main() -> int:
    """Run self-tests or validate the supplied Git ref against Cargo.toml."""
    parser = argparse.ArgumentParser()
    parser.add_argument("--ref-type")
    parser.add_argument("--ref-name")
    parser.add_argument("--ref-object-type")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()

    if args.self_test:
        self_test()
        print("release-ref authority self-test passed")
        return 0

    if not args.ref_type or not args.ref_name or not args.ref_object_type:
        parser.error(
            "--ref-type, --ref-name, and --ref-object-type are required "
            "outside --self-test"
        )

    try:
        expected = validate_release_ref(
            args.ref_type, args.ref_name, args.ref_object_type, cargo_version()
        )
    except ValueError as error:
        print(f"release-ref error: {error}", file=sys.stderr)
        return 1

    print(f"release ref agrees with {expected}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
