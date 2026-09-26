//! Verifies the shipped configuration surface: extension manifest
//! references, snippets, project/catalog schemas, and the portable project
//! templates with their glob behavior.

mod common;

use common::{
    clojure_language, commonlisp_language, expand_snippet, glob_match, parse, project_root,
};
use serde_json::Value;
use std::collections::BTreeMap;
use std::path::PathBuf;

fn read_json(path: std::path::PathBuf) -> Value {
    let text =
        std::fs::read_to_string(&path).unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    serde_json::from_str(&text).unwrap_or_else(|err| panic!("{}: {err}", path.display()))
}

fn manifest() -> toml::Value {
    toml::from_str(&std::fs::read_to_string(project_root().join("extension.toml")).unwrap())
        .unwrap()
}

/// Representative source paths for each template, mapped to the mode its
/// `file_types` entry must select.
fn template_fixtures() -> Vec<(&'static str, Vec<(&'static str, &'static str)>)> {
    vec![
        (
            "zhk",
            vec![
                ("workflows/plan/main.lisp", "Lispico Clojure"),
                ("workflows/lib/00-core.lisp", "Lispico Clojure"),
                ("README.md", "__unmatched__"),
            ],
        ),
        (
            "yagel",
            vec![
                ("rules/defaults/doctor.clj", "Lispico Clojure"),
                ("packs/memory/rules/memory.clj", "Lispico Clojure"),
                ("README.md", "__unmatched__"),
            ],
        ),
        (
            "go-lispico",
            vec![
                ("internal/goldset/testdata/pipeline.lisp", "Lispico Clojure"),
                ("corpus/examples.lisp", "Lispico CL"),
                ("src/common_lisp.rs", "__unmatched__"),
            ],
        ),
    ]
}

#[test]
fn manifest_references_existing_resources() {
    let manifest = manifest();

    let grammars = manifest.get("grammars").and_then(|g| g.as_table()).unwrap();
    for (name, entry) in grammars {
        assert!(
            entry.get("repository").and_then(|v| v.as_str()).is_some(),
            "grammar {name}: missing repository"
        );
        assert!(
            entry.get("commit").and_then(|v| v.as_str()).is_some(),
            "grammar {name}: missing pinned commit"
        );
        assert!(
            project_root()
                .join("grammars")
                .join(name)
                .join("src/parser.c")
                .exists(),
            "grammar {name}: pinned submodule sources missing (run `git submodule update --init --recursive`)"
        );
    }

    for (language, path) in snippet_files(&manifest) {
        assert!(path.is_file(), "snippet file {} is missing", path.display());
        language_dir_for_name(&language);
    }

    let servers = manifest
        .get("language_servers")
        .and_then(|s| s.as_table())
        .expect("extension.toml must register language servers");
    let language_names: Vec<String> = ["commonlisp", "lispico-clojure", "lispico-cl"]
        .iter()
        .map(|dir| {
            let config: toml::Value = toml::from_str(
                &std::fs::read_to_string(
                    project_root()
                        .join("languages")
                        .join(dir)
                        .join("config.toml"),
                )
                .unwrap(),
            )
            .unwrap();
            config
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap()
                .to_string()
        })
        .collect();

    for (server, entry) in servers {
        let languages = entry
            .get("languages")
            .and_then(|l| l.as_array())
            .expect("language server must declare languages");
        assert!(
            !languages.is_empty(),
            "language server {server} declares no languages"
        );
        for language in languages {
            let language = language.as_str().unwrap();
            assert!(
                language_names.iter().any(|n| n == language),
                "server {server}: language {language:?} has no config.toml"
            );
        }
    }
}

#[test]
fn language_modes_do_not_claim_global_suffixes() {
    for dir in ["lispico-clojure", "lispico-cl"] {
        let config: toml::Value = toml::from_str(
            &std::fs::read_to_string(
                project_root()
                    .join("languages")
                    .join(dir)
                    .join("config.toml"),
            )
            .unwrap(),
        )
        .unwrap();
        let suffixes = config.get("path_suffixes");
        assert!(
            suffixes.is_none() || suffixes.unwrap().as_array().is_some_and(|s| s.is_empty()),
            "{dir}: Lispico modes must not claim global path suffixes"
        );
    }
}

