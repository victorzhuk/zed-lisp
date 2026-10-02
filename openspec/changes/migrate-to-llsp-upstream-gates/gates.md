# Upstream gate record — llsp migration baseline

Record owner: `migrate-to-llsp-upstream-gates`. This record is the authoritative
current state for **G1** (language-ID retention) and **G3** (host-aware parity)
only. G2 is authoritative in `../migrate-to-llsp-cutover/design.md` under
**Identifier-map evidence**; G4 is authoritative in
`../migrate-to-llsp-host-feasibility/feasibility.md` under **Gate consequence**;
G5 is authoritative in `../migrate-to-llsp-acceptance/acceptance.md` under
**Real-server proof status**. This record holds links for those three gates and
no local mutable state for them. A later measurement appends a superseding,
attributed section; historical facts are never rewritten.

Two evidence marks are used and never mixed:

- **read at `436bc84`** — a fact established by reading a source file or a
  published release artifact pinned to the commit below.
- **observed on the installed binary** — a fact established by a live protocol
  run against the pinned, provenance-verified binary.

The parent's `v0.2.0` findings are neither: they are measurements of code that
no longer ships and are preserved as history in the *Superseded baseline*
section.

## Gate ownership index

| Gate | Current state | Measured / read on | Established by | Authoritative record |
|---|---|---|---|---|
| G1 — language-ID retention across settings reload | **met** | `v0.2.1` (`436bc84`), 2026-10-02 | observed on the installed binary (run `c6b8142f`) | this record, §G1 measurement |
| G2 — wire identifier shape matches a declared dialect ID | open here — established by the cutover's own map and packaging/dispatch evidence | — | — | `../migrate-to-llsp-cutover/design.md` (Identifier-map evidence) — link only, no local state |
| G3 — host-aware context, catalog, phase parity | **unmet — blocks release** | `v0.2.1` (`436bc84`), 2026-10-02 | read at `436bc84` (capability absence) + attributed history | this record, §Host-aware parity gap and §Analysis capability gap |
| G4 — digest-before-extraction feasibility in the WASM host | open here — decided by the feasibility child | — | — | `../migrate-to-llsp-host-feasibility/feasibility.md` (Gate consequence) — link only, no local state |
| G5 — real-server proof | open here — decided by the acceptance child | — | — | `../migrate-to-llsp-acceptance/acceptance.md` (Real-server proof status) — link only, no local state |

No row above reads *met* without the observation or reading behind it; the
G2/G4/G5 rows deliberately carry no state.

## Release under measurement (provenance)

- **Release:** `v0.2.1`, published 2026-09-30T08:49:02Z, tag target commit
  `436bc84c0f520c424d6b7a1c086f38e1ce0448e8` (GitHub release metadata for
  `victorzhuk/llsp`). *(read from the published release)*
- **Binary's own report:** `llsp --version` prints verbatim `llsp 0.2.1` and no
  commit hash. A version string was not fabricated. *(observed on the
  installed binary)*
- **Provenance via verified release artifact:** the published
  `llsp-x86_64-unknown-linux-musl.tar.gz` carries `SHA256SUMS` digest
  `9225445246f8429854c2f3f1b71c26d3fb3392d29cea602b356254d013ba3581` and the
  downloaded archive verified against it. Its member binary
  `llsp-x86_64-unknown-linux-musl/llsp` has SHA-256
  `60b14c494615bd1c6f73aff52e00268444105b50e6e593ea13985f1741526e2d`, which is
  byte-identical to the installed binary used for the run. The release tag
  maps to commit `436bc84`; the binary is therefore accepted as the pinned
  build. *(read at `436bc84` from the published artifacts, verified locally)*
- **Installed binary path:** `/home/zhuk/.local/bin/llsp`.

The pin is the initial measurement of this record, not a perpetual constraint.
A superseding measurement is appended only against an approved new pin (§8
below).

## Superseded baseline — llsp `v0.2.0` (`88e3e72`), preserved as history

The parent change (`migrate-to-llsp/design.md`, *Blocking findings*) measured
`v0.2.0` on two independent binaries. Those findings are real measurements of
code this release replaces. They are superseded, not current: no row in this
record inherits them, and neither a current pass nor a current failure rewrites
them.

- **F1 (v0.2.0)** — `workspace/didChangeConfiguration` discarded the client
  language ID: `reload_documents` re-detected from path and text alone, a
  `lispico-cl` buffer lost its dialect and its reader-invalid diagnostics after
  one accepted settings change, and nothing self-healed.
