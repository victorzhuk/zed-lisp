## Why

The project has local build, lint, and test commands but no GitHub Actions automation to verify pull requests or publish release artifacts. Adding CI and release automation makes regressions visible before merge and gives tagged releases a repeatable packaging path.

## What Changes

- Add a GitHub Actions CI workflow for pull requests and pushes that checks formatting, runs Clippy, runs tests, and builds the WebAssembly extension.
- Add a tag-triggered release job that builds the release WebAssembly target and publishes a GitHub Release artifact.
- Keep the workflow aligned with existing Rust commands and the `wasm32-wasip1` extension target.
- Do not change extension runtime behavior.

## Capabilities

### New Capabilities

- `github-actions-ci-release`: Continuous integration and release automation for the Rust-based Zed extension.

### Modified Capabilities

None.

## Impact

- Adds `.github/workflows/ci.yml`.
- Uses the existing Cargo project, `Cargo.lock`, and `wasm32-wasip1` target.
- Requires GitHub Actions permissions for release creation on tag builds.
