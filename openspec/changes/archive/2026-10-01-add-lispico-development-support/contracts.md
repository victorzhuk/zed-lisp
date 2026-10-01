# Approved shared contracts

These contracts were approved for recording and implementation planning. Approval does not authorize upstream implementation or access to an installed user packstore. The existing version-1 resources remain shipped historical evidence until the coordinated cutover is implemented.

## Ownership and delivery

[revise-lispico-shared-contracts](../archive/2026-10-01-revise-lispico-shared-contracts/proposal.md) completed the schema and example-resource migration in zed-lisp. Runtime and host producers own their metadata. llsp owns consumption, checking, and editor behavior. Capability deltas remain owned by this parent; dependency records introduce no duplicate requirements. No second checker or language server is introduced.

Catalog and project formats cut over to schema version 2 together. Existing declaration fields, profile and library identities, and the exactly-one source-version/source-revision requirement remain. No version-1 shim or inferred conversion is accepted. A new pack-layer snapshot format has its own schema version 1; its version is independent of the project/catalog version and digest framing version.

## Declared-source fingerprints

### Fields and canonical bytes

A catalog may carry `source_files` and `source_fingerprint`, but must carry both or neither. `source_files` is a nonempty unique array of nonempty relative POSIX paths, strictly ascending by unsigned UTF-8 byte order. Its owner must include every declaration-determining input, including maintained metadata, and exclude the generated catalog itself. Entry source locations are not a substitute for this manifest.

`source_fingerprint` and configured `expected_fingerprint` use `sha256:` followed by exactly 64 lowercase hexadecimal digits. SHA-256 consumes the following byte stream:

1. ASCII `lispico-source-v1` followed by one NUL byte.
2. For each manifest path in its declared order: unsigned 64-bit little-endian UTF-8 path-byte length; path bytes; unsigned 64-bit little-endian raw-file-byte length; raw file bytes.

Do not normalize case, Unicode, line endings, or file contents. Do not sort or normalize malformed consumer input into validity. Reject absolute paths, backslashes, NUL, empty path components, `.` and `..` components, duplicate paths, and unsorted manifests.

Inclusive limits are 256 files, 1 MiB per file, and 16 MiB total file bytes. Enforce limits on the bytes actually read. Candidate metadata/source supersets measured during planning fit these limits: runtime 63 files/875552 bytes, zhk 29/1060283 bytes, Yagel 99/1170286 bytes. These measurements establish capacity, not completeness of a future producer manifest.

### Provenance and invalidation

Configured `expected_fingerprint` requires a matching catalog fingerprint even when no source root is available. A configured `source_root` requires the manifest/fingerprint pair and successful recomputation; computed, catalog, and independently configured expected digests must agree when all are present. Without a source root, label the result catalog-only provenance. A match proves equality of declared bytes, not completeness of the source manifest or cleanliness of a whole checkout.

Dirty producers record the source revision and fingerprint their actual declared bytes; they must not label modified declaration sources as an immutable release version. Both hosts currently pin go-lispico v0.14.0, so their runtime catalogs must describe that tag, not sibling checkout HEAD. Do not reuse process-local `Dialect.Fingerprint()` or computational inventory phase labels as source or execution-phase metadata.

Only explicitly configured roots may be read. Require regular files; reject symlinks in the root and every traversed component. Confinement must remain valid while reading, through anchored, handle-relative access rather than a lexical precheck followed by an unconstrained reopen. If a supported platform cannot enforce that property, fail with an actionable tool/context error rather than weakening it. Missing, unreadable, nonregular, oversized, or mismatching inputs invalidate the whole catalog and its previously loaded declarations. Never retain stale symbols or silently substitute another catalog. Unlisted paths must not be opened.

## Installed Yagel pack-layer snapshots

### Selection and capture

llsp must not open a live installed packstore. Yagel owns an explicit inert export using a caller-selected store; no ambient HOME discovery, host boot, session, macro expansion, or rule execution is permitted. New exporter entry points and flags remain implementation-plan work, not existing APIs.

Reuse the production engine's selection semantics. If the v1 active set is nonempty, capture that set and do not merge in active v2 packs. Otherwise use `CaptureActiveV2`. Do not promise stronger cross-generation atomicity than the current engine. An active-set metadata read alone is not a safe content capture.