- **F2 (v0.2.0)** — the derived wire identifier was not a guaranteed declared
  dialect identifier; the explicit map was identified as zed-lisp's own fix.
- **F3 (v0.2.0)** — no host-aware contract upstream (no catalog, host-profile,
  layer, or phase vocabulary).

## The v0.2.1 fix, read from source

Mark: **read at `436bc84`** — this is a source reading, not the observation;
the observation is §G1 measurement.

- `Document` now carries the client-reported identifier:
  `src/document.rs:26` declares `pub language_id: Option<String>`, and
  `src/server.rs:451-456` (`did_open`) stores it on the document instead of
  discarding it.
- The reload path feeds the retained identifier back into detection:
  `reload_documents` (`src/server.rs:549-565`) calls
  `Settings::detect(path.as_deref(), doc.language_id.as_deref(), doc.text())`
  (`src/server.rs:556`) — the `None` that produced F1 is gone.
- `Settings::detect` (`src/document.rs:228-256`) resolves dialects in the order
  `files.associations` → client language ID → modeline (`#lang` / `-*- mode:
  ... -*-`) → file extension → `default_dialect` → first dialect, so a retained
  identifier resolves a `.lisp` buffer that has no matching suffix and no
  association.
- The upstream regression test
  `tests/sync.rs:106` (`language_id_survives_configuration_reload`) covers
  `lispico-clojure` on `#(1 2)` and `lispico-cl` on `(f [x])`, asserting the
  diagnostics survive three settings reloads.

## G1 measurement — language-ID retention, observed on the installed binary

**Run identity.** Run `c6b8142f`, 2026-10-02, against the pinned installed
binary (§Release under measurement). Total run time 1.3 s (budget 180 s);
every request and stage observation completed within its 10 s window; no
stage, control, or response is missing. The harness was temporary tooling
outside this repository (stdlib-only, isolated `HOME`/`XDG_CONFIG_HOME` in a
private temp directory, no user or project configuration, no workspace roots,
built-in dialects only, `LLSP_*` environment overrides removed); no harness is
committed here. Results below are the durable record; the ephemeral transcript
was not retained in the repository.

**Method.** `initialize` with **no** language identifier (identifier goes on
`didOpen` and never on `initialize`), with initialization options
`{"files":{"associations":{},"default_dialect":"common-lisp"},"workspace":{"index":false}}`.
Three temporary `.lisp` buffers opened at version 1, no modelines, each with
its client-reported identifier on `didOpen`:

- `lispico-cl` — text `(f [x])\n(car '(1 2))\n(first [1 2])\n(def probe-me 1)\n`
- `lisp` — same text as `lispico-cl`
- `lispico-clojure` — text `#(1 2)\n(car '(1 2))\n(first [1 2])\n(def probe-me 1)\n`

Stages: didOpen; accepted `workspace/didChangeConfiguration` #1 (settings
`{"llsp":{"files":{"associations":{},"default_dialect":"lispico-clojure"},"workspace":{"index":false}}}`);
accepted change #2 (same shape, `default_dialect` `lispico-cl`); an ordinary
`textDocument/didChange` to version 2 appending one newline only (the
reader-invalid condition is preserved, so diagnostic loss can never be
confused with a syntax repair); and a restore change (`default_dialect`
`common-lisp`). At didOpen and after each stage, fresh
`textDocument/publishDiagnostics` for every URI and a fresh
`textDocument/hover` at line 2, character 1 (`first`, a builtin of all three
dialects) per URI were collected in the same run. Acceptance of each settings
change is evidenced by fresh publications for all three URIs after the
notification and the absence of the server's rejection warning
`llsp: configuration ignored:` (`src/server.rs:545`, emitted via
`window/showMessage`, `src/server.rs:430`); no `window/showMessage` of any kind
was received during the run. Dialect identity is taken from the hover response
only — the fence identifier (the dialect's first declared language id,
`src/features/assist.rs:275-277`) plus the builtin dialect name
(`builtin (<dialect name>)`, `src/features/assist.rs:307-311`) — never from the
sent identifier or the diagnostics.

**Observed results (15/15 stage results complete).**