#[test]
fn common_lisp_defaults_are_preserved() {
    let config: toml::Value = toml::from_str(
        &std::fs::read_to_string(
            project_root()
                .join("languages")
                .join("commonlisp")
                .join("config.toml"),
        )
        .unwrap(),
    )
    .unwrap();

    let suffixes: Vec<&str> = config
        .get("path_suffixes")
        .and_then(|s| s.as_array())
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect();
    assert_eq!(suffixes, vec!["lisp", "lsp", "cl", "asd"]);
    assert_eq!(
        config.get("grammar").and_then(|g| g.as_str()),
        Some("commonlisp")
    );
}

/// Resolves the manifest's snippet files to the language each one serves.
/// Zed keys a snippet file by its stem and matches it against the lowercased
/// language name, so every stem must equal some language's scope id.
fn snippet_files(manifest: &toml::Value) -> Vec<(String, PathBuf)> {
    let paths: Vec<&str> = match manifest
        .get("snippets")
        .expect("extension.toml must register snippet files")
    {
        toml::Value::String(path) => vec![path.as_str()],
        toml::Value::Array(paths) => paths.iter().map(|p| p.as_str().unwrap()).collect(),
        other => panic!("snippets must be a path or a list of paths, got {other:?}"),
    };
    paths
        .into_iter()
        .map(|entry| {
            let path = project_root().join(entry);
            let stem = path.file_stem().unwrap().to_str().unwrap().to_string();
            let language = ["commonlisp", "lispico-clojure", "lispico-cl"]
                .iter()
                .map(|dir| language_name(dir))
                .find(|name| name.to_lowercase() == stem)
                .unwrap_or_else(|| panic!("snippet file {entry} matches no language scope"));
            (language, path)
        })
        .collect()
}

fn language_name(dir: &str) -> String {
    let config: toml::Value = toml::from_str(
        &std::fs::read_to_string(
            project_root()
                .join("languages")
                .join(dir)
                .join("config.toml"),
        )
        .unwrap(),
    )
    .unwrap();
    config
        .get("name")
        .and_then(|n| n.as_str())
        .unwrap()
        .to_string()
}

fn language_dir_for_name(language: &str) -> String {
    ["commonlisp", "lispico-clojure", "lispico-cl"]
        .into_iter()
        .find(|dir| language_name(dir) == language)
        .unwrap_or_else(|| panic!("no language directory declares name {language:?}"))
        .to_string()
}

fn language_grammar(language: &str) -> &'static str {
    let config: toml::Value = toml::from_str(
        &std::fs::read_to_string(
            project_root()
                .join("languages")
                .join(language_dir_for_name(language))
                .join("config.toml"),
        )
        .unwrap(),
    )
    .unwrap();
    match config.get("grammar").and_then(|g| g.as_str()).unwrap() {
        "clojure" => "clojure",
        "commonlisp" => "commonlisp",
        other => panic!("unknown grammar {other:?}"),
    }
}

#[test]
fn expanded_snippets_parse_in_their_language() {
    let manifest = manifest();
    for (language, path) in snippet_files(&manifest) {
        let grammar = match language_grammar(&language) {
            "clojure" => clojure_language(),
            _ => commonlisp_language(),
        };
        let snippets: BTreeMap<String, Value> =
            serde_json::from_str(&std::fs::read_to_string(&path).unwrap())
                .unwrap_or_else(|err| panic!("{}: {err}", path.display()));
        assert!(!snippets.is_empty(), "{}: no snippets", path.display());
        for (prefix, snippet) in snippets {
            let body = snippet
                .get("body")
                .and_then(|b| b.as_str())
                .unwrap_or_else(|| panic!("{}: {prefix} has no body", path.display()));
            let expanded = expand_snippet(body);
            parse(
                &grammar,
                &expanded,
                true,
                &format!("{} snippet {prefix}", path.display()),
            );
        }
    }
}

