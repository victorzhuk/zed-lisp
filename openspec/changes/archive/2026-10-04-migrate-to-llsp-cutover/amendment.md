# Canonical predecessor amendment map

The archived change `2026-10-01-add-lispico-development-support` is immutable
history: it is not edited, re-marked, or re-archived. This map audits its
decision-5 amendment classes against the current canonical specs under
`openspec/specs/`, records the finding per class, and names where any missing
normative text would live. The finding in every class here is **already
satisfied**: the canonical baselines landed with the archive, so this audit
authors no delta, and the open parent `migrate-to-llsp` remains the sole
normative delta carrier for `llsp-language-server-integration` and
`common-lisp-language-server-integration`.

## Class 1 — server identities

| Parent decision-5 item | Canonical requirement | Current wording | Finding | Owner | Evidence |
|---|---|---|---|---|---|
| `lispico-lsp` / `sextant` / `lispico` replaced by `llsp` in all three modes | `lispico-language-server` — *Lispico mode isolation from ordinary Common Lisp* | `openspec/specs/lispico-language-server/spec.md:8-10` already routes both Lispico modes to "the shared language server with their explicit language identifiers" and assigns "registered server entries, language identifiers, binary resolution, cache handling, and settings forwarding" to `llsp-language-server-integration` | **already satisfied** — no `sextant` or `lispico-lsp` identity survives in any normative `lispico-*` text | cutover (this change); canonical text landed with the archive | `grep -n 'sextant\|lispico-lsp' openspec/specs/lispico-*` returns nothing; the shared entry is registered in `extension.toml` and dispatched by `src/common_lisp.rs` |
| The same for plain Common Lisp | `common-lisp-language-server-integration` — *Language server declaration*, *Language server binary resolution precedence* | The canonical baseline (`openspec/specs/common-lisp-language-server-integration/spec.md`) still names sextant with the Roswell fallback — and the open parent's MODIFIED delta (`openspec/changes/migrate-to-llsp/specs/common-lisp-language-server-integration/spec.md`) replaces exactly those requirements with the llsp declaration and the config → PATH → verified-download chain | **already carried** — the correction is the parent's own delta, not a second one | open parent `migrate-to-llsp` (sole carrier) | the delta's `MODIFIED` headers at `spec.md:3,17,69` and the `REMOVED` *Code label formatting* at `spec.md:78-80` |

## Class 2 — merged launch requirement

| Parent decision-5 item | Canonical requirement | Current wording | Finding | Owner | Evidence |
|---|---|---|---|---|---|
| The two independent server launch paths collapse into the new capability | `lispico-language-server` — *Lispico mode isolation from ordinary Common Lisp*; `llsp-language-server-integration` — *Single shared server entry for all three language modes*, *llsp binary resolution precedence* | The isolation requirement defers all launch ownership to `llsp-language-server-integration`; the new capability's requirements own the single entry, the three-step resolution, the cache, and forwarding | **already satisfied** — one shared path, no per-mode launch requirement remains | cutover (this change) implements it; canonical text landed with the archive | `openspec/changes/migrate-to-llsp/specs/llsp-language-server-integration/spec.md:7-44`; implementation evidence in this change's design §Identifier-map evidence and the `src/tests.rs` contract tests |

## Class 3 — contradictory "no download" / Roswell wording

| Parent decision-5 item | Canonical requirement | Current wording | Finding | Owner | Evidence |
|---|---|---|---|---|---|
| Drop the "no download" mandate and the Roswell preservation from launch text | `common-lisp-language-server-integration` — *Language server binary resolution precedence*; `lispico-language-server` — *Lispico mode isolation* | The canonical `lispico-*` launch text contains no download prohibition and no Roswell step. The sextant/Roswell chain survives only in the canonical `common-lisp-language-server-integration` baseline, whose superseding MODIFIED delta is authored in the parent — resolving the contradiction at archive time instead of duplicating a competing body now | **already satisfied** in the `lispico-*` normative text; carried by the parent's delta for the Common Lisp baseline | open parent `migrate-to-llsp` | `grep -n 'Roswell' openspec/specs/` matches only `common-lisp-language-server-integration/spec.md` (the baseline the parent's delta replaces); the landed resolver has no Roswell or source-build path (`src/tests.rs`: `make_executable`/`download` assertions, error text names three remedies) |

## Class 4 — analyzer/checker ownership

| Parent decision-5 item | Canonical requirement | Current wording | Finding | Owner | Evidence |
|---|---|---|---|---|---|
| Upstream analyzer/checker ownership moves to llsp only where llsp actually supplies it | `lispico-static-diagnostics` — *Shared editor and batch results* | `openspec/specs/lispico-static-diagnostics/spec.md:76-80` already names the llsp `check` subcommand as the batch entry point "where its semantics fit", and records the host-aware analysis llsp does not provide as "an open upstream prerequisite and an unmet part of this capability, not a reduced scope that satisfies it" | **already satisfied** — llsp owns exactly what it supplies; the host-aware gaps stay unmet external prerequisites; catalog and declaration ownership stays with go-lispico/zhk/Yagel | open parent / external llsp-side work (identity recorded as unknown in the baseline record) | the requirement's *Checker coverage narrower than the editor contract* scenario; baseline record §Host-aware parity gap and §Analysis capability gap |

## Result

- Normative gaps found in the canonical `lispico-*` text: **none** — no new
  delta is authored in this change or in the parent for the predecessor
  amendment, so `openspec/changes/migrate-to-llsp-cutover/` carries no
  `specs/` directory and no competing `MODIFIED` body exists anywhere.
- The authoritative `Custom server arguments pass-through` requirement remains
  the parent's single `MODIFIED` body
  (`openspec/changes/migrate-to-llsp/specs/common-lisp-language-server-integration/spec.md:69`).
- Every semantic, context, catalog, library, phase, project-format,
  source-evidence and data-ownership requirement keeps its intent; no
  completion mark in the archive or in the open parent was altered by this
  audit.
