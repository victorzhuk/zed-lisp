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
fn catalog_schema_rejects_invalid_entries_and_provenance() {
    let schema = read_json(project_root().join("schemas/lispico-catalog.schema.json"));
    let schema = jsonschema::validator_for(&schema).expect("catalog schema must compile");

    let valid: Value = serde_json::from_str(
        r#"{
        "schema_version": 2,
        "owner": "go-lispico",
        "source_version": "0.3.0",
        "dialect": "clojure",
        "library": "core",
        "entries": [
            { "name": "map", "kind": "function", "cell": "value" },
            { "name": "pi", "kind": "value", "cell": "value" }
        ]
    }"#,
    )
    .unwrap();
    assert!(schema.is_valid(&valid), "minimal valid catalog is rejected");

    let mut value_with_arity = valid.clone();
    value_with_arity["entries"][1]["arity"] = serde_json::json!({ "min": 0 });
    assert!(
        !schema.is_valid(&value_with_arity),
        "value entries must not declare arity"
    );

    let mut clojure_function_cell = valid.clone();
    clojure_function_cell["entries"][0]["cell"] = Value::from("function");
    assert!(
        !schema.is_valid(&clojure_function_cell),
        "clojure catalogs must resolve every symbol through the value cell"
    );

    let mut cl_function_cell = valid.clone();
    cl_function_cell["dialect"] = Value::from("cl");
    cl_function_cell["entries"][0]["cell"] = Value::from("function");
    assert!(
        schema.is_valid(&cl_function_cell),
        "cl (Lisp-2) catalogs may declare function-cell entries"
    );

    let mut no_provenance = valid.clone();
    no_provenance
        .as_object_mut()
        .unwrap()
        .remove("source_version");
    assert!(
        !schema.is_valid(&no_provenance),
        "catalogs must pin a source version or revision"
    );

    let mut both_provenance = valid.clone();
    both_provenance["source_revision"] = Value::from("f9ce4a1");
    assert!(
        !schema.is_valid(&both_provenance),
        "exactly one of source_version and source_revision is allowed"
    );

    let mut unknown_field = valid.clone();
    unknown_field["entries"][0]["signature"] = Value::from("(map f coll)");
    assert!(
        !schema.is_valid(&unknown_field),
        "invented signature fields must be rejected"
    );

    let mut empty_entries = valid.clone();
    empty_entries["entries"] = Value::from(Vec::<Value>::new());
    assert!(!schema.is_valid(&empty_entries));

    let fingerprint = format!("sha256:{}", "a".repeat(64));
    let paths: Vec<String> = (0..256).map(|i| format!("src/file{i}.lisp")).collect();
    let mut pinned = valid.clone();
    pinned["source_revision"] = Value::from("f9ce4a1");
    pinned.as_object_mut().unwrap().remove("source_version");
    pinned["source_fingerprint"] = Value::from(fingerprint.clone());
    pinned["source_files"] = Value::from(paths.clone());
    pinned["source_files"][0] = Value::from("dir/a:b.lisp");
    pinned["source_files"][1] = Value::from("name:variant.lisp");
    assert!(
        schema.is_valid(&pinned),
        "path-string source_files with colon segments are accepted"
    );

    let mut object_files = pinned.clone();
    object_files["source_files"][0] =
        serde_json::json!({ "path": "src/file0.lisp", "digest": fingerprint });
    assert!(
        !schema.is_valid(&object_files),
        "source_files must be path strings, not objects"
    );

    let mut duplicate_files = pinned.clone();
    duplicate_files["source_files"][1] = Value::from("dir/a:b.lisp");
    assert!(
        !schema.is_valid(&duplicate_files),
        "duplicate source_files entries are rejected"
    );

    let mut too_many_files = pinned.clone();
    too_many_files["source_files"] = Value::from(
        (0..257)
            .map(|i| Value::from(format!("src/file{i}.lisp")))
            .collect::<Vec<_>>(),
    );
    assert!(
        !schema.is_valid(&too_many_files),
        "source_files is capped at 256 entries"
    );

    let mut no_files = pinned.clone();
    no_files.as_object_mut().unwrap().remove("source_files");
    assert!(
        !schema.is_valid(&no_files),
        "source_fingerprint without source_files is rejected"
    );

    let mut unprefixed_fingerprint = pinned.clone();
    unprefixed_fingerprint["source_fingerprint"] = Value::from("a".repeat(64));
    assert!(
        !schema.is_valid(&unprefixed_fingerprint),
        "source_fingerprint must carry the sha256: prefix"
    );

    let mut bad_path = pinned.clone();
    bad_path["source_files"][0] = Value::from("C:/abs/src/a.lisp");
    assert!(
        !schema.is_valid(&bad_path),
        "source_files entries reject drive-prefixed absolute paths"
    );

    bad_path["source_files"][0] = Value::from("C:rel/src/a.lisp");
    assert!(
        !schema.is_valid(&bad_path),
        "source_files entries reject drive-relative prefixes"
    );

    bad_path["source_files"][0] = Value::from("src\\a.lisp");
    assert!(
        !schema.is_valid(&bad_path),
        "source_files entries reject backslash separators"
    );

    bad_path["source_files"][0] = Value::from("src/./a.lisp");
    assert!(
        !schema.is_valid(&bad_path),
        "source_files entries reject dot segments"
    );

    bad_path["source_files"][0] = Value::from("src/../a.lisp");
    assert!(
        !schema.is_valid(&bad_path),
        "source_files entries reject parent segments"
    );

    bad_path["source_files"][0] = Value::from("src/a.lisp/");
    assert!(
        !schema.is_valid(&bad_path),
        "source_files entries reject trailing separators"
    );

    bad_path["source_files"][0] = Value::from("src/\u{0}");
    assert!(
        !schema.is_valid(&bad_path),
        "source_files entries reject NUL bytes"
    );

    let project_schema = read_json(project_root().join("schemas/lispico-project.schema.json"));
    let project_schema =
        jsonschema::validator_for(&project_schema).expect("project schema must compile");
    let catalog = |fingerprint: Value| {
        serde_json::json!({
            "schema_version": 2,
            "contexts": [{
                "name": "a",
                "files": ["src/**/*.lisp"],
                "dialect": "clojure",
                "profile": "runtime"
            }],
            "catalogs": [{
                "path": "vendor/catalog.json",
                "owner": "zhk",
                "expected_revision": "f9ce4a1",
                "expected_fingerprint": fingerprint
            }]
        })
    };
    assert!(
        project_schema.is_valid(&catalog(Value::from(format!("sha256:{}", "a".repeat(64))))),
        "a prefixed expected_fingerprint is a valid catalog pin"
    );
    for bad in [
        Value::from("a".repeat(64)),
        Value::from(format!("sha256:{}", "A".repeat(64))),
        Value::from(format!("sha256:{}", "a".repeat(63))),
    ] {
        assert!(
            !project_schema.is_valid(&catalog(bad)),
            "expected_fingerprint must be a prefixed lowercase sha256 digest"
        );
    }
}

