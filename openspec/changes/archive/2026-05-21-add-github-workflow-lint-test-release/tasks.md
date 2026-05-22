## 1. Add Workflow Skeleton

- [x] 1.1 Create `.github/workflows/ci.yml`
- [x] 1.2 Configure triggers for pull requests, branch pushes, and `v*` tag pushes
- [x] 1.3 Define default read-only permissions for CI jobs

## 2. Add CI Verification

- [x] 2.1 Check out the repository in the CI job
- [x] 2.2 Select stable Rust and install the `wasm32-wasip1` target
- [x] 2.3 Add `cargo fmt --check`
- [x] 2.4 Add `cargo clippy --target wasm32-wasip1 -- -D warnings`
- [x] 2.5 Add `cargo test`
- [x] 2.6 Add `cargo build --release --target wasm32-wasip1`

## 3. Add Release Publishing

- [x] 3.1 Add a release job that runs only for `v*` tag pushes after CI succeeds
- [x] 3.2 Grant `contents: write` only to the release job
- [x] 3.3 Package `extension.toml`, `languages/`, `README.md`, `LICENSE`, and the built WebAssembly artifact
- [x] 3.4 Create or update the GitHub Release for the tag
- [x] 3.5 Upload the packaged extension artifact with overwrite behavior for reruns

## 4. Verify

- [x] 4.1 Run `openspec status --change add-github-workflow-lint-test-release`
- [x] 4.2 Review `.github/workflows/ci.yml` for valid GitHub Actions syntax
- [x] 4.3 Run local commands that mirror CI where available: `cargo fmt --check`, `cargo clippy --target wasm32-wasip1 -- -D warnings`, `cargo test`, and `cargo build --release --target wasm32-wasip1`
