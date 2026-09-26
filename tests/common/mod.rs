//! Shared helpers for the extension test harness.
//!
//! The module is compiled into every test target; not every target uses every
//! helper.
#![allow(dead_code)]

use tree_sitter::{Language, Parser, Tree};

pub fn project_root() -> std::path::PathBuf {
    std::path::Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap()).to_path_buf()
}

use tree_sitter_language::LanguageFn;

pub fn commonlisp_language() -> Language {
    // The grammar entry points live in the crate's rlib so the build-script
    // parser archives link into every test binary.
    unsafe {
        Language::new(LanguageFn::from_raw(
            zed_common_lisp::testing::tree_sitter_commonlisp,
        ))
    }
}

pub fn clojure_language() -> Language {
    unsafe {
        Language::new(LanguageFn::from_raw(
            zed_common_lisp::testing::tree_sitter_clojure,
        ))
    }
}

/// Parses `source` and fails with every `ERROR`/`MISSING` node reported when
/// `expected_complete` is set. Recovery fixtures use `expected_complete =
/// false` and only require the parser to produce a tree without hanging.
pub fn parse(language: &Language, source: &str, expected_complete: bool, label: &str) -> Tree {
    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("grammar runtime ABI is compatible with the tree-sitter crate");

    let tree = parser
        .parse(source, None)
        .unwrap_or_else(|| panic!("{label}: parser returned no tree"));

    if expected_complete {
        let mut cursor = tree.walk();
        let mut issues = Vec::new();
        'walk: loop {
            let node = cursor.node();
            if node.is_error() || node.is_missing() {
                let range = node.byte_range();
                let excerpt = &source[range.start..source.len().min(range.end)];
                issues.push(format!(
                    "{} {}..{}: {:?}",
                    node.kind(),
                    node.start_position(),
                    node.end_position(),
                    excerpt
                ));
            }
            if cursor.goto_first_child() {
                continue;
            }
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    break 'walk;
                }
            }
        }
        assert!(
            issues.is_empty(),
            "{label}: unexpected incomplete parse:\n{}",
            issues.join("\n")
        );
    }

    tree
}

/// Compiles a shipped query file against `language`, reporting the file path
/// on failure. Only the query test target uses this; the shared module is
/// compiled into every test target.
#[allow(dead_code)]
pub fn compile_query(language: &Language, source: &str, label: &str) {
    if let Err(err) = tree_sitter::Query::new(language, source) {
        panic!("{label}: query does not compile against the pinned grammar: {err}");
    }
}

/// Expands snippet placeholder syntax (`${n:default}` keeps its default text,
/// `$n` and `$0` are removed) so an expanded snippet can be syntax-checked.
pub fn expand_snippet(body: &str) -> String {
    let mut out = String::with_capacity(body.len());
    let bytes = body.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        match body[i..].find('$') {
            Some(offset) if offset > 0 || bytes[i] == b'$' => {
                let at = i + offset;
                out.push_str(&body[i..at]);
                let rest = &body[at + 1..];
                if let Some(digit) = rest.chars().next().filter(|c| c.is_ascii_digit()) {
                    i = at + 1 + digit.len_utf8();
                } else if rest.starts_with('{') {
                    match rest.find('}') {
                        Some(end) => {
                            let inner = &rest[1..end];
                            match inner.split_once(':') {
                                Some((_, default)) => out.push_str(default),
                                None => {}
                            }
                            i = at + 1 + end + 1;
                        }
                        None => {
                            out.push('$');
                            i = at + 1;
                        }
                    }
                } else {
                    out.push('$');
                    i = at + 1;
                }
            }
            _ => {
                out.push_str(&body[i..]);
                break;
            }
        }
    }
    out
}

/// Matches the configuration glob semantics the design pins: `/` separator,
/// `*` within a segment, `**` across segments (including zero segments when
/// used as a whole segment).
pub fn glob_match(pattern: &str, path: &str) -> bool {
    let pattern_segments: Vec<&str> = pattern.split('/').collect();
    let path_segments: Vec<&str> = path.split('/').collect();
    match_segments(&pattern_segments, &path_segments)
}

fn match_segments(pattern: &[&str], path: &[&str]) -> bool {
    match pattern.split_first() {
        None => path.is_empty(),
        Some((&"**", rest)) => (0..=path.len()).any(|skip| match_segments(rest, &path[skip..])),
        Some((segment, rest)) => {
            let Some((head, tail)) = path.split_first() else {
                return false;
            };
            segment_match(segment, head) && match_segments(rest, tail)
        }
    }
}

fn segment_match(pattern: &str, segment: &str) -> bool {
    let pattern: Vec<char> = pattern.chars().collect();
    let segment: Vec<char> = segment.chars().collect();
    let mut states = vec![0usize];
    let mut current = 0;
    while current < segment.len() {
        let ch = segment[current];
        let mut next = Vec::new();
        for &state in &states {
            match pattern.get(state) {
                Some('*') => {
                    if !next.contains(&state) {
                        next.push(state);
                    }
                    if !next.contains(&(state + 1)) {
                        next.push(state + 1);
                    }
                }
                Some(&p) if p == ch => {
                    if !next.contains(&(state + 1)) {
                        next.push(state + 1);
                    }
                }
                _ => {}
            }
        }
        states = next;
        current += 1;
        if states.is_empty() {
            return false;
        }
    }
    states
        .iter()
        .any(|&state| matches!(pattern.get(state), None | Some('*')))
}
