## MODIFIED Requirements

### Requirement: CI verification

The workflow SHALL verify formatting, linting, tests, and the release WebAssembly build. It MUST use stable Rust, install the `wasm32-wasip1` target, run `cargo fmt --check`, run `cargo clippy --target wasm32-wasip1 -- -D warnings`, execute `cargo test` through a bounded project wrapper, and run `cargo build --release --target wasm32-wasip1`. Test commands SHALL have finite wall-clock limits and explicit worker caps, with the same limits in local wrappers and CI.

Verification SHALL include both Lispico dialect query corpora, existing Common Lisp regression fixtures, server dispatch/configuration tests, and resource packaging checks. Compatible upstream analyzer/protocol/catalog evidence SHALL be required before claiming complete runtime/host support. Validation MUST NOT depend on personal absolute paths or a sibling checkout.

#### Scenario: Formatting is invalid
- **WHEN** `cargo fmt --check` fails
- **THEN** the CI job fails

#### Scenario: Lint warnings are present
- **WHEN** `cargo clippy --target wasm32-wasip1 -- -D warnings` reports a warning or error
- **THEN** the CI job fails

#### Scenario: Tests fail
- **WHEN** a required bounded test command fails or reaches its time limit
- **THEN** the CI job fails without treating a timeout as a pass

#### Scenario: Query references an invalid grammar node
- **WHEN** a shipped query cannot compile against its pinned grammar
- **THEN** verification fails before packaging

#### Scenario: WebAssembly build fails
- **WHEN** `cargo build --release --target wasm32-wasip1` fails
- **THEN** the CI job fails

### Requirement: Release artifact publishing

For `v*` tag pushes, the workflow SHALL publish a GitHub Release artifact after CI succeeds. The artifact MUST include the built WebAssembly extension output, `extension.toml`, the `languages/` directory, `README.md`, and `LICENSE`, plus every new shipped snippet, configuration example/schema, and extension-owned declaration resource referenced by the package. Native server installation SHALL be documented separately until a verified release distribution contract exists.

#### Scenario: Tagged release succeeds
- **WHEN** a `v*` tag push passes CI
- **THEN** GitHub Actions creates or updates the GitHub Release for that tag and uploads a package containing all declared extension resources

#### Scenario: Referenced resource is absent
- **WHEN** an extension manifest or shipped example references a required packaged resource that is missing
- **THEN** packaging validation fails

#### Scenario: CI fails for a tag
- **WHEN** a `v*` tag push fails CI verification
- **THEN** the release publishing job does not upload an artifact