For v1, acquire the existing active-set lease, verify objects, and hold the lease through source-byte copying. Release it on success and failure. This export requires bounded store-index writes for lease bookkeeping; that future behavior is explicitly approved as part of the design, not described as a read-only operation. For v2, `CaptureActiveV2` returns verified cloned records and self-contained snapshots (`internal/packstore/v2.go:284–312`); use those owned bytes rather than reopening store objects.

### Representation

Add `schemas/lispico-packs.schema.json` as a new closed snapshot schema. Project schema version 2 allows a `packs` layer to select exactly one of a source-directory `root` or a relative `snapshot` manifest path. A snapshot selection also requires `expected_fingerprint`, pinning the raw snapshot JSON bytes as `sha256:<lowercase hex>`. Existing embedded, global, and project layer semantics remain unchanged.

The snapshot contains its schema version, producing Yagel source revision, selection generation (`v1` or `v2`), selected pack names and content digests, and sorted final pack-layer rule entries. Each entry retains its logical rule key and originating pack identity. A readable entry references a regular relative companion source file and its raw-byte SHA-256 digest. An unreadable entry has no source payload and explicitly claims its key. Collection problems must remain distinguishable from successful complete collection. Output contains no wall-clock timestamp or absolute user-store path.

Snapshot source references resolve from the snapshot file's directory. Snapshot and companion reads use the same anchored-root, no-symlink, bounded-read rules as catalog verification. Verify the expected snapshot digest and every readable payload before publishing semantic state. Failed replacement clears prior snapshot-derived results. All payloads must exist before a manifest is published; publishing a partial or unverified bundle is forbidden.

This is a pack-layer snapshot, not an assembled global/project snapshot. For a pack-relative path `p`, `:shadows` yields logical key `p`; otherwise the key is `packs/<name>/p`. Within packs, the lexicographically later pack name wins at the same key. Preserve the actual collector's manifest validation, skipped-pack problems, unreadable-key claims, and whole-collection resource failures instead of duplicating a different winner algorithm.

The consumer overlays logical pack keys between embedded and global/project layers. An unreadable pack winner blocks lower definitions; an explicit higher-layer winner may replace it. Bound collection and payloads by the existing 256-file, 1 MiB/file, 16 MiB-total envelope. A resource breach invalidates the snapshot rather than presenting partial results as complete. Keep original logical keys separate from exported payload paths; navigation without an available source checkout falls back to verified snapshot payload locations, not fabricated original paths.

A snapshot represents the exported selected set. It makes no claim that a live installed store has not changed since export. Regenerate explicitly and reload/watch the selected snapshot and payloads. Source-directory pack fixtures alone are not evidence of installed-store parity. The existing `rules check` v1-only acquisition path is not a complete oracle for the production engine's v2 branch and is not repaired by this change.

## Declaration semantics

Preserve actual library enablement. CL aliases and adapters belong to their enabled runtime library; do not introduce a synthetic `cl-vocab` library. Core/stdlib/JSON declarations used by host contexts must not be restricted to the runtime profile. Catalog identity and consumer lookup include both name and cell: where the runtime registers one callable in both CL cells, preserve both verified entries rather than discarding value-position visibility. Unknown arity, parameter names, result types, and phase remain absent; a first argument-count guard alone does not prove a complete signature.

## Verification and permission boundaries

Testing mode remains existing-service-strict. Source-verification acceptance includes actual byte mutations, changed manifest membership, missing and unreadable files, malformed paths/order, symlink/race confinement, size boundaries, and failure after a prior successful load. Pack acceptance includes active-generation selection, protected copying, logical shadows, unreadable winners with higher-layer replacement, corrupt payloads, and failed replacement. Test consumer-visible results rather than copied hash implementations or golden source bytes.

Owner implementation requires separate approval and fresh baselines. Planning observed concurrent changes and unrelated worktrees; preserve them and Yagel's dirty Makefile. No owner task is complete merely because this contract exists. Final acceptance still requires real llsp/checker parity and actual Zed behavior after the independent shared-server migration.