| Stage | Buffer (sent identifier) | Reader-invalid diagnostics | Hover pair (fence id, builtin name) |
|---|---|---|---|
| didOpen | lispico-cl | 4 × `invalid-syntax` published (incl. line 0 chars 3–4 and 5–6 for `(f [x])`) | (lispico-cl, lispico-cl) |
| didOpen | lisp | 0 published | (lisp, common-lisp) |
| didOpen | lispico-clojure | 1 × `invalid-syntax` published | (lispico-clojure, lispico-clojure) |
| change #1 (default lispico-clojure) | lispico-cl | 4 × `invalid-syntax` still published | (lispico-cl, lispico-cl) |
| change #1 | lisp | 0 published | (lisp, common-lisp) |
| change #1 | lispico-clojure | 1 × `invalid-syntax` published | (lispico-clojure, lispico-clojure) |
| change #2 (default lispico-cl) | lispico-cl | 4 × `invalid-syntax` still published | (lispico-cl, lispico-cl) |
| change #2 | lisp | 0 published | (lisp, common-lisp) |
| change #2 | lispico-clojure | 1 × `invalid-syntax` published | (lispico-clojure, lispico-clojure) |
| ordinary edit (version 2, +newline) | lispico-cl | 4 × `invalid-syntax` still published | (lispico-cl, lispico-cl) |
| ordinary edit | lisp | 0 published | (lisp, common-lisp) |
| ordinary edit | lispico-clojure | 1 × `invalid-syntax` published | (lispico-clojure, lispico-clojure) |
| restore (default common-lisp) | lispico-cl | 4 × `invalid-syntax` still published | (lispico-cl, lispico-cl) |
| restore | lisp | 0 published | (lisp, common-lisp) |
| restore | lispico-clojure | 1 × `invalid-syntax` published | (lispico-clojure, lispico-clojure) |

Sample raw hover values: `` ```lispico-cl\nfirst\n```\nbuiltin (lispico-cl) ``
and `` ```lisp\nfirst\n```\nbuiltin (common-lisp) ``. Every response's fence
identifier and builtin name agreed with each other and with the same-run
didOpen pair; no response was missing, null, errored, malformed, or
self-contradictory.

**Retention criterion applied afterwards.** G1 is met when the current
measurement shows the client-reported identifier retained across the accepted
configuration reload, with the reader-invalid diagnostics continuing to be
published for that buffer, and both controls unflipped. On this measurement:
the `lispico-cl` buffer kept its dialect and kept publishing its
reader-invalid diagnostics through both accepted changes, the ordinary edit,
and the restore; both controls kept their dialects at every stage; all settings
changes were accepted. **G1 is met on `v0.2.1` (`436bc84`).** This is an
observation about this release's behavior, consistent with the §source reading
of the fix; the `v0.2.0` history above is not rewritten by it.

## Declared and derived identifiers

Mark: **read at `436bc84`** (server side) and **read from the installed
editor's source** (editor side, Zed 1.22.0).

- Declared identifiers, verbatim from the dialect files at `436bc84`:
  - `dialects/lispico-cl.toml:2` — `language_ids = ["lispico-cl"]`
  - `dialects/lispico-clojure.toml:2` — `language_ids = ["lispico-clojure"]`
  - `dialects/common-lisp.toml:52` — `language_ids = ["lisp", "commonlisp", "common-lisp"]`
  - The server matches only these exact strings
    (`Settings::detect` → `by_language_id`, `src/document.rs:242`).
- Derived identifier when a server entry declares no map, read from the editor
  source (Zed `v1.22.0`, the installed editor): `LanguageRegistry::language_id`
  (`crates/language/src/language.rs:450-454`) returns the manifest
  `language_ids` map value when present, else `language_name.lsp_id()`;
  `LanguageName::lsp_id` (`crates/language_core/src/language_name.rs:48-54`)
  lowercases the display name (the only special case being `"Plain Text"` →
  `plaintext`). The derivation therefore yields:
  - `Common Lisp` → `common lisp`
  - `Lispico Clojure` → `lispico clojure`
  - `Lispico CL` → `lispico cl`
- Per-mode verdict, as read: **none of the three derived strings is among the
  identifiers its dialect declares.** Each derived string contains a space
  where every declared identifier uses a hyphen, so all three modes fail to
  match a dialect without an explicit map. The outcome is recorded as read; the
  explicit map is required regardless, because a coincidence with a declared
  identifier would be an accident of normalization, not a guarantee.

## Identifier-map classification

This item is a **zed-lisp fix, not an upstream gate.** llsp matches only the
identifiers its dialects declare and is behaving as specified; there is no
upstream obligation behind F2. The owning change is `migrate-to-llsp-cutover`
(registration slice, tasks 1.1–1.2 there), which records G2 in its own design.
**No upstream change is required for this item.** G2's own state lives in the
cutover's record, not here.

