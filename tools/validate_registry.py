#!/usr/bin/env python3
"""Validate data/registry.json against data/registry.schema.json plus
structural rules the schema can't express, then print a summary.

Usage: python3 tools/validate_registry.py [path/to/registry.json]
Exits non-zero on any problem — suitable for CI.

No third-party deps: implements the small subset of JSON Schema this
repo actually uses (type/required/enum/pattern/items/properties).
"""
import json
import re
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def check(instance, schema, path="$"):
    """Tiny JSON-Schema-subset checker; yields problem strings."""
    t = schema.get("type")
    if t:
        py = {"object": dict, "array": list, "string": str, "integer": int, "number": (int, float)}[t]
        if not isinstance(instance, py) or (t == "integer" and isinstance(instance, bool)):
            yield f"{path}: expected {t}, got {type(instance).__name__}"
            return
    if "enum" in schema and instance not in schema["enum"]:
        yield f"{path}: {instance!r} not in {schema['enum']}"
    if "pattern" in schema and isinstance(instance, str) and not re.fullmatch(schema["pattern"], instance):
        yield f"{path}: {instance!r} fails pattern {schema['pattern']}"
    if t == "object":
        for req in schema.get("required", []):
            if req not in instance:
                yield f"{path}: missing required key '{req}'"
        for key, sub in schema.get("properties", {}).items():
            if key in instance:
                yield from check(instance[key], sub, f"{path}.{key}")
    if t == "array" and "items" in schema:
        for i, item in enumerate(instance):
            yield from check(item, schema["items"], f"{path}[{i}]")


def structural(reg):
    ids = [e["id"] for e in reg["entries"]]
    for dup in [i for i, c in Counter(ids).items() if c > 1]:
        yield f"duplicate entry id: {dup}"
    for e in reg["entries"]:
        for tag in e["tags"]:
            if tag not in reg["tags"]:
                yield f"entry {e['id']} uses tag '{tag}' missing from taxonomy"
    hs = sum(1 for e in reg["entries"] if e["type"] == "handshake")
    if hs != reg["total_handshakes"]:
        yield f"total_handshakes says {reg['total_handshakes']} but found {hs}"


def main():
    reg_path = Path(sys.argv[1]) if len(sys.argv) > 1 else ROOT / "data" / "registry.json"
    reg = json.loads(reg_path.read_text())
    schema = json.loads((ROOT / "data" / "registry.schema.json").read_text())

    problems = list(check(reg, schema)) + list(structural(reg))
    if problems:
        print(f"INVALID — {len(problems)} problem(s):")
        for p in problems:
            print(f"  - {p}")
        sys.exit(1)

    by_type = Counter(e["type"] for e in reg["entries"])
    print(f"ok — {len(reg['entries'])} entries "
          f"({by_type['handshake']} handshakes, {by_type['ecosystem_partner']} partners, "
          f"{by_type.get('project', 0)} plain projects); "
          f"{sum(1 for e in reg['entries'] if e.get('token') == 'token_live')} live tokens")


if __name__ == "__main__":
    main()
