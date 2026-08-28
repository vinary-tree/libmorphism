#!/usr/bin/env python3
"""Fail closed when the formal-to-Rust correspondence matrix becomes stale."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
MATRIX = ROOT / "formal" / "refinement.tsv"
IDENTIFIER = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")


def contains_identifier(path: Path, identifier: str) -> bool:
    text = path.read_text(encoding="utf-8")
    return re.search(rf"\b{re.escape(identifier)}\b", text) is not None


def checked_path(relative: str) -> Path:
    path = (ROOT / relative).resolve()
    if ROOT not in path.parents:
        raise ValueError(f"matrix path escapes repository: {relative}")
    if not path.is_file():
        raise ValueError(f"matrix path is not a file: {relative}")
    return path


def main() -> int:
    failures: list[str] = []
    seen: set[str] = set()
    rows = 0

    for line_number, raw_line in enumerate(
        MATRIX.read_text(encoding="utf-8").splitlines(), start=1
    ):
        if not raw_line or raw_line.startswith("#"):
            continue
        columns = raw_line.split("\t")
        if len(columns) != 5:
            failures.append(f"line {line_number}: expected 5 tab-separated columns")
            continue
        row_id, formal_source, formal_claim, rust_reference, test_reference = columns
        rows += 1
        if row_id in seen:
            failures.append(f"line {line_number}: duplicate id {row_id}")
        seen.add(row_id)

        references = (
            ("formal", formal_source, formal_claim),
            ("Rust", *rust_reference.split("::", maxsplit=1)),
            ("test", *test_reference.split("::", maxsplit=1)),
        )
        for kind, relative, symbol_path in references:
            try:
                path = checked_path(relative)
            except ValueError as error:
                failures.append(f"line {line_number}: {error}")
                continue
            symbols = symbol_path.split("::")
            for symbol in symbols:
                if not IDENTIFIER.fullmatch(symbol):
                    failures.append(
                        f"line {line_number}: invalid {kind} symbol {symbol!r}"
                    )
                elif not contains_identifier(path, symbol):
                    failures.append(
                        f"line {line_number}: {kind} symbol {symbol} missing from {relative}"
                    )

    if rows == 0:
        failures.append("correspondence matrix has no data rows")
    if failures:
        for failure in failures:
            print(f"refinement error: {failure}", file=sys.stderr)
        return 1
    print(f"verified {rows} formal-to-Rust correspondence rows")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
