# Proposal

## Why

The extension's user-facing documents still describe a world the migration leaves behind: a `sextant` server for Common Lisp with a Roswell build fallback, a separately supplied `lispico-lsp` binary for the two Lispico modes, two server entries, and two resolution chains. `README.md`, `CHANGELOG.md` and `examples/README.md` would then disagree with the shipped `extension.toml` about the server name, the resolution order, and the platforms a user can expect to work on.

Two other facts force a documentation change at release time, not at specification time. The limitations of the shared server are gate-backed, and each gate has one authoritative record: a reader must be able to see which limitations are currently open, which are verified behavior, and which have been superseded by compatible evidence. And the release contract itself is versioned: the documented platform list, asset names and `SHA256SUMS` entry describe one pinned release, so the prose has to follow that release rather than a measurement taken on an earlier one.

## What Changes

- Rewrite the existing `README.md` architecture and configuration sections for one shared `llsp` server entry serving `Common Lisp`, `Lispico Clojure` and `Lispico CL`: the explicit language-ID map, the resolution order (configured binary, then `PATH`, then a checksum-verified release download), and the supported platform list taken from the pinned release's published assets.
- Document the gate-backed state from each gate's authoritative current record and name the release each measurement came from. A limitation that is still open is stated with its release consequence; one whose gate is met is described as verified behavior, with the superseded limitation retained in migration history rather than asserted as current.
- Add exactly one `[Unreleased]` entry to the existing `CHANGELOG.md` describing the user-visible outcome of the migration: one server for three modes, the resolution chain, and the platforms it covers. It describes the current measured candidate and is revised in place if a gate closes before close-out.
- Keep `examples/README.md` consistent with the landed example settings, including the `files.associations` finding as an optional user mitigation while the language-ID gate is open and as nothing more than labelled history once it is met.
- Run the bounded full check suite and `openspec validate migrate-to-llsp --strict` and require both clean before any archive is requested.
- Archive in dependency order with ordinary validated change archives once every implementation, evidence and pre-archive task is complete: `migrate-to-llsp-upstream-gates`, then `migrate-to-llsp-host-feasibility`, then `migrate-to-llsp-cutover`, then `migrate-to-llsp-acceptance`; then `migrate-to-llsp` once, applying both capability deltas together with any audited predecessor amendments; then verify the resulting canonical requirements and that every evidence link resolves; and only then this change, as the close-out record.

This change changes no capability requirement, so it writes no spec delta. Both capability deltas are owned by the open parent `migrate-to-llsp`; children supply implementation and evidence. This one documents and releases what they land.

## What This Does Not Change

- No capability requirement is added, modified or removed here. `openspec/changes/migrate-to-llsp-docs-release/specs/` is intentionally absent.
- No extension code, no server entry, no resolution behavior, no example settings, and no schema. A documented value that the code does not implement is a defect in the cutover, fixed there.
- No gate is measured, claimed or cleared. Each conclusion is read from the record that owns it — G1 and G3 from upstream-gates, G2 from cutover, G4 from feasibility, G5 from acceptance — and compared by attribution; a snapshot of a superseded pin is not quoted.
- No statement that a limitation must remain open. Superseded limitations are preserved as labelled, attributed history; open ones remain explicit blockers in the unreleased documentation.
- No re-archive or edit of the archived `2026-10-01-add-lispico-development-support`. It is immutable history; what is checked is that the current canonical requirements it produced are satisfied and that the cutover's amendment audit covers every amendment class it found.
- No new changelog file, no release cut, and no version bump. The changelog gains one `[Unreleased]` entry and nothing else.
- No host-context, catalog, library or phase capability. That work is owned by a separate llsp-side change, is not implemented or approved as a reduced equivalent here, and closes this migration's gates only through external completion plus compatible local verification.