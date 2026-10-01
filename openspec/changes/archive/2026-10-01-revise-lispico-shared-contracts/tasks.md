# Tasks

Testing mode: existing-service-strict. Local schema resources, examples, documentation, and validation are complete. Producer, host, LSP, and Zed acceptance work remains in the parent and ownership changes.

- [x] 1.1 Establish consumer-visible schema contracts and implement project/catalog version 2 plus the closed installed-pack snapshot schema. Verify meaningful valid/invalid boundaries and preserve unknown metadata, library identities, and both namespace cells.
- [x] 1.2 Migrate relative-path examples and package-resource validation coherently. Verify every shipped example and snapshot resource against the selected schemas without inventing missing producer artifacts or changing language associations.
- [x] 1.3 Run bounded local wrappers and a real schema-validation smoke; update existing README, examples README, and changelog after behavior lands. Record commands and actual results, with host/LSP/Zed parity explicitly still pending upstream.
- [x] 1.4 Record schema/resource completion for producer and consumer owners; verify strict OpenSpec state, relative links, and approved contract references without prematurely completing or archiving the parent.

## Completion evidence

- `timeout 60s cargo fmt --check && timeout 10m make test && timeout 10m make build && timeout 10m make lint && timeout 60s make check-package`: exit 0 on the final integration tree. Tests: 9 library, 13 configuration, 2 corpus, and 4 query cases passed. Release WASM build and Clippy with denied warnings passed.
- `source_manifest_contracts`, `packs_layer_selection_contracts`, and `installed_pack_snapshot_contracts` exercise valid documents and invalid consumer boundaries through the actual JSON Schema validator. All three shipped project examples validate; a separate Draft7 validation smoke also passed.
- Spec, quality, and security reviews returned merge-ready with no blockers. Nonblocking notes concern duplicated test cases, description consistency, existing unrestricted live roots, and the unchanged legacy resolver installation message.
- Project/catalog schema version 2 and snapshot schema version 1 are resources only. Source fingerprint recomputation, secure filesystem reads, installed-pack capture, host contexts, llsp integration, and Zed acceptance remain upstream work. No installed store was accessed.
- Existing documentation links resolved. Archive only this prerequisite with specification updates skipped; capability deltas remain owned by the active parent.
