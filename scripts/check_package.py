#!/usr/bin/env python3
"""Packaging validation for the extension's declared resources.

Fails when the extension manifest references a resource that is missing, or
when a shipped resource is malformed, so a broken package is caught before
publication. Run directly or via `make check-package`.

Archive contract: the release tarball is a Zed extension package containing
extension.toml, languages/, snippets/, schemas/, examples/, README.md,
LICENSE and the compiled wasm only. grammars/ and .gitmodules are
checkout-only: Zed clones and compiles grammars itself from the
repository/commit pins in extension.toml, so the validator's grammar checks
apply to the working tree, never to the archive. The CI release job enforces
this same contract on the built tarball before upload.
"""

import json
import subprocess
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

QUERY_FILES = (
    "highlights.scm",
    "indents.scm",
    "injections.scm",
    "outline.scm",
    "textobjects.scm",
    "brackets.scm",
    "overrides.scm",
)

LANGUAGES = ("commonlisp", "lispico-clojure", "lispico-cl")


def fail(message: str) -> None:
    print(f"check-package: {message}", file=sys.stderr)
    sys.exit(1)


def git(*args: str) -> str:
    result = subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True)
    return result.stdout.strip()


def main() -> None:
    try:
        manifest = tomllib.loads((ROOT / "extension.toml").read_text())
    except tomllib.TOMLDecodeError as err:
        fail(f"extension.toml does not parse: {err}")

    cargo_version = ""
    for line in (ROOT / "Cargo.toml").read_text().splitlines():
        if line.startswith("version = "):
            cargo_version = line.split('"')[1]
            break
    if manifest.get("version") != cargo_version:
        fail(
            f"extension.toml version {manifest.get('version')!r} != "
            f"Cargo.toml version {cargo_version!r}"
        )

    for name, entry in manifest.get("grammars", {}).items():
        for key in ("repository", "commit"):
            if not entry.get(key):
                fail(f"grammars.{name} is missing {key}")
        parser = ROOT / "grammars" / name / "src" / "parser.c"
        if not parser.exists():
            fail(
                f"grammars/{name}/src/parser.c is missing; "
                "run `git submodule update --init --recursive`"
            )
        # Zed reuses grammars/<name> only when its origin equals the manifest
        # repository verbatim; a `.git` suffix makes the dev install fail.
        url = git("config", "-f", ".gitmodules", f"submodule.grammars/{name}.url")
        if url != entry["repository"]:
            fail(f"submodule grammars/{name} url {url!r} != grammars.{name}.repository")
        head = git("-C", f"grammars/{name}", "rev-parse", "HEAD")
        if head != entry["commit"]:
            fail(f"submodule grammars/{name} is at {head}, manifest pins {entry['commit']}")

    languages = {}
    for directory in LANGUAGES:
        config = ROOT / "languages" / directory / "config.toml"
        if not config.exists():
            fail(f"languages/{directory}/config.toml is missing")
        try:
            parsed = tomllib.loads(config.read_text())
        except tomllib.TOMLDecodeError as err:
            fail(f"languages/{directory}/config.toml does not parse: {err}")
        name = parsed.get("name")
        if not name:
            fail(f"languages/{directory}/config.toml has no name")
        if name in languages:
            fail(f"duplicate language name {name!r}")
        languages[name] = directory
        grammar = parsed.get("grammar")
        if grammar not in manifest.get("grammars", {}):
            fail(f"language {name}: grammar {grammar!r} is not declared in extension.toml")
        for query in QUERY_FILES:
            path = ROOT / "languages" / directory / query
            if path.exists() and not path.read_text().strip():
                fail(f"languages/{directory}/{query} is empty")

    for server, entry in manifest.get("language_servers", {}).items():
        for language in entry.get("languages", []):
            if language not in languages:
                fail(f"language server {server}: language {language!r} has no config.toml")

    snippets = manifest.get("snippets", [])
    if isinstance(snippets, str):
        snippets = [snippets]
    if not isinstance(snippets, list):
        fail("snippets must be a path or a list of paths")
    # Zed keys snippet files by stem and matches it against the lowercased language name.
    scopes = {name.lower(): name for name in languages}
    for entry in snippets:
        path = ROOT / entry
        if not path.is_file():
            fail(f"snippet file {entry} is missing")
        if path.stem != "snippets" and path.stem not in scopes:
            fail(f"snippet file {entry} does not match any language scope")
        try:
            entries = json.loads(path.read_text())
        except json.JSONDecodeError as err:
            fail(f"{path} does not parse: {err}")
        if not entries:
            fail(f"{path} contains no snippets")
        for prefix, snippet in entries.items():
            if not snippet.get("body"):
                fail(f"{path}: snippet {prefix!r} has an empty body")

    for schema in ("lispico-project.schema.json", "lispico-catalog.schema.json"):
        path = ROOT / "schemas" / schema
        try:
            json.loads(path.read_text())
        except json.JSONDecodeError as err:
            fail(f"{path} does not parse: {err}")

    for project in ("zhk", "yagel", "go-lispico"):
        base = ROOT / "examples" / project
        config = base / ".lispico.json"
        if not config.exists():
            fail(f"examples/{project}/.lispico.json is missing")
        try:
            json.loads(config.read_text())
        except json.JSONDecodeError as err:
            fail(f"{config} does not parse: {err}")
        settings = base / ".zed" / "settings.json"
        if not settings.exists():
            fail(f"examples/{project}/.zed/settings.json is missing")
        try:
            settings_json = json.loads(settings.read_text())
        except json.JSONDecodeError as err:
            fail(f"{settings} does not parse: {err}")
        for mode in settings_json.get("file_types", {}):
            if mode not in languages:
                fail(f"{settings}: file_types references unknown language {mode!r}")

    print("check-package: ok")


if __name__ == "__main__":
    main()