## Host-aware parity gap

Mark: **read at `436bc84`** (upstream absence) cross-read against the parent
change's own requirements. Nothing here implements any of the enumerated work.

Requirements of the parent's model that have no llsp equivalent on this pin,
each currently unsatisfied and owed by external work:

| Parent requirement (canonical name) | Canonical home | Missing upstream surface | Owning external work |
|---|---|---|---|
| Portable deterministic project configuration / Unambiguous context selection and reload | `openspec/specs/lispico-project-context/spec.md` | no `.lispico.json` or portable project format in llsp | the llsp-side change in the parent's dependency chain — identity not yet known (recorded as unknown, not a placeholder slug) |
| Inert versioned host and library declarations / Library availability is explicit | `openspec/specs/lispico-project-context/spec.md` | no library concept | same as above |
| zhk ordered workflow visibility | `openspec/specs/lispico-project-context/spec.md` | no host-profile or workflow vocabulary | same as above; catalog data owned by zhk |
| Yagel layers and file isolation / Yagel phase and workflow context | `openspec/specs/lispico-project-context/spec.md` | no layer or phase vocabulary | same as above; catalog data owned by Yagel |
| Reproducible declared-source verification / Explicit installed-pack snapshot selection | `openspec/specs/lispico-project-context/spec.md` | no pack snapshot or provenance model | same as above |
| Scope and known-call diagnostics / Host availability diagnostics | `openspec/specs/lispico-static-diagnostics/spec.md` | no host-aware analyzer (see §Analysis capability gap) | same as above |

The llsp-side change that owns this work is implemented under its own approval
in its own repository; its identity is an execution-time evidence field and is
**not yet known** to this repository, so it is recorded as unknown here rather
than given a placeholder slug. An unidentified owner or an unreleased promise
does not clear G3.

Three kinds of open work are distinguished and never conflated:

1. **Open migration tasks** — the editable rows in
   `../migrate-to-llsp/tasks.md` (its sections 3 and 4 name the host-aware
   slices) and in this split's child changes. These are tracked and tickable in
   their own changes.
2. **Archived task evidence** — `openspec/changes/archive/2026-10-01-add-lispico-development-support/`
   is immutable history. Its completion marks are evidence of what was done
   when its change closed; they are not open tasks, and no later change ticks
   or edits them.
3. **Unsatisfied canonical requirements** — the `lispico-*` requirements in
   `openspec/specs/` listed above are requirements, not tasks. They are
   satisfied only by behavior or by a normative delta applied through the
   parent's archive; a ticked task row does not satisfy one.

Nothing in this split implements catalog, host-profile, layer, or phase
behavior, and the catalog, profile and snapshot data remain owned by
**go-lispico, zhk and Yagel** respectively.

## Release contract (pinned release)

Mark: **read from the published `v0.2.1` release**; all five archives were
downloaded and digest-verified locally for this record.

- **Published assets (verbatim):**
  `llsp-aarch64-apple-darwin.tar.gz`,
  `llsp-aarch64-unknown-linux-musl.tar.gz`,
  `llsp-x86_64-apple-darwin.tar.gz`,
  `llsp-x86_64-unknown-linux-musl.tar.gz`,
  `llsp-x86_64-pc-windows-msvc.zip`, plus the digest file `SHA256SUMS` and the
  upstream convenience script `install.sh`. The extension's resolution chain
  executes `install.sh` under no circumstance; the resolution contract forbids
  it.
- **Platform matrix:** Linux x86_64 (musl) and Linux aarch64 (musl), macOS
  x86_64 and macOS aarch64, Windows x86_64 — five archives, five platforms.
  **Platform with no published archive: Windows aarch64.** It is unsupported
  and is named as such in the extension's error text; no other platform's
  archive may be substituted for it.
- **Archive member layout:** every archive contains a single top-level
  directory `llsp-<target>/` holding the binary `llsp` (Windows: `llsp.exe`)
  plus `LICENSE`, `CHANGELOG.md`, and `README.md`. The binary member path is
  `llsp-<target>/llsp` (Windows: `llsp-x86_64-pc-windows-msvc/llsp.exe`).
- **Digest coverage:** `SHA256SUMS` carries six entries — one per distributed
  file: all five archives and `install.sh`. **Every archive is covered.** All
  five archives were verified against their entries during this record's
  preparation; all matched. An uncovered archive would be recorded here as a
  release-contract defect, not worked around.
