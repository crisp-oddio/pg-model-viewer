#!/usr/bin/env python3
"""Set the app version across all version-bearing files.

Usage: python3 tools/bump_version.py <x.y.z>

Updates: package.json, package-lock.json, src-tauri/tauri.conf.json,
src-tauri/Cargo.toml, src-tauri/Cargo.lock. Used by the Release workflow.
"""
import json
import re
import sys


def main() -> None:
    v = sys.argv[1]
    if not re.fullmatch(r"\d+\.\d+\.\d+", v):
        sys.exit(f"invalid version: {v}")

    for path in ("package.json", "package-lock.json"):
        with open(path, encoding="utf-8") as f:
            data = json.load(f)
        data["version"] = v
        if path == "package-lock.json":
            data["packages"][""]["version"] = v
        with open(path, "w", encoding="utf-8", newline="\n") as f:
            json.dump(data, f, indent=2)
            f.write("\n")

    path = "src-tauri/tauri.conf.json"
    with open(path, encoding="utf-8") as f:
        conf = json.load(f)
    conf["version"] = v
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        json.dump(conf, f, indent=2)
        f.write("\n")

    path = "src-tauri/Cargo.toml"
    with open(path, encoding="utf-8") as f:
        s = f.read()
    s = re.sub(r'^version = "[^"]+"', f'version = "{v}"', s, count=1, flags=re.M)
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(s)

    path = "src-tauri/Cargo.lock"
    with open(path, encoding="utf-8") as f:
        s = f.read()
    s = re.sub(
        r'(name = "pg-model-viewer"\nversion = )"[^"]+"', rf'\1"{v}"', s, count=1
    )
    with open(path, "w", encoding="utf-8", newline="\n") as f:
        f.write(s)

    print(f"version set to {v}")


if __name__ == "__main__":
    main()
