#!/usr/bin/env python3
"""Bump the workspace version in Cargo.toml and refresh Cargo.lock.

Usage:
    python scripts/bump_version.py <major|minor|patch|X.Y.Z>

The version lives in a single place (`[workspace.package].version` in the
root Cargo.toml) and every crate inherits it via `version.workspace = true`,
so bumping it there is sufficient.
"""
import re
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CARGO_TOML = ROOT / "Cargo.toml"
VERSION_RE = re.compile(r'^(version\s*=\s*")(\d+)\.(\d+)\.(\d+)(")', re.MULTILINE)

# Member crates whose Cargo.toml may have path-dependency entries pinning an
# exact version of another workspace crate (e.g. `version = "0.1.5", path =
# "../simconnect-proto"`) — these don't inherit from [workspace.package] and
# must be bumped separately.
MEMBER_CARGO_TOMLS = [
    ROOT / "simconnect-proto" / "Cargo.toml",
    ROOT / "simconnect-derive" / "Cargo.toml",
    ROOT / "simconnect" / "Cargo.toml",
    ROOT / "simconnect-cli" / "Cargo.toml",
]
PATH_DEP_VERSION_RE = re.compile(
    r'(\{\s*package\s*=\s*"flybywireless-[\w-]+"\s*,\s*version\s*=\s*")\d+\.\d+\.\d+(")'
)


def read_text(path):
    # newline="" preserves the file's existing line endings (this repo's
    # Cargo.toml files are CRLF) instead of normalizing to LF on write.
    with open(path, "r", newline="") as f:
        return f.read()


def write_text(path, text):
    with open(path, "w", newline="") as f:
        f.write(text)


def bump(major, minor, patch, part):
    if part == "major":
        return major + 1, 0, 0
    if part == "minor":
        return major, minor + 1, 0
    if part == "patch":
        return major, minor, patch + 1
    raise ValueError(part)


def main():
    if len(sys.argv) != 2:
        sys.exit(f"usage: {sys.argv[0]} <major|minor|patch|X.Y.Z>")
    arg = sys.argv[1]

    text = read_text(CARGO_TOML)
    m = VERSION_RE.search(text)
    if not m:
        sys.exit("could not find [workspace.package] version in Cargo.toml")

    major, minor, patch = int(m.group(2)), int(m.group(3)), int(m.group(4))

    if re.fullmatch(r"\d+\.\d+\.\d+", arg):
        new_major, new_minor, new_patch = (int(x) for x in arg.split("."))
    else:
        new_major, new_minor, new_patch = bump(major, minor, patch, arg)

    old_version = f"{major}.{minor}.{patch}"
    new_version = f"{new_major}.{new_minor}.{new_patch}"

    new_text = text[: m.start()] + m.group(1) + new_version + m.group(5) + text[m.end():]
    write_text(CARGO_TOML, new_text)
    print(f"Bumped workspace version: {old_version} -> {new_version}")

    for member_toml in MEMBER_CARGO_TOMLS:
        member_text = read_text(member_toml)
        new_member_text, count = PATH_DEP_VERSION_RE.subn(
            r"\g<1>" + new_version + r"\g<2>", member_text
        )
        if count:
            write_text(member_toml, new_member_text)
            print(f"Updated {count} path-dependency version pin(s) in {member_toml.relative_to(ROOT)}")

    result = subprocess.run(["cargo", "update", "--workspace"], cwd=ROOT)
    if result.returncode != 0:
        sys.exit("cargo update failed; Cargo.toml was still updated")


if __name__ == "__main__":
    main()
