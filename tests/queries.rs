//! Compiles every shipped query file against the grammar version the
//! extension pins, so a query that references an invalid grammar node fails
//! verification before packaging. The existing Common Lisp queries are part
//! of this gate: they must keep compiling against the pinned commonlisp
//! grammar.

mod common;

use common::{clojure_language, commonlisp_language, compile_query, project_root};
use std::path::PathBuf;

const LANGUAGES: &[(&str, fn() -> tree_sitter::Language)] = &[
    ("commonlisp", commonlisp_language),
    ("lispico-clojure", clojure_language),
    ("lispico-cl", clojure_language),
];

fn language_dir(name: &str) -> PathBuf {
    project_root().join("languages").join(name)
}

#[test]
fn shipped_queries_compile_against_pinned_grammars() {
    for (name, language) in LANGUAGES {
        let dir = language_dir(name);
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|err| panic!("language directory {name} is missing: {err}"));

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().is_none_or(|ext| ext != "scm") {
                continue;
            }
            let source = std::fs::read_to_string(&path).unwrap();
            compile_query(&language(), &source, &path.to_string_lossy());
        }
    }
}

#[test]
fn every_declared_language_has_a_config_with_a_registered_grammar() {
    let manifest: toml::Value =
        toml::from_str(&std::fs::read_to_string(project_root().join("extension.toml")).unwrap())
            .unwrap();

    let declared_grammars: Vec<&str> = manifest
        .get("grammars")
        .and_then(|g| g.as_table())
        .map(|t| t.keys().map(|k| k.as_str()).collect())
        .unwrap_or_default();

    for (name, _) in LANGUAGES {
        let config: toml::Value = toml::from_str(
            &std::fs::read_to_string(language_dir(name).join("config.toml"))
                .unwrap_or_else(|err| panic!("missing config for {name}: {err}")),
        )
        .unwrap();

        let grammar = config
            .get("grammar")
            .and_then(|g| g.as_str())
            .unwrap_or_else(|| panic!("{name}: config has no grammar"));
        assert!(
            declared_grammars.contains(&grammar),
            "{name}: grammar {grammar:?} is not declared in extension.toml"
        );
    }
}

fn query_captures<'a>(
    language: tree_sitter::Language,
    query_source: &str,
    code: &'a str,
) -> Vec<(String, String)> {
    use tree_sitter::{Query, QueryCursor, StreamingIterator};

    let query = Query::new(&language, query_source).unwrap();
    let tree = common::parse(&language, code, true, "capture fixture");
    let root = tree.root_node();
    let mut cursor = QueryCursor::new();
    let mut result = Vec::new();
    let mut matches = cursor.captures(&query, root, code.as_bytes());
    while let Some((matched, capture_index)) = matches.next() {
        let capture = &matched.captures[*capture_index];
        result.push((
            query.capture_names()[capture.index as usize].to_string(),
            capture.node.utf8_text(code.as_bytes()).unwrap().to_string(),
        ));
    }
    result
}

#[test]
fn highlight_queries_capture_expected_dialect_structures() {
    let clojure = clojure_language();
    let zhk_head =
        std::fs::read_to_string(project_root().join("languages/lispico-clojure/highlights.scm"))
            .unwrap();
    let captures = query_captures(
        clojure.clone(),
        &zhk_head,
        "(defn latest (step) (zhk/step step \"x\"))",
    );

    // defn is a definition keyword and its name a function; the zhk/ call is
    // a function reference with its namespace distinguishable.
    assert!(
        captures
            .iter()
            .any(|(name, text)| name == "keyword.function" && text == "defn"),
        "defn head must capture as keyword.function: {captures:?}"
    );
    assert!(
        captures
            .iter()
            .any(|(name, text)| name == "function" && text == "latest"),
        "defn name must capture as function: {captures:?}"
    );
    assert!(
        captures
            .iter()
            .any(|(name, text)| name == "type" && text == "zhk"),
        "qualified namespace must capture as type: {captures:?}"
    );

    let cl_head =
        std::fs::read_to_string(project_root().join("languages/lispico-cl/highlights.scm"))
            .unwrap();
    let captures = query_captures(
        clojure.clone(),
        &cl_head,
        "(defun route (event) #'count-down #(count-down 3))",
    );

    assert!(
        captures
            .iter()
            .any(|(name, text)| name == "function" && text == "count-down"),
        "the `#'` referenced symbol must capture as a function in Lispico CL: {captures:?}"
    );
    assert!(
        !captures
            .iter()
            .any(|(name, text)| name == "keyword.function" && text == "count-down"),
        "`#(...)` must not become an anonymous definition keyword: {captures:?}"
    );

    // Outline items exist only for named definitions in both modes.
    for (file, head) in [
        (
            "languages/lispico-clojure/outline.scm",
            "(defn named (x) x)",
        ),
        ("languages/lispico-cl/outline.scm", "(defun named (x) x)"),
    ] {
        let outline = std::fs::read_to_string(project_root().join(file)).unwrap();
        let captures = query_captures(clojure.clone(), &outline, head);
        assert!(
            captures
                .iter()
                .any(|(name, text)| name == "name" && text == "named"),
            "{file}: named definition must be an outline item: {captures:?}"
        );
        let captures = query_captures(clojure.clone(), &outline, "(fn [x] x)");
        assert!(
            captures.is_empty(),
            "{file}: anonymous functions must not be outline items: {captures:?}"
        );
    }
}

#[test]
fn common_lisp_list_textobjects_select_the_whole_interior() {
    let language = commonlisp_language();
    let query_source =
        std::fs::read_to_string(project_root().join("languages/commonlisp/textobjects.scm"))
            .unwrap();

    let mut inside: Vec<String> = query_captures(language, &query_source, "(f 1 2 3)")
        .into_iter()
        .filter(|(name, _)| name == "class.inside")
        .map(|(_, text)| text)
        .collect();
    inside.sort();
    inside.dedup();

    assert_eq!(
        inside,
        vec!["1", "2", "3"],
        "the list interior must cover every child after the head"
    );
}
