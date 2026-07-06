# ADR 0001: Use A Cargo Workspace

Status: Accepted
Date: 2026-07-06

## Context

Learn2Rust is both a Rust course and a growing application repository. It needs to support:

- A real application that grows over time.
- Small chapter labs that are safe to break.
- Shared commands from the repository root.
- VS Code tasks that work predictably.
- Future crates for core logic, CLI, and GUI.
- A learner who may be returning to programming and needs a stable home base.

Using separate standalone projects for every chapter would make each lesson feel isolated. Putting every exercise and application feature into one crate would make early mistakes too expensive and would blur the separation between learning labs and real product code.

## Decision

Use one root Cargo workspace.

The workspace root owns orchestration and shared commands. It should not become the main application crate.

Workspace members should be added gradually:

- `labs/ch00_setup` starts the course with a tiny runnable package.
- `labs/chXX_*` packages provide focused practice areas.
- `crates/focus_forge_core` will own domain logic.
- `crates/focus_forge_cli` will own command-line interaction.
- `crates/focus_forge_gui` will own the desktop GUI.

The root commands should remain central:

```powershell
cargo check
cargo test
cargo fmt --check
cargo clippy --workspace --all-targets --all-features
```

## Consequences

Positive:

- Learners can run checks from one place.
- VS Code can understand the whole project.
- Labs can stay small while the real app grows.
- Future crates can share dependency versions through workspace configuration.
- Agents can reason about the repository from a stable root.

Tradeoffs:

- The workspace manifest must be updated as labs and crates are added.
- A broken workspace member can affect root-level commands.
- Chapter authors must keep completed lab states compiling unless using an explicit compile-fail test strategy.

## Alternatives Considered

Separate project per chapter:

- Rejected because it fragments progress and makes the course feel like unrelated exercises.

One crate for everything:

- Rejected because it mixes labs, CLI, GUI, and domain logic too early.

Branches per chapter:

- Deferred. Tags may be useful for checkpoints, but branch-heavy learning flows add Git complexity before the learner needs it.
