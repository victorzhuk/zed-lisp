# Proposal

## Why

The archived change `add-lispico-development-support` (archived 2026-10-01) assumed a language server and static checker that were to be built inside go-lispico (`lispico-lsp` / `lispico-check`). The dialect problem they were to solve is now solved by an existing published server: **llsp** (`github.com/victorzhuk/llsp`, Apache-2.0, `v0.2.0`), which ships the `common-lisp`, `lispico-clojure`, and `lispico-cl` dialects and publishes verified release archives plus `SHA256SUMS`.

Keeping the go-lispico server plan means two analyzers for the same dialects, with no reconciliation owner. The extension meanwhile still registers two independent servers (`sextant` for `Common Lisp`, `lispico` for the two opt-in modes) and one of them has no shipped binary at all.

llsp is not yet a drop-in replacement, and this proposal does not pretend otherwise. One reproduced upstream gap blocks a full cutover, and one more integration requirement must be met on the extension side:

1. **Dialect identity is lost on settings change (upstream defect).** `workspace/didChangeConfiguration` re-detects every open document from path and text alone, discarding the client's language ID. A `.lisp` buffer whose dialect came from the language ID alone silently becomes `common-lisp`, and real diagnostics disappear (reproduced on llsp `v0.2.0`, `88e3e72`; `lispico-cl` reader-invalid errors vanish on the first accepted settings change). Not self-healing; a `files.associations` entry or a close/reopen can restore it. This is the one item that cannot be closed from this repository.
2. **The derived wire identifier is not guaranteed to be a declared dialect ID (extension-side fix).** Zed's fallback identifier is a normalized form of the display name, not the display name itself, while the server matches only the exact identifiers each dialect declares (`lisp`, `lispico-clojure`, `lispico-cl`). Whether a derived value coincides with a declared identifier is an accident of normalization. The fix belongs here: an explicit `[language_servers.llsp.language_ids]` map for all three modes. No upstream change is required for this item.

A third, separate gap is host-aware parity: llsp has no catalog, host-profile, layer, or phase vocabulary, so the parent's context, catalog, library, and phase contracts stay open. That is a capability gap, not a defect to be fixed unilaterally, and it is gated rather than worked around.

This change therefore **prepares** the migration: one shared server for all three languages, a real resolution chain including a checksum-verified release download, and cutover held behind named gates.

## What Changes

- Replace the two registered servers with a single `llsp` language server registration for all three languages (`Common Lisp`, `Lispico Clojure`, `Lispico CL`) plus an explicit language-ID map, so all three modes share one server entry and one resolution chain and reach llsp with the correct dialect identifier.
- Retarget the `common-lisp-language-server-integration` resolution contract from sextant to llsp: configured binary → `PATH` → verified release download, with no Roswell build, no source build, and no `install.sh` execution. Remove the sextant-specific code-label formatting contract, which the shared server does not use.
- Introduce the new capability `llsp-language-server-integration`, which owns everything that is genuinely new: single-server routing across the three dialects, config/`PATH`/checksum-verified download, cache integrity, offline verified-cache reuse, settings forwarding, launch lifecycle, and the host-aware release gates.
- Amend the predecessor `add-lispico-development-support` change only where it names a server identity, analysis owner, or launch path that no longer exists: `llsp` replaces `sextant` and `lispico-lsp` in all three modes, the two server launch paths merge into the new capability, contradictory "no download" / "preserve Roswell" wording is dropped, and upstream analyzer/checker ownership moves to llsp where llsp actually provides it. The predecessor is now archived and the canonical baselines carry the amended wording; the cutover's [amendment map](../migrate-to-llsp-cutover/amendment.md) is the audit of record. Catalogs stay owned by go-lispico/zhk/Yagel, and every semantic, context, catalog, library, phase, project-format, and evidence requirement keeps its intent and its existing completion marks.
- Prove the migration against a real llsp binary: behavior contract tests for precedence, errors, cache integrity, and dialect transitions; a real stdio LSP smoke test; and actual Zed acceptance covering unsaved buffers, Unicode ranges, reload, restart, and worktree isolation. No stubs.
- Land bounded feasibility proof **before** any dependency, tooling, or code decision: computing SHA-256 over a release archive and unpacking it inside the WASM extension host is unproven, and the extension API provides no digest or extraction helper. Standard vetted Rust hashing and extraction crates are the expected route; adding a dependency still requires the repository's normal reviewed approval, and no specific crate is mandated here. If it cannot be done, the design must come back for approval rather than weaken the contract.

## Capabilities

### New Capabilities

- `llsp-language-server-integration`: shared single-server routing for the three dialects, binary resolution with checksum-verified release download, cache integrity and offline reuse, settings forwarding, and the host-aware gates that release the cutover.

### Modified Capabilities

- `common-lisp-language-server-integration`: language declaration retargeted from sextant to llsp with an explicit language-ID map; resolution precedence retargeted to config → `PATH` → verified download with the Roswell build step removed; entrypoint and settings pass-through requirements retained; sextant-only code-label formatting removed.

No deltas are written for `lispico-language-server`, `lispico-project-context`, `lispico-static-diagnostics`, or `lispico-language-support`: their canonical baselines now exist in `openspec/specs/` and already carry the llsp-aware wording this migration needs — the cutover's [amendment map](../migrate-to-llsp-cutover/amendment.md) audits all four amendment classes against them and finds no normative gap, so a duplicate delta would only collide at archive.

## Impact

- **zed-lisp:** `extension.toml`, `src/common_lisp.rs`, `examples/*/.zed/settings.json`, `examples/README.md`, `Makefile`, `scripts/check_package.py`, `.github/workflows/ci.yml`, `README.md`, and `CHANGELOG.md` under `[Unreleased]` during implementation.
- **llsp (upstream, blocked gates):** the settings-change language-ID retention contract needs an upstream guarantee before the cutover can ship. This change records it as a gate; it does not patch that repository and does not propose to work around it by degrading to suffix- or default-dialect selection. The identifier-map requirement is zed-lisp's own work and is not an upstream item.
- **go-lispico, zhk, Yagel:** catalog and declaration ownership is unchanged. The upstream analyzer/checker tasks in the parent change are retargeted only where llsp supplies the same behavior; everything host-aware stays open.
- **Coverage is not reduced by approval.** llsp does not provide the missing-library, catalog-identity, or phase-violation analysis the parent contract requires, and nothing here accepts that as a reduced outcome; it is recorded as the open gap that blocks host-context parity.
- **Implementation is not approved by this document.** Approval covers specification preparation. The separate authorization that admits the atomic unreleased development cutover is recorded in [`migrate-to-llsp-cutover/design.md`](../migrate-to-llsp-cutover/design.md) (§Authorization record); release admission stays separate and waits for the gates on compatible evidence.

See [design](design.md) for decisions and gate ordering, and [tasks](tasks.md) for the ordered work.
