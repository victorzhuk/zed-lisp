# Proposal

## Why

The approved ownership plans need a reproducible source-fingerprint contract and a faithful inert installed-pack representation. The shipped version-1 resources cannot express those inputs. This bounded local prerequisite records the coordinated resource migration approved after the ownership split.

## What Changes

- Cut project and catalog schemas to version 2 without compatibility aliases or inferred conversions.
- Add the closed version-1 installed-pack snapshot schema.
- Migrate local examples and schema/package conformance coverage together; retain syntax-only behavior while upstream consumers remain unavailable.
- Follow the exact [approved contracts](../../add-lispico-development-support/contracts.md), including source manifests, bounded race-safe verification, explicit snapshot pins, and unreadable winners.

## Capabilities

No duplicate capability deltas. The active parent owns project/context and host capabilities, including the approved additions to its project-context delta. This record owns the local schemas/resources implementation and evidence only. Its skip_specs setting prevents duplicate canonical requirements.

## Dependencies

The approved parent contracts are the prerequisite, not parent completion or archive. This local change can precede owner implementation. Runtime and host producers, llsp host contexts, and final Zed acceptance consume its versioned resource contract. It does not claim to implement the upstream analyzer or pack exporter.

## Impact

Planned local scope: `schemas/lispico-project.schema.json`, `schemas/lispico-catalog.schema.json`, NEW `schemas/lispico-packs.schema.json`, examples, `tests/config.rs`, package-resource validation, and existing operator-facing documentation. All edits remain pending until this change receives an executable reviewed plan and apply authorization. Existing version-1 completed tasks remain historical evidence.
