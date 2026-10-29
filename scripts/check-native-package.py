#!/usr/bin/env python3
"""Validate a packaged native Hachures archive or extracted directory."""

import sys
import tarfile
import zipfile
from pathlib import PurePosixPath


REQUIRED_SUFFIXES = (
    "assets/CREDITS.md",
    "assets/fonts/IMFellEnglish-Regular.ttf",
    "assets/models/tower-round-bottom-a.glb",
    "assets/shaders/hachure.wesl",
    "assets/ui/icon-cible.png",
)


def fail(message: str) -> int:
    print(f"native package check failed: {message}", file=sys.stderr)
    return 1


def archive_names(path: str) -> set[str]:
    if path.endswith(".zip"):
        with zipfile.ZipFile(path) as archive:
            return {PurePosixPath(name).as_posix().rstrip("/") for name in archive.namelist()}
    if path.endswith(".tar.gz"):
        with tarfile.open(path, "r:gz") as archive:
            return {PurePosixPath(member.name).as_posix().rstrip("/") for member in archive.getmembers()}
    return set()


def directory_names(path: str) -> set[str]:
    from pathlib import Path

    root = Path(path)
    return {item.relative_to(root).as_posix() for item in root.rglob("*") if item.is_file()}


def main() -> int:
    if len(sys.argv) != 2:
        return fail("usage: check-native-package.py ARCHIVE_OR_DIRECTORY")
    path = sys.argv[1]
    names = archive_names(path) if path.endswith((".zip", ".tar.gz")) else directory_names(path)
    if not names:
        return fail(f"package is empty or unsupported: {path}")

    if path.endswith(".zip") or path.endswith(".tar.gz"):
        root = next(iter(names)).split("/", 1)[0]
        names = {name.removeprefix(root + "/") for name in names}

    executable = "Hachures.exe" if path.endswith(".zip") or "windows" in path.lower() else "Hachures"
    required = (executable, *REQUIRED_SUFFIXES)
    missing = [name for name in required if name not in names]
    if missing:
        return fail(f"missing required entries: {', '.join(missing)}")

    print(f"native package check passed: {path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
