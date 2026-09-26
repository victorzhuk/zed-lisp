//! Builds the pinned grammar parsers natively for the Rust test harness.
//!
//! The grammar sources are the pinned submodules under `grammars/` — the same
//! commits `extension.toml` registers with Zed. The build is skipped for wasm
//! targets: the extension ships grammar wasm separately, and the test harness
//! never runs there.

fn main() {
    let target = std::env::var("TARGET").unwrap_or_default();
    if target.starts_with("wasm32") {
        return;
    }

    for (library_name, grammar_dir) in [
        ("zed_lisp_test_commonlisp", "grammars/commonlisp"),
        ("zed_lisp_test_clojure", "grammars/clojure"),
    ] {
        let parser = format!("{grammar_dir}/src/parser.c");
        if !std::path::Path::new(&parser).exists() {
            panic!(
                "missing {parser}. The grammar submodules are pinned by extension.toml; \
                 initialize them with `git submodule update --init --recursive`."
            );
        }
        println!("cargo:rerun-if-changed={parser}");
        cc::Build::new()
            .file(&parser)
            .include(format!("{grammar_dir}/src"))
            .warnings(false)
            .compile(library_name);
    }
}
