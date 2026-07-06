# ADR 0002: Build focus_forge_core To Its End State First

Status: Accepted
Date: 2026-07-06

## Context

The Dependency Introduction Plan schedules `thiserror` around Chapter 5 and
`serde`/`serde_json` at Chapter 7. Building `focus_forge_core` as a complete,
testable, persistable slice requires those crates at Phase 2, earlier than the
teaching schedule.

## Decision

Build the core crate to its end state now, including serde-based JSON
persistence and a `thiserror` error type. The chapter numbers in the Dependency
Introduction Plan describe when the *learner is taught* a dependency, not when
it first appears in product code.

## Consequences

Positive:

- `sample_data/demo_workspace.json` becomes loadable and is verified by a test.
- The CLI slice can be built directly on a real core.
- The domain model, validation, and persistence are demonstrated together.

Tradeoffs:

- Product code uses crates before their teaching chapter. Chapters must
  introduce serde and thiserror as concepts the learner already saw in the core.

## Alternatives Considered

Std-only core first, serde added at Chapter 7:

- Rejected for this crate because it delays a usable product slice and would
  leave the committed sample data unverifiable for several phases.