- `SHA256SUMS` entries for the archives, verbatim:
  - `3c1ba2fe1a403a22689f8a6a85d9f852239629312b915bf6f88013683dfbb1ee` — `llsp-aarch64-apple-darwin.tar.gz`
  - `f67995616cf00439032b039def95bab6d369df3a95bddb1278d4c5ba1b7722e8` — `llsp-aarch64-unknown-linux-musl.tar.gz`
  - `e475c9aa888db572a6172fe25080bc29d6faf8b7438190ba4a2aac810490f264` — `llsp-x86_64-apple-darwin.tar.gz`
  - `7c5ea8c25fbddeb0e7ad0f37e30e0a89acecc84fa42d74686c738c8ae1c0e913` — `llsp-x86_64-pc-windows-msvc.zip`
  - `9225445246f8429854c2f3f1b71c26d3fb3392d29cea602b356254d013ba3581` — `llsp-x86_64-unknown-linux-musl.tar.gz`

## Analysis capability gap

Mark: **read at `436bc84`**.

- `unresolved_call` defaults to off:
  `src/config.rs:103` declares the field, `src/config.rs:115` sets
  `unresolved_call: Level::Off` in the default configuration.
- No lint exists for **missing libraries**, for **catalog identity**, or for
  **phase violations** — the pinned source has no library, catalog, or phase
  model at all, so no lint over those concepts exists.
- Current status: **unmet.** This gap blocks host-context parity (G3) and, on
  this `v0.2.1` measurement, blocks release. It is **not an approved
  reduction** and **is not parity with the parent's planned checker**; the
  available diagnostics (`unused-binding`, `duplicate-definition`,
  `unresolved_call` off by default) overlap the parent's contract in name only,
  because the parent's contract is about host-aware resolution the server does
  not model.
- A later closure of this gap requires attributed external completion plus
  compatible verification of the original requirements; a superseding
  measurement changes the current status only on that evidence. A code merge
  upstream that does not ship the host-context surface — catalog, provenance
  and profile requirements included — does not close it.
- Gate consequence on this measurement: implementation of the migration may
  still proceed only under the separately recorded unreleased authorization
  described in §Implementation admission; the documented behavior must report
  the reduced available capability rather than present ordinary Common Lisp
  results as host-aware results.

## Upstream guarantee statement

Exactly what upstream must guarantee for G1 to hold on any release: **a
settings-driven re-detect preserves each open document's client-reported
language ID.** Concretely: when `workspace/didChangeConfiguration` causes open
documents to be re-detected, the dialect resolution for each document must
consider the identifier the client reported on `textDocument/didOpen`, for as
long as the document stays open. No `llsp` code is patched from this
repository to obtain this guarantee; this record states the obligation and the
`v0.2.1` release satisfies it as observed above.

## Proposed parent amendment (specification only — not performed here)