#[test]
fn project_schema_rejects_unknown_and_invalid_configuration() {
    let schema = read_json(project_root().join("schemas/lispico-project.schema.json"));
    let schema = jsonschema::validator_for(&schema).unwrap();

    let base: Value = serde_json::from_str(
        r#"{
        "schema_version": 2,
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
    bad_version["schema_version"] = Value::from(3);
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
        "schema_version": 2,
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
fn source_manifest_contracts() {
    let schema = read_json(project_root().join("schemas/lispico-catalog.schema.json"));
    let schema = jsonschema::validator_for(&schema).expect("catalog schema must compile");

    let digest = "a".repeat(64);
    let base: Value = serde_json::from_str(
        r#"{
        "schema_version": 2,
        "owner": "go-lispico",
        "source_version": "0.3.0",
        "dialect": "clojure",
        "library": "core",
        "entries": [
            { "name": "map", "kind": "function", "cell": "value" },
            { "name": "pi", "kind": "value", "cell": "value" }
        ]
    }"#,
    )
    .unwrap();

    assert!(
        schema.is_valid(&base),
        "a catalog without a source manifest is valid"
    );

    let mut with_manifest = base.clone();
    with_manifest["source_fingerprint"] = Value::from(format!("sha256:{digest}"));
    with_manifest["source_files"] = Value::from(vec!["src/core.lisp"]);
    assert!(
        schema.is_valid(&with_manifest),
        "a complete source manifest is valid"
    );

    let mut files_only = base.clone();
    files_only["source_files"] = with_manifest["source_files"].clone();
    assert!(
        !schema.is_valid(&files_only),
        "source_files requires source_fingerprint"
    );

    let mut fingerprint_only = base.clone();
    fingerprint_only["source_fingerprint"] = Value::from(format!("sha256:{digest}"));
    assert!(
        !schema.is_valid(&fingerprint_only),
        "source_fingerprint requires source_files"
    );

    let mut bad_fingerprint = with_manifest.clone();
    bad_fingerprint["source_fingerprint"] = Value::from("not-a-sha256");
    assert!(
        !schema.is_valid(&bad_fingerprint),
        "source_fingerprint must be a prefixed lowercase sha256"
    );

    let mut object_files = with_manifest.clone();
    object_files["source_files"][0] =
        serde_json::json!({ "path": "src/core.lisp", "digest": digest });
    assert!(
        !schema.is_valid(&object_files),
        "source_files must be path strings, not objects"
    );

    for path in [
        "/absolute/core.lisp",
        "back\\slash.lisp",
        "C:/drive.lisp",
        "./relative.lisp",
        "src/./core.lisp",
        "src//core.lisp",
        "src/core.lisp/",
        "..",
        "src/../core.lisp",
        "nul\u{0}byte.lisp",
    ] {
        let mut malformed = with_manifest.clone();
        malformed["source_files"][0] = Value::from(path);
        assert!(
            !schema.is_valid(&malformed),
            "non-canonical source path {path:?} is accepted"
        );
    }

    let mut unicode = with_manifest.clone();
    unicode["source_files"][0] = Value::from("источники/ядро.lisp");
    assert!(
        schema.is_valid(&unicode),
        "a canonical Unicode relative path is valid"
    );

    let mut full = with_manifest.clone();
    full["source_files"] = Value::from(
        (0..256)
            .map(|index| Value::from(format!("src/file{index}.lisp")))
            .collect::<Vec<_>>(),
    );
    assert!(
        schema.is_valid(&full),
        "256 source files is the accepted maximum"
    );

    let mut overfull = full.clone();
    overfull["source_files"]
        .as_array_mut()
        .unwrap()
        .push(Value::from("src/extra.lisp"));
    assert!(
        !schema.is_valid(&overfull),
        "257 source files exceeds the limit"
    );

    let mut duplicate = with_manifest.clone();
    duplicate["source_files"] = Value::from(vec!["src/core.lisp", "src/core.lisp"]);
    assert!(
        !schema.is_valid(&duplicate),
        "duplicate source entries are rejected"
    );

    let mut cl_same_name = base.clone();
    cl_same_name["dialect"] = Value::from("cl");
    cl_same_name["entries"] = Value::from(vec![
        serde_json::json!({ "name": "map", "kind": "function", "cell": "function" }),
        serde_json::json!({ "name": "map", "kind": "function", "cell": "value" }),
    ]);
    assert!(
        schema.is_valid(&cl_same_name),
        "cl catalogs may hold function- and value-cell entries under one name"
    );

    let mut clojure_function = base.clone();
    clojure_function["entries"][0]["cell"] = Value::from("function");
    assert!(
        !schema.is_valid(&clojure_function),
        "clojure catalogs resolve through the value cell only"
    );
}

#[test]
fn packs_layer_selection_contracts() {
    let schema = read_json(project_root().join("schemas/lispico-project.schema.json"));
    let schema = jsonschema::validator_for(&schema).expect("project schema must compile");

    let fingerprint = format!("sha256:{}", "b".repeat(64));

    let context_with = |layers: Value| {
        serde_json::json!({
            "schema_version": 2,
            "contexts": [{
                "name": "rules",
                "files": ["rules/**/*.clj"],
                "dialect": "clojure",
                "profile": "yagel-rule",
                "layers": layers
            }]
        })
    };

    assert!(
        schema.is_valid(&context_with(serde_json::json!([
            { "kind": "embedded" },
            { "kind": "packs", "root": "packs" },
            { "kind": "project", "root": "rules" }
        ]))),
        "a live packs root is a valid selection"
    );

    assert!(
        schema.is_valid(&context_with(serde_json::json!([
            { "kind": "packs", "snapshot": "packs/snapshot.json", "expected_fingerprint": fingerprint }
        ]))),
        "a locked snapshot selection is valid"
    );

    assert!(
        !schema.is_valid(&context_with(serde_json::json!([{ "kind": "packs" }]))),
        "a packs layer with neither root nor snapshot is rejected"
    );

    assert!(
        !schema.is_valid(&context_with(serde_json::json!([
            { "kind": "packs", "root": "packs", "snapshot": "packs/snapshot.json", "expected_fingerprint": fingerprint }
        ]))),
        "root and the snapshot pair are mutually exclusive"
    );

    assert!(
        !schema.is_valid(&context_with(serde_json::json!([
            { "kind": "packs", "root": "packs", "snapshot": "packs/snapshot.json" }
        ]))),
        "root cannot be combined with a snapshot"
    );

    assert!(
        !schema.is_valid(&context_with(serde_json::json!([
            { "kind": "packs", "snapshot": "packs/snapshot.json" }
        ]))),
        "a snapshot requires its expected fingerprint"
    );

    assert!(
        !schema.is_valid(&context_with(serde_json::json!([
            { "kind": "packs", "expected_fingerprint": fingerprint }
        ]))),
        "an expected fingerprint requires its snapshot"
    );

    assert!(
        !schema.is_valid(&context_with(serde_json::json!([
            { "kind": "global", "root": "vendor", "snapshot": "packs/snapshot.json" }
        ]))),
        "only packs layers may select a snapshot"
    );

    assert!(
        !schema.is_valid(&context_with(serde_json::json!([
            { "kind": "embedded", "expected_fingerprint": fingerprint }
        ]))),
        "only packs layers may pin a snapshot fingerprint"
    );

    for bad in [
        "b".repeat(64),
        format!("sha256:{}", "B".repeat(64)),
        "sha256:abcd".to_string(),
    ] {
        assert!(
            !schema.is_valid(&context_with(serde_json::json!([
                { "kind": "packs", "snapshot": "packs/snapshot.json", "expected_fingerprint": bad }
            ]))),
            "expected_fingerprint must be a prefixed lowercase sha256 digest"
        );
    }

    assert!(
        schema.is_valid(&context_with(serde_json::json!([
            { "kind": "packs", "snapshot": "name:variant/snapshot.json", "expected_fingerprint": fingerprint }
        ]))),
        "snapshot paths with colon segments are accepted"
    );

    for bad in ["C:relative", "a:variant"] {
        assert!(
            !schema.is_valid(&context_with(serde_json::json!([
                { "kind": "packs", "snapshot": bad, "expected_fingerprint": fingerprint }
            ]))),
            "snapshot paths reject drive-relative prefixes"
        );
    }
}

#[test]
fn installed_pack_snapshot_contracts() {
    let schema = read_json(project_root().join("schemas/lispico-packs.schema.json"));
    let schema = jsonschema::validator_for(&schema).expect("packs schema must compile");

    let digest = format!("sha256:{}", "c".repeat(64));
    let valid: Value = serde_json::from_str(&format!(
        r#"{{
        "schema_version": 1,
        "source_revision": "f9ce4a1",
        "selection_generation": "v2",
        "packs": [{{ "name": "memory", "digest": "opaque:abc" }}],
        "entries": [
            {{ "key": "rules/memory.clj", "pack": "memory", "status": "readable", "source": "rules/memory.clj", "source_digest": "{digest}" }}
        ],
        "problems": [{{ "message": "skipped", "pack": "memory", "key": "rules/other.clj" }}]
    }}"#
    ))
    .unwrap();
    assert!(schema.is_valid(&valid), "a complete snapshot is valid");

    let empty: Value = serde_json::from_str(
        r#"{
        "schema_version": 1,
        "source_revision": "f9ce4a1",
        "selection_generation": "v2",
        "packs": [],
        "entries": [],
        "problems": []
    }"#,
    )
    .unwrap();
    assert!(
        schema.is_valid(&empty),
        "an empty v2-generation pack set is valid"
    );

    for field in [
        "schema_version",
        "source_revision",
        "selection_generation",
        "packs",
        "entries",
        "problems",
    ] {
        let mut missing = valid.clone();
        missing.as_object_mut().unwrap().remove(field);
        assert!(!schema.is_valid(&missing), "root field {field} is required");
    }

    let mut bad_generation = valid.clone();
    bad_generation["selection_generation"] = Value::from("v3");
    assert!(
        !schema.is_valid(&bad_generation),
        "unknown selection generations are rejected"
    );

    let mut readable_without_source = valid.clone();
    readable_without_source["entries"][0]
        .as_object_mut()
        .unwrap()
        .remove("source");
    assert!(
        !schema.is_valid(&readable_without_source),
        "readable entries carry their source path"
    );

    let mut bad_source_digest = valid.clone();
    bad_source_digest["entries"][0]["source_digest"] = Value::from("short");
    assert!(
        !schema.is_valid(&bad_source_digest),
        "readable entries carry a prefixed lowercase sha256 digest"
    );

    for bad in [
        "c".repeat(64),
        format!("sha256:{}", "C".repeat(64)),
        "sha256:abcd".to_string(),
    ] {
        let mut bad_digest = valid.clone();
        bad_digest["entries"][0]["source_digest"] = Value::from(bad);
        assert!(
            !schema.is_valid(&bad_digest),
            "source_digest must be a prefixed lowercase sha256 digest"
        );
    }

    assert!(
        {
            let mut colon_source = valid.clone();
            colon_source["entries"][0]["source"] = Value::from("name:variant/memory.clj");
            schema.is_valid(&colon_source)
        },
        "entry sources with colon segments are accepted"
    );

    assert!(
        {
            let mut colon_source = valid.clone();
            colon_source["entries"][0]["source"] = Value::from("dir/a:b.clj");
            schema.is_valid(&colon_source)
        },
        "entry sources with nested colon segments are accepted"
    );

    for bad in ["C:relative", "a:variant"] {
        let mut bad_source_path = valid.clone();
        bad_source_path["entries"][0]["source"] = Value::from(bad);
        assert!(
            !schema.is_valid(&bad_source_path),
            "entry sources reject drive-relative prefixes"
        );
    }

    let mut bad_source_path = valid.clone();
    bad_source_path["entries"][0]["source"] = Value::from("/abs/rules/memory.clj");
    assert!(
        !schema.is_valid(&bad_source_path),
        "entry sources are canonical relative paths"
    );

    let unreadable: Value = serde_json::json!({
        "schema_version": 1,
        "source_revision": "f9ce4a1",
        "selection_generation": "v1",
        "packs": [],
        "entries": [{ "key": "rules/broken.clj", "pack": "memory", "status": "unreadable" }],
        "problems": [{ "message": "unreadable" }]
    });
    assert!(
        schema.is_valid(&unreadable),
        "unreadable entries omit source information"
    );

    let mut unreadable_with_source = unreadable.clone();
    unreadable_with_source["entries"][0]["source"] = Value::from("rules/broken.clj");
    assert!(
        !schema.is_valid(&unreadable_with_source),
        "unreadable entries must not claim a source"
    );

    let mut unknown_root = valid.clone();
    unknown_root["extra"] = Value::Bool(true);
    assert!(
        !schema.is_valid(&unknown_root),
        "unknown root fields are rejected"
    );

    let mut unknown_entry = valid.clone();
    unknown_entry["entries"][0]["line"] = Value::from(1);
    assert!(
        !schema.is_valid(&unknown_entry),
        "unknown entry fields are rejected"
    );

    let mut unknown_problem = valid.clone();
    unknown_problem["problems"][0]["severity"] = Value::from("warn");
    assert!(
        !schema.is_valid(&unknown_problem),
        "unknown problem fields are rejected"
    );

    let mut pack_without_digest = valid.clone();
    pack_without_digest["packs"][0]
        .as_object_mut()
        .unwrap()
        .remove("digest");
    assert!(
        !schema.is_valid(&pack_without_digest),
        "packs carry a digest"
    );

    let mut empty_problem = valid.clone();
    empty_problem["problems"][0]["message"] = Value::from("");
    assert!(
        !schema.is_valid(&empty_problem),
        "problem messages are non-empty"
    );

    let mut readable_without_digest = valid.clone();
    readable_without_digest["entries"][0]
        .as_object_mut()
        .unwrap()
        .remove("source_digest");
    assert!(
        !schema.is_valid(&readable_without_digest),
        "readable entries must carry a source digest"
    );

    let mut unreadable_with_digest = unreadable.clone();
    unreadable_with_digest["entries"][0]["source_digest"] = Value::from(digest.clone());
    assert!(
        !schema.is_valid(&unreadable_with_digest),
        "unreadable entries must not claim a source digest"
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

    // The go-lispico template's CL corpus fixture must parse in the mode its
    // template selects for it (Lispico CL, which is backed by the clojure
    // grammar).
    let source =
        std::fs::read_to_string(project_root().join("examples/go-lispico/corpus/examples.lisp"))
            .unwrap();
    parse(
        &clojure,
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

/// Runs the packaging checker against a mutated manifest copy and returns
/// (exit status, stderr). The mutation, not the clean run, is the proof.
fn check_package_with_manifest(manifest_text: &str) -> (i32, String) {
    let dir = std::env::temp_dir().join(format!(
        "check-package-mutation-{}-{}",
        std::process::id(),
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos()
    ));
    std::fs::create_dir_all(&dir).unwrap();
    let manifest_path = dir.join("extension.toml");
    std::fs::write(&manifest_path, manifest_text).unwrap();

    let output = std::process::Command::new("python3")
        .arg(project_root().join("scripts/check_package.py"))
        .arg("--manifest")
        .arg(&manifest_path)
        .output()
        .expect("python3 must be available to run scripts/check_package.py");
    let _ = std::fs::remove_dir_all(&dir);
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

fn clean_manifest_text() -> String {
    std::fs::read_to_string(project_root().join("extension.toml")).unwrap()
}

#[test]
fn check_package_fails_when_a_language_has_no_identifier_entry() {
    // Drop the Lispico CL identifier from the map, leaving the other two.
    let mutated = clean_manifest_text().replace(
        "\"Lispico CL\" = \"lispico-cl\"",
        "# no identifier declared",
    );
    assert_ne!(mutated, clean_manifest_text());
    let (code, stderr) = check_package_with_manifest(&mutated);
    assert_eq!(code, 1, "a missing language_ids entry must fail the check");
    assert!(
        stderr.contains("no language_ids entry"),
        "the failure must name the missing identifier: {stderr}"
    );
}

#[test]
fn check_package_fails_when_a_previous_server_entry_survives() {
    let mutated = format!(
        "{}\n[language_servers.sextant]\nname = \"sextant\"\nlanguages = [\"Common Lisp\"]\n\n[language_servers.sextant.language_ids]\n\"Common Lisp\" = \"lisp\"\n",
        clean_manifest_text()
    );
    let (code, stderr) = check_package_with_manifest(&mutated);
    assert_eq!(
        code, 1,
        "a surviving previous server entry must fail the check"
    );
    assert!(
        stderr.contains("sextant") && stderr.contains("only 'llsp'"),
        "the failure must name the rejected server: {stderr}"
    );
}

#[test]
fn check_package_fails_when_the_llsp_entry_is_absent() {
    let manifest = clean_manifest_text();
    let cut = manifest
        .find("[language_servers.")
        .expect("the clean manifest declares a language server");
    let mutated = manifest[..cut].to_string();
    let (code, stderr) = check_package_with_manifest(&mutated);
    assert_eq!(code, 1, "a manifest without any server entry must fail");
    assert!(
        stderr.contains("must declare the 'llsp' server"),
        "the failure must name the absent server: {stderr}"
    );
}
