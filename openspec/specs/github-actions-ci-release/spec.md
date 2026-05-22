## ADDED Requirements

### Requirement: GitHub Actions workflow definition

The repository SHALL define a GitHub Actions workflow at `.github/workflows/ci.yml` that runs for pull requests, pushes to the main branch, and version tag pushes matching `v*`.

#### Scenario: Pull request opens
- **WHEN** a pull request targets the repository
- **THEN** GitHub Actions runs the CI verification job without publishing a release

#### Scenario: Version tag is pushed
- **WHEN** a tag matching `v*` is pushed
- **THEN** GitHub Actions runs CI verification and then runs release publishing if CI succeeds

### Requirement: CI verification

The workflow SHALL verify formatting, linting, tests, and the release WebAssembly build. The verification MUST use stable Rust, install the `wasm32-wasip1` target, run `cargo fmt --check`, run `cargo clippy --target wasm32-wasip1 -- -D warnings`, run `cargo test`, and run `cargo build --release --target wasm32-wasip1`.

#### Scenario: Formatting is invalid
- **WHEN** `cargo fmt --check` fails
- **THEN** the CI job fails

#### Scenario: Lint warnings are present
- **WHEN** `cargo clippy --target wasm32-wasip1 -- -D warnings` reports a warning or error
- **THEN** the CI job fails

#### Scenario: Tests fail
- **WHEN** `cargo test` fails
- **THEN** the CI job fails

#### Scenario: WebAssembly build fails
- **WHEN** `cargo build --release --target wasm32-wasip1` fails
- **THEN** the CI job fails

### Requirement: Release artifact publishing

For `v*` tag pushes, the workflow SHALL publish a GitHub Release artifact after CI succeeds. The artifact MUST include the built WebAssembly extension output, `extension.toml`, the `languages/` directory, `README.md`, and `LICENSE`.

#### Scenario: Tagged release succeeds
- **WHEN** a `v*` tag push passes CI
- **THEN** GitHub Actions creates or updates the GitHub Release for that tag and uploads the packaged extension artifact

#### Scenario: CI fails for a tag
- **WHEN** a `v*` tag push fails CI verification
- **THEN** the release publishing job does not upload an artifact

### Requirement: Scoped release permissions

The workflow SHALL grant write access to repository contents only for the release publishing job. CI-only jobs MUST run without release publishing permissions.

#### Scenario: Main branch push runs CI
- **WHEN** a non-tag push to the main branch runs the workflow
- **THEN** the workflow does not request release publishing permissions for the CI job
