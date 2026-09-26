//! Parses the corpus: every complete fixture must parse its pinned grammar
//! without `ERROR` or `MISSING` nodes, and the recovery fixtures must not
//! hang the parser. See tests/corpus/README.md for fixture provenance.

mod common;

use common::{clojure_language, commonlisp_language, parse, project_root};
use std::path::PathBuf;

const CORPUS_GRAMMARS: &[(&str, fn() -> tree_sitter::Language)] = &[
    ("lispico-clojure", clojure_language),
    ("lispico-cl", clojure_language),
    ("commonlisp", commonlisp_language),
];

fn corpus_dir() -> PathBuf {
    project_root().join("tests/corpus")
}

fn lisp_sources(dir: &std::path::Path, label: &str) -> Vec<PathBuf> {
    let entries = std::fs::read_dir(dir)
        .unwrap_or_else(|err| panic!("corpus directory {label} is missing: {err}"));
    let mut sources = Vec::new();
    for entry in entries {
        let entry = entry.unwrap_or_else(|err| panic!("reading corpus directory {label}: {err}"));
        let path = entry.path();
        if path
            .extension()
            .is_none_or(|ext| ext != "lisp" && ext != "clj")
        {
            continue;
        }
        sources.push(path);
    }
    assert!(
        !sources.is_empty(),
        "{label}: no corpus fixtures found; an empty corpus silently validates nothing"
    );
    sources
}

#[test]
fn complete_corpus_parses_without_errors() {
    for (dir, language) in CORPUS_GRAMMARS {
        let dir = corpus_dir().join(dir);
        for path in lisp_sources(&dir, dir.file_name().unwrap().to_str().unwrap()) {
            let source = std::fs::read_to_string(&path).unwrap();
            let label = path
                .strip_prefix(project_root())
                .unwrap()
                .to_string_lossy()
                .into_owned();
            parse(&language(), &source, true, &label);
        }
    }
}

#[test]
fn incomplete_corpus_recovers_without_hanging() {
    for (dir, language) in CORPUS_GRAMMARS {
        let incomplete = corpus_dir().join(dir).join("incomplete");
        for path in lisp_sources(&incomplete, &format!("{dir}/incomplete")) {
            let source = std::fs::read_to_string(&path).unwrap();
            let label = path
                .strip_prefix(project_root())
                .unwrap()
                .to_string_lossy()
                .into_owned();
            let tree = parse(&language(), &source, false, &label);
            // Recovery must still anchor the document: the tree covers the
            // meaningful input.
            let root = tree.root_node();
            assert!(
                root.end_byte() >= source.trim_end().len(),
                "{label}: recovered tree does not span the document"
            );
        }
    }
}
