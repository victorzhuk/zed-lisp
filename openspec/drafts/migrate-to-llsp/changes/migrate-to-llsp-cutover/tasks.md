# Tasks

This child implements the parent's section 3. Everything lands as one change: the registration and the removal of the previous entries are never two steps, and no partial cutover lands in any state, released or unreleased.

Two premises are consumed, not re-derived here: the identifiers and release contract come from the baseline record taken against the release this split pins, and the digest-before-extraction route comes from the feasibility child's recorded answer. A negative feasibility answer returns the parent design for approval; this change does not proceed on a weakened digest requirement.

Obligation ids in parentheses name the manifest obligations a row discharges.

## 0. Authorization and preconditions

- [ ] 0.1 Record the separately obtained implementation authorization for this child before any implementation row below starts: its reference, who granted it, and its date. Record whether it explicitly admits an atomic unreleased development cutover while G3 and G5 remain open. If it does not, implementation does not start. *(ob-3.1)*
- [ ] 0.2 Record the active server pin — release tag and commit — that this implementation and its G1/G4 evidence were measured against, and the identifiers and release contract verified for it. *(ob-3.1)*
- [ ] 0.3 Record the current passing G1 measurement and the G4 result with its dependency approval, each with a pointer to its authoritative owner record: G1/G3 to `openspec/changes/migrate-to-llsp-upstream-gates/gates.md`, G4 to `openspec/changes/migrate-to-llsp-host-feasibility/feasibility.md`. This row records; it never states that an approval exists. It has no runtime behavior contract. *(ob-3.1)*
- [ ] 0.4 Record the unreleased boundary this change lands under: single registration plus single removal together, no release tag, no marketplace publication, no release package publication, no release-readiness claim until G1–G5 are met on compatible evidence. *(ob-3.1)*

## 1. Registration and removal — one edit

- [ ] 1.1 Register `[language_servers.llsp]` in `extension.toml` with `name = "llsp"` and `languages = ["Common Lisp", "Lispico Clojure", "Lispico CL"]`. *(ob-3.1)*
- [ ] 1.2 Declare `[language_servers.llsp.language_ids]` in `extension.toml` mapping `Common Lisp` = `lisp`, `Lispico Clojure` = `lispico-clojure`, `Lispico CL` = `lispico-cl`, using the identifiers the baseline record read from the dialects themselves. No identifier is computed or normalized in Rust. *(ob-3.1)*
- [ ] 1.3 In the same change, delete `[language_servers.sextant]` and `[language_servers.lispico]` from `extension.toml`. No shim entry, alias, fallback entry or parallel registration remains. *(ob-3.1)*
- [ ] 1.4 Reduce `dispatch_language_server` to the single `llsp` ID; an unknown ID is an error naming the one registered server. Both previous IDs are rejected as unknown. *(ob-3.1)*
- [ ] 1.5 Extend `scripts/check_package.py` to fail when a language in any `language_servers` entry has no `language_ids` entry, and to fail when any `language_servers` entry other than `llsp` is present. Prove both failures by mutating the manifest, not by observing the clean run. *(ob-3.1)*
- [ ] 1.6 Fill in this change's design "Identifier-map evidence" section: the extension build identifier, the exact declared map, both packaging mutations failing without it, the dispatch evidence rejecting both previous IDs, and the resulting G2 result. This section is the sole authoritative G2 record; if any item is missing, G2 is not met. *(ob-3.1)*
- [ ] 1.7 Re-check the release packaging list in `.github/workflows/ci.yml` against what the manifest now needs and record the result. The extension resolves its server at runtime and ships no server binary, so the member list is expected to be unchanged; the check records that conclusion. *(ob-3.1)*

## 2. Resolution chain

