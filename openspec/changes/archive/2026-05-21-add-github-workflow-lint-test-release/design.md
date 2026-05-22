## Context

The repository is a Rust-based Zed extension. Local development already has Cargo and Make targets for formatting, linting, testing, and building the `wasm32-wasip1` release artifact. There are currently no GitHub Actions workflows.

The workflow should validate normal pull request and branch changes without publishing anything, then publish only when a maintainer pushes a version tag.

## Goals / Non-Goals

**Goals:**

- Run formatting, linting, tests, and WebAssembly release build in GitHub Actions.
- Build with the `wasm32-wasip1` target used by the extension.
- Publish a GitHub Release artifact for `v*` tags.
- Keep release permissions limited to the release job.

**Non-Goals:**

- Change Rust code, language queries, or extension behavior.
- Add a separate release management tool.
- Publish to the Zed extension registry.

## Decisions

### Single Workflow File

Use `.github/workflows/ci.yml` for both CI and release automation. A single workflow keeps shared setup in one place and is enough for this small extension.

Alternative considered: separate `ci.yml` and `release.yml`. That adds more files without a current need.

### Rust Setup

Use the stable Rust toolchain available through `rustup`, then install the `wasm32-wasip1` target before linting and release builds.

Alternative considered: adding a third-party Rust setup action. Native `rustup` is sufficient for this project and avoids an extra dependency.

### Verification Commands

Run the same checks contributors can run locally:

- `cargo fmt --check`
- `cargo clippy --target wasm32-wasip1 -- -D warnings`
- `cargo test`
- `cargo build --release --target wasm32-wasip1`

This matches the existing `Makefile` behavior while making CI strict enough to fail on formatting or lint warnings.

### Release Trigger And Publishing

Publish releases only for tags matching `v*`. The release job should depend on the CI job and use `contents: write` only where release publishing needs it.

Use the GitHub CLI available on GitHub-hosted runners to create or update the release and upload the packaged extension artifact.

Alternative considered: using a third-party release action. `gh` keeps the workflow dependency surface smaller.

## Risks / Trade-offs

- GitHub-hosted runners may change preinstalled tool versions -> install/select stable Rust explicitly before running Cargo commands.
- Release upload could fail if a release for the tag already exists -> make the upload idempotent by using `gh release upload --clobber` after ensuring the release exists.
- Packaging may omit required extension files -> include `extension.toml`, `languages/`, `README.md`, `LICENSE`, and the built WebAssembly artifact in the release archive.