This section specifies the amendments the open parent change
(`../migrate-to-llsp/`) needs so no document can drift into being the sole
source of a gate state. It **records the required amendments; it does not
perform them.** The parent edits are outside this record's one-file write
scope and land under their own approval (they are specified here, and nothing
in this child's delivery performs them).

1. **Gate table pointers.** The parent's *Gates before cutover* table
   (`design.md`) must point each gate at its authoritative record: G1 and G3
   to this file, G2 to `migrate-to-llsp-cutover/design.md` (Identifier-map
   evidence), G4 to `migrate-to-llsp-host-feasibility/feasibility.md` (Gate
   consequence), G5 to `migrate-to-llsp-acceptance/acceptance.md` (Real-server
   proof status). The parent holds no second mutable copy of any gate state;
   its `v0.2.0` findings in *Blocking findings* are labelled superseded
   history with this record as the current authority.
2. **Authorization.** The parent's *Authorization* section must name a
   separate authorization admitting an atomic unreleased development cutover
   while G3 and G5 remain open, and must state that specification preparation
   alone authorizes no implementation.
3. **Gates before cutover.** The parent's gate section must distinguish
   **implementation admission** (current passing G1 on the active pin;
   verified dialect identifiers and release contract; G4 met with any required
   dependency approval; plus the separately recorded authorization) from
   **release admission** (G1–G5 met on compatible evidence; no release tag,
   marketplace publication, release package publication or release-readiness
   claim before that).
4. **Sole delta carrier and archive ordering.** The parent's archive decision
   must record that the open parent is the sole normative delta carrier for
   `llsp-language-server-integration` and
   `common-lisp-language-server-integration` (and any audited canonical
   predecessor amendments); that the four no-delta children own no capability
   delta; and that archive ordering is by change — `migrate-to-llsp-upstream-gates`,
   `migrate-to-llsp-host-feasibility`, `migrate-to-llsp-cutover`,
   `migrate-to-llsp-acceptance`, then the parent once, then
   `migrate-to-llsp-docs-release` last — with no archive command naming a
   capability.
5. **Parent task-section pointer.** `openspec/changes/migrate-to-llsp/tasks.md`
   must link from its task section to this child
   (`migrate-to-llsp-upstream-gates`) and to this authoritative `gates.md`
   record, so every task row that consumes a gate state reads it from the
   owner rather than restating it.

## Integration evidence exposed to the parent-wide checks

- Release and commit every row here was measured on: **`v0.2.1` /
  `436bc84c0f520c424d6b7a1c086f38e1ce0448e8`** (binary provenance via the
  verified `llsp-x86_64-unknown-linux-musl.tar.gz` artifact; see §Release under
  measurement).
- Outcome-neutral measurement pointer for the language-ID gate: §G1
  measurement in this file, run `c6b8142f`, 2026-10-02 — a complete 15-result
  same-run transcript with both controls; G1 decided from the retention
  criterion as **met**.
- Identifier-map classification: repository-side fix (§Identifier-map
  classification), owned by `migrate-to-llsp-cutover`; no upstream change
  required.
- Enumeration of still-open work and its owning change: §Host-aware parity
  gap; owning llsp-side change identity **not yet known** (recorded as
  unknown).
- Analysis gap recorded as a blocker, not a reduction: §Analysis capability
  gap.
- Integration checks resolve gate states through the authority map in
  §Gate ownership index and reject a stale or mismatched snapshot: any snapshot
  not attributed to the current active pin is stale.

## Implementation admission — requirements this record does not grant

This record states requirements; it asserts no approval and grants none.

- The cutover child's own record must cite a **separately recorded
  authorization** that explicitly admits an atomic unreleased development
  cutover while G3 and G5 are open, together with:
  - the **current passing G1 measurement** on the active pin (this record,
    §G1 measurement, for `v0.2.1`);
  - the **verified dialect identifiers** (§Declared and derived identifiers)
    and the **verified release contract** (§Release contract);
  - **G4 met** with any required dependency approval — read from the
    feasibility child's own record, never from here.
- Where such an authorization does not explicitly admit the boundary,
  implementation remains blocked. Specification preparation alone authorizes
  nothing.
- Release admission is a different boundary: no release tag, marketplace
  publication, release package publication or release-readiness claim before
  G1–G5 are met on compatible evidence.

## External completion and superseding measurement (recorded absences)

- **llsp-side change identity (task 8.1):** the actual llsp-side change that
  owes host-aware behavior has not been identified to this repository; its
  identity, upstream completion evidence, released tag and commit, artifact
  provenance, requirement mapping and catalog/profile revisions are **not
  recorded here because they do not exist yet.** An unidentified owner or an
  unreleased promise does not clear G3.
- **Separate new-pin approval (task 8.2):** no approval to replace the active
  server pin exists. Its absence is recorded precisely: no such approval has
  been granted, so no superseding pin is adopted and no unreleased build or
  branch is used.
- **Superseding baseline measurement (tasks 8.3–8.4):** not triggered — there
  is no approved new pin to measure. When one exists, this record appends a
  superseding attributed section (re-running the G1 measurement, the
  identifier reads, the release assets, digests and layout, the supported
  platforms, the configuration surface, and the complete G3 requirement
  mapping), and the feasibility child's host-route proof is re-exercised
  against the new archive — remeasured where the archive layout, host route or
  dependencies changed, with an applicability rationale where they did not.
  The `v0.2.1` findings above are preserved byte-identically as history when
  that happens.

## Baseline completion boundary

This child completes on this baseline record and the measurements attributed to
it; it does not wait on a downstream child's evidence. The open conditional
items are explicit: the llsp-side change identity (unknown), the new-pin
approval (absent), and G2/G4/G5 (owned by their own records). Any contradiction
between a later measurement and an existing row, or a missing host prerequisite,
keeps G3 open and is escalated for remeasurement rather than edited into
another child's record. Downstream evidence — cutover selection, acceptance
and documentation all naming the one active pin, and the remeasurement a
changed pin forces in their records — is a later integration and remeasurement
obligation carried by the parent's integration checks, not a precondition of
this baseline. Completion of this record never implies gate closure beyond
what its rows carry: G1 met and G3 unmet on `v0.2.1` are the only gate states
this record holds.