- [ ] 2.1 Replace both arms of `language_server_command` with one resolver: configured binary from `LspSettings::for_worktree` first, then `worktree.which("llsp")`, then the verified release download. The three steps cannot be reordered. *(ob-3.2)*
- [ ] 2.2 Implement the download step: select the asset for the current platform from the release the baseline record verified, fetch it, compute its SHA-256, compare it with the release's `SHA256SUMS` entry for that asset, and extract only on a match. *(ob-3.2)*
- [ ] 2.3 Cache the extracted binary under a version-named directory, mark it executable, and write the per-entry completion state only after the digest check, extraction and executable bit have all succeeded. A failure at any step removes what it wrote. *(ob-3.2)*
- [ ] 2.4 Prune stale version directories when a new version completes, leaving only the current version's entry. *(ob-3.2)*
- [ ] 2.5 After the configured binary and `PATH` miss, reuse a complete verified cache entry for the active version and platform without any release lookup or network access. If none exists, attempt the supported platform's verified release download. With an empty or unusable cache and an offline/unreachable release, return the documented actionable error; do not retry indefinitely and do not start another version. Digest mismatch and extraction failure remain fatal for that attempt. A cache miss alone never ends resolution. *(ob-3.2)*
- [ ] 2.6 Return one actionable error naming the supported platforms, the reason resolution stopped, and the three remedies — configure a binary path, expose `llsp` on `PATH`, or place a verified binary in the cache — for an unsupported platform, an offline or unreachable release listing, a digest mismatch and an extraction failure alike. *(ob-3.2)*
- [ ] 2.7 Keep the release selection strict: an asset that corresponds to no supported platform is an error, never a substitution of another platform's archive. *(ob-3.2)*
- [ ] 2.8 Assert the absences: no Roswell step, no source build, no installer script, no alternate-platform path. *(ob-3.2)*

## 3. Forwarding

- [ ] 3.1 Attach the configured arguments and environment to the command on the configured-binary, `PATH`, verified-download and verified-cache paths alike, unmodified. *(ob-3.3)*
- [ ] 3.2 Keep `language_server_initialization_options` returning `lsp_settings.initialization_options` and `language_server_workspace_configuration` returning `lsp_settings.settings` unchanged — no wrapping, renaming, restructuring, default injection or suppression. *(ob-3.3)*
- [ ] 3.3 Add no reopen of a buffer and no server restart to force configuration through. *(ob-3.3)*
- [ ] 3.4 Surface a configuration the server rejects as a configuration error; filter no initialization option out of the outgoing request, whatever the feasibility child's answer about `project_file` was. *(ob-3.3)*

## 4. Behavior contract tests

- [ ] 4.1 Precedence: a configured binary wins with no `PATH` lookup and no download; a `PATH` hit wins with no release lookup; a verified download is reached only when neither earlier step applies; and with an empty cache and a reachable release, a first installation reaches verified download and completes. *(ob-3.4)*
- [ ] 4.2 Forwarding per path: the exact argument list and environment reach the returned command on each of the four paths; initialization options and workspace settings come back unmodified; a rejected initialization option surfaces as a configuration error. *(ob-3.4)*
- [ ] 4.3 Digest mismatch and extraction failure: neither leaves a usable binary, a completion state, or a startable entry. *(ob-3.4)*
- [ ] 4.4 Interrupted download state: a download that stops partway leaves nothing a later attempt can start. *(ob-3.4)*
- [ ] 4.5 Non-executable cache entry: an entry whose binary is not executable is never started; online, resolution re-resolves and reaches verified download; offline, it returns the documented error. *(ob-3.4)*
- [ ] 4.6 Pruning: installing a version removes the older version directories and keeps the current one. *(ob-3.4)*
- [ ] 4.7 Verified offline reuse: offline with a complete verified current entry starts it with no release lookup and zero network calls. Offline with no usable current entry returns the documented error. *(ob-3.4)*
- [ ] 4.8 Unsupported platform and unknown server ID: the first reports the supported platform list rather than another platform's archive; the second rejects the ID and both previous server IDs. *(ob-3.4)*
- [ ] 4.9 Review every test in `src/tests.rs` for the ban on wiring-copy, length-grew and non-empty assertions: each case must fail by a real defect. A test that only proves a value was copied or a collection grew is deleted, not kept. *(ob-3.4)*
- [ ] 4.10 Extend `scripts/check_package.py`'s coverage for the manifest and run `make test`, which depends on it. *(ob-3.4)*

## 5. Removal of the previous servers

- [ ] 5.1 Delete the sextant resolution code, its release-asset mapping, its download path, its cache layout and `cached_binary_path`, and the `label_for_completion` branch that only formatted `sextant` labels. *(ob-3.5)*
- [ ] 5.2 Delete the `lispico-lsp` resolution path and the `LISPICO_SERVER_BINARY` constant it resolves. *(ob-3.5)*
- [ ] 5.3 Delete the tests over both previous servers. No compatibility path, deprecation window or conditional on an old setting remains. The ban on legacy server strings covers the live extension sources only: historical evidence and the amendment map in section 8 legitimately retain the old identities. *(ob-3.5)*