#[test]
fn project_templates_validate_against_the_schema() {
    let schema = read_json(project_root().join("schemas/lispico-project.schema.json"));
    let schema = jsonschema::validator_for(&schema).expect("project schema must compile");

    for project in ["zhk", "yagel", "go-lispico"] {
        let path = project_root()
            .join("examples")
            .join(project)
            .join(".lispico.json");
        let instance = read_json(path.clone());
        schema
            .validate(&instance)
            .unwrap_or_else(|err| panic!("{}: {err}", path.display()));
    }
}

#[test]
fn shipped_catalog_schema_compiles() {
    let schema = read_json(project_root().join("schemas/lispico-catalog.schema.json"));
    jsonschema::validator_for(&schema).expect("catalog schema must compile");
}

#[test]
fn project_schema_rejects_unknown_and_invalid_configuration() {
    let schema = read_json(project_root().join("schemas/lispico-project.schema.json"));
    let schema = jsonschema::validator_for(&schema).unwrap();

    let base: Value = serde_json::from_str(
        r#"{
        "schema_version": 1,
        "contexts": [
            {
                "name": "a",
                "files": ["src/**/*.lisp"],
                "dialect": "clojure",
                "profile": "runtime"
            }
        ]
    }"#,
    )
    .unwrap();

    assert!(schema.is_valid(&base));

    let mut unknown = base.clone();
    unknown["surprise"] = Value::Bool(true);
    assert!(
        !schema.is_valid(&unknown),
        "unknown root fields must be rejected"
    );

    let mut bad_version = base.clone();
    bad_version["schema_version"] = Value::from(2);
    assert!(
        !schema.is_valid(&bad_version),
        "unsupported schema versions must be rejected"
    );

    let mut no_dialect = base.clone();
    no_dialect["contexts"][0]
        .as_object_mut()
        .unwrap()
        .remove("dialect");
    assert!(!schema.is_valid(&no_dialect));

    let mut zhk_without_prelude = base.clone();
    zhk_without_prelude["contexts"][0]["profile"] = Value::from("zhk");
    assert!(
        !schema.is_valid(&zhk_without_prelude),
        "zhk contexts require a prelude"
    );

    let mut zhk_with_layers = zhk_without_prelude.clone();
    zhk_with_layers["contexts"][0]["prelude"] = Value::from(vec!["workflows/lib/*.lisp"]);
    zhk_with_layers["contexts"][0]["layers"] =
        Value::from(vec![serde_json::json!({"kind": "embedded"})]);
    assert!(
        !schema.is_valid(&zhk_with_layers),
        "zhk contexts must not declare layers"
    );

    let mut cl_host = base.clone();
    cl_host["contexts"][0]["profile"] = Value::from("zhk");
    cl_host["contexts"][0]["dialect"] = Value::from("cl");
    cl_host["contexts"][0]["prelude"] = Value::from(vec!["workflows/lib/*.lisp"]);
    assert!(
        !schema.is_valid(&cl_host),
        "host profiles are Lispico Clojure only"
    );

    let mut runtime_with_prelude = base.clone();
    runtime_with_prelude["contexts"][0]["prelude"] = Value::from(vec!["lib/*.lisp"]);
    assert!(!schema.is_valid(&runtime_with_prelude));

    let mut missing_files = base.clone();
    missing_files["contexts"][0]
        .as_object_mut()
        .unwrap()
        .remove("files");
    assert!(!schema.is_valid(&missing_files));

    let valid_zhk: Value = serde_json::from_str(
        r#"{
        "schema_version": 1,
        "contexts": [
            {
                "name": "a",
                "files": ["workflows/**/*.lisp"],
                "dialect": "clojure",
                "profile": "zhk",
                "libraries": ["stdlib", "json", "zhk"],
                "source_roots": ["workflows"],
                "prelude": ["workflows/lib/*.lisp"]
            }
        ]
    }"#,
    )
    .unwrap();
    assert!(schema.is_valid(&valid_zhk));

    let mut catalog_missing_identity = base.clone();
    catalog_missing_identity["catalogs"] = Value::from(vec![serde_json::json!({
        "path": "vendor/catalog.json",
        "owner": "zhk"
    })]);
    assert!(
        !schema.is_valid(&catalog_missing_identity),
        "catalogs must pin expected_version or expected_revision"
    );

    let mut catalog_conflicting_identity = base.clone();
    catalog_conflicting_identity["catalogs"] = Value::from(vec![serde_json::json!({
        "path": "vendor/catalog.json",
        "owner": "zhk",
        "expected_version": "0.3.0",
        "expected_revision": "f9ce4a1"
    })]);
    assert!(
        !schema.is_valid(&catalog_conflicting_identity),
        "catalog identity fields conflict when both version and revision are set"
    );

    let mut catalog_only = base.clone();
    catalog_only["catalogs"] = Value::from(vec![serde_json::json!({
        "path": "vendor/catalog.json",
        "owner": "zhk",
        "expected_revision": "f9ce4a1"
    })]);
    assert!(
        schema.is_valid(&catalog_only),
        "catalog-only provenance (no source_root) is valid"
    );
}