## 6. Shipped resources

- [ ] 6.1 Retarget `examples/zhk/.zed/settings.json`, `examples/yagel/.zed/settings.json` and `examples/go-lispico/.zed/settings.json` to the `llsp` server entry, preserving every display name, every `file_types` glob, and the opt-in suffix semantics: Lispico modes still claim no suffix globally, and ordinary Common Lisp associations are unchanged. *(ob-3.6)*
- [ ] 6.2 Apply the feasibility child's `project_file` answer to those templates. If llsp does not accept it, remove it from the templates; the outgoing request is filtered in neither case. *(ob-3.6)*
- [ ] 6.3 Leave `extension.toml`'s `snippets` and `grammars` untouched and confirm the Lispico snippet files and both grammar pins are unchanged. *(ob-3.6)*
- [ ] 6.4 If G1 is currently open, carry the recorded `files.associations` observation into `examples/README.md` as an optional user mitigation — not a required setting, not added to the templates, not presented as a substitute for the gate. If retention now succeeds, omit it or label it an attributed historical note. *(ob-3.6)*
- [ ] 6.5 Update `examples/README.md`'s server column and prerequisites prose to name `llsp`, the resolution order and the supported platforms. *(ob-3.6)*

## 7. Common Lisp non-regression

- [ ] 7.1 Confirm recognition, highlighting, bracket matching, indentation, outline and text objects are unchanged for `Common Lisp` through the shared entry. *(ob-3.7)*
- [ ] 7.2 Confirm the settings surface is unchanged: the same Zed settings keys configure the same three modes, and a missing or failing server leaves structural editing usable with the failure reported separately. *(ob-3.7)*

## 8. Canonical predecessor amendment

The archived change `2026-10-01-add-lispico-development-support` is immutable history. This section audits it against the current canonical specs and authors only the amendments still missing, as deltas in the open `migrate-to-llsp` parent.

- [ ] 8.1 Produce `openspec/changes/migrate-to-llsp-cutover/amendment.md`, mapping every parent decision-5 item to its canonical requirement, the current wording, the required action or the finding "already satisfied", the owner, and the verification evidence. *(ob-3.8)*
- [ ] 8.2 Audit server identities and merged launch ownership against `lispico-language-server`. Confirm there is one shared llsp path and no conflicting no-download or Roswell mandate in current normative launch text, including the canonical "Lispico mode isolation from ordinary Common Lisp" requirement. *(ob-3.8)*
- [ ] 8.3 Audit analyzer/checker ownership against `lispico-static-diagnostics`, including its "Shared editor and batch results" requirement. llsp owns only behavior it actually supplies, and host-aware gaps remain unmet external prerequisites. Catalog and declaration ownership remains go-lispico/zhk/Yagel. *(ob-3.8)*
- [ ] 8.4 Where the audit finds a normative gap, author the complete `MODIFIED` requirement with all preserved scenarios in the corresponding `openspec/changes/migrate-to-llsp/specs/<capability>/spec.md`. Where the canonical wording is already correct, write no delta and record the "already satisfied" finding in the amendment map. No semantic, context, catalog, library, phase, project-format, source-evidence or data-ownership requirement is reduced. *(ob-3.8)*
- [ ] 8.5 Amend the open parent's `proposal.md`, `design.md` and `tasks.md` to replace obsolete references to an active predecessor change and to nonexistent baselines with this canonical amendment map; the canonical Lispico baselines now exist and the predecessor is archived. Do not alter any existing task checkbox and do not reduce any semantic or evidence requirement. *(ob-3.8)*
- [ ] 8.6 Verify every preserved requirement, scenario and completion mark against the pre-amendment source and the archived history, and strict-validate the parent with any new deltas. *(ob-3.8)*
- [ ] 8.7 Confirm exactly one authoritative operation exists per capability and requirement: no `specs/` directory under `migrate-to-llsp-cutover`, no competing `MODIFIED` body, and exactly one `Custom server arguments pass-through` requirement whose authoritative text is the parent's. *(ob-3.8)*