#[test]
fn template_contexts_do_not_overlap_on_representative_paths() {
    for (project, fixtures) in template_fixtures() {
        let instance = read_json(
            project_root()
                .join("examples")
                .join(project)
                .join(".lispico.json"),
        );

        let contexts = instance["contexts"].as_array().unwrap();
        let mut names = Vec::new();
        for context in contexts {
            let name = context["name"].as_str().unwrap();
            assert!(
                !names.contains(&name),
                "{project}: duplicate context {name}"
            );
            names.push(name);
        }

        for (path, expected_mode) in fixtures {
            let matching: Vec<&str> = contexts
                .iter()
                .filter(|context| {
                    let includes = context["files"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|g| glob_match(g.as_str().unwrap(), path));
                    let excludes = context
                        .get("exclude")
                        .and_then(|e| e.as_array())
                        .map(|patterns| {
                            patterns
                                .iter()
                                .any(|g| glob_match(g.as_str().unwrap(), path))
                        })
                        .unwrap_or(false);
                    includes && !excludes
                })
                .map(|context| context["name"].as_str().unwrap())
                .collect();

            if expected_mode == "__unmatched__" {
                assert!(
                    matching.is_empty(),
                    "{project}: {path} unexpectedly matched {matching:?}"
                );
            } else {
                assert_eq!(
                    matching.len(),
                    1,
                    "{project}: {path} must match exactly one context, got {matching:?}"
                );
            }
        }
    }
}

#[test]
fn template_file_types_select_the_documented_modes() {
    for (project, fixtures) in template_fixtures() {
        let settings = read_json(
            project_root()
                .join("examples")
                .join(project)
                .join(".zed/settings.json"),
        );
        let file_types = settings["file_types"].as_object().unwrap();

        for (path, expected_mode) in fixtures {
            let mut selected = Vec::new();
            for (mode, patterns) in file_types {
                for pattern in patterns.as_array().unwrap() {
                    if glob_match(pattern.as_str().unwrap(), path) {
                        selected.push(mode.clone());
                    }
                }
            }

            if expected_mode == "__unmatched__" {
                assert!(
                    selected.is_empty(),
                    "{project}: {path} unexpectedly selected {selected:?}"
                );
            } else {
                assert_eq!(
                    selected,
                    vec![expected_mode],
                    "{project}: {path} must select exactly {expected_mode:?}"
                );
            }
        }

        for mode in file_types.keys() {
            language_dir_for_name(mode);
        }
    }
}

#[test]
fn template_source_fixtures_parse_in_their_selected_mode() {
    let clojure = clojure_language();
    let commonlisp = commonlisp_language();

    // The go-lispico template's CL corpus fixture must parse in the mode its
    // template selects for it (Lispico CL).
    let source =
        std::fs::read_to_string(project_root().join("examples/go-lispico/corpus/examples.lisp"))
            .unwrap();
    parse(
        &commonlisp,
        &source,
        true,
        "examples/go-lispico/corpus/examples.lisp",
    );

    // Representative zhk and yagel fixture sources used by the template
    // tests come from the corpus; the mode they select must accept them.
    for path in [
        "tests/corpus/lispico-clojure/zhk-route.lisp",
        "tests/corpus/lispico-clojure/yagel-rule.clj",
    ] {
        let source = std::fs::read_to_string(project_root().join(path)).unwrap();
        parse(&clojure, &source, true, path);
    }
}
