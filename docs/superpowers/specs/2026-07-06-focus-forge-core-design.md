# Design: focus_forge_core (Phase 2 core slice)

Date: 2026-07-06
Status: Approved

## Goal

Scaffold `crates/focus_forge_core`, the domain foundation of Focus Forge. It owns
the domain types, validation rules, the error type, and JSON persistence. The CLI
and GUI crates will depend on it; it depends on neither. This is the Phase 2 "First
Product Slice" work from `IMPLEMENTATION.md`.

The crate is built to its end state now (serde + JSON persistence included), rather
than growing std-only first. See the Dependency Ordering Decision below.

## Dependencies

Added to root `[workspace.dependencies]` and referenced from the crate with
`.workspace = true`:

- `serde` (features = ["derive"]) — serialization
- `serde_json` — JSON persistence
- `thiserror` — the error type

Dev-dependency (test-only):

- `tempfile` — isolated file paths in storage tests

## Dependency Ordering Decision

`IMPLEMENTATION.md`'s Dependency Introduction Plan schedules `thiserror` around
Chapter 5 and `serde`/`serde_json` at Chapter 7, with standard-library-only work
through Chapter 4. Building `focus_forge_core` to its end state now introduces those
crates earlier than that schedule.

We accept this deliberately: a complete, testable, persistable core slice is worth
more than strict dependency ordering, and it makes the committed
`sample_data/demo_workspace.json` immediately real. To keep the docs from
contradicting the code:

- Add `docs/decision-records/0002-build-core-end-state-first.md` recording the choice.
- Update the Dependency Introduction Plan in `IMPLEMENTATION.md` to note that
  `focus_forge_core` introduces serde/serde_json/thiserror at Phase 2, and that the
  chapter numbers describe when the *learner is taught* the dependency, not when it
  first appears in product code.

## Module Layout

```
crates/focus_forge_core/
  Cargo.toml
  src/
    lib.rs        # public re-exports, crate-level docs
    error.rs      # CoreError (thiserror), Result alias
    project.rs    # Project, ProjectStatus
    task.rs       # Task, Priority
    note.rs       # Note
    workspace.rs  # Workspace aggregate root + operations
    storage.rs    # load_workspace / save_workspace
  tests/
    core_behavior.rs   # validation, transitions, serde round-trips
    sample_data.rs     # loads the committed sample_data/demo_workspace.json
```

The crate is student-facing, so all source follows the Teaching Comment Style
recorded in `IMPLEMENTATION.md` (rich `///` doc comments plus inline `//` notes with
C#/Java/TypeScript bridges).

## Domain Model

All types derive `Serialize` and `Deserialize`. IDs are `String` (caller supplies a
slug such as `"proj-learn-rust"`); timestamps are `String` holding ISO-8601 text.
Enums use serde `rename_all = "snake_case"` so they match the committed sample data.

```rust
struct Workspace {
    version: u32,
    name: String,
    projects: Vec<Project>,
}

struct Project {
    id: String,
    name: String,
    description: String,
    status: ProjectStatus,   // "planned" | "active" | "paused" | "done"
    created_at: String,      // ISO-8601
    updated_at: String,      // ISO-8601
    tags: Vec<String>,
    tasks: Vec<Task>,
    notes: Vec<Note>,
}

enum ProjectStatus { Planned, Active, Paused, Done }

struct Task {
    id: String,
    title: String,
    priority: Priority,      // "low" | "medium" | "high"
    done: bool,
}

enum Priority { Low, Medium, High }

struct Note {
    id: String,
    text: String,
    created_at: String,      // ISO-8601
}
```

This maps exactly onto `sample_data/demo_workspace.json`; there is no schema drift.
Fields are public for now (early-chapter simplicity); tightening to accessors can be
a later refactoring chapter.

## Public API

Validate-on-construction: fallible constructors return `Result` and reject invalid
input, so an invalid value cannot exist. Timestamps are passed in as parameters
(not generated) so tests stay deterministic and the crate stays off a time
dependency like `chrono`.

```rust
// project.rs
Project::new(id: &str, name: &str) -> Result<Project>
//   rejects blank id/name; status = Planned; description empty; tags/tasks/notes empty;
//   created_at/updated_at set to "" (a new in-memory project has no persisted time yet).
//   Sample data carries real ISO-8601 timestamps because it is authored by hand, not
//   built via Project::new; both forms deserialize into the same struct.

// task.rs
Task::new(id: &str, title: &str, priority: Priority) -> Result<Task>
//   rejects blank id/title; done = false
Task::complete(&mut self)
//   sets done = true

// note.rs
Note::new(id: &str, text: &str, created_at: &str) -> Result<Note>
//   rejects blank text

// workspace.rs
Workspace::new(name: &str) -> Workspace
Workspace::add_project(&mut self, project: Project) -> Result<()>   // rejects duplicate id
Workspace::project(&self, id: &str) -> Option<&Project>
Workspace::add_task(&mut self, project_id: &str, task: Task) -> Result<()>  // rejects unknown project
Workspace::add_note(&mut self, project_id: &str, note: Note) -> Result<()>  // rejects unknown project
Workspace::validate(&self) -> Result<()>   // re-checks invariants; used after load
```

Blank means empty after `trim()`.

## Error Type

```rust
// error.rs
pub enum CoreError {
    EmptyName,
    EmptyTitle,
    EmptyText,
    BlankId,
    DuplicateId(String),
    UnknownProject(String),
    Io(String),
    Json(String),
}
pub type Result<T> = std::result::Result<T, CoreError>;
```

`CoreError` derives `thiserror::Error` with human-readable `#[error("...")]` messages.
I/O and serde failures in `storage.rs` are mapped into `Io`/`Json` (carrying the
underlying message as a `String`) so the public API never leaks foreign error types.

## Storage

```rust
// storage.rs
load_workspace(path: &Path) -> Result<Workspace>
//   read file -> serde_json::from_str -> Workspace::validate
save_workspace(workspace: &Workspace, path: &Path) -> Result<()>
//   serde_json::to_string_pretty -> write file
```

## Testing

- Unit tests per module: construction validation (each error variant), `ProjectStatus`
  and `Priority` serde round-trips (snake_case verified), `Task::complete`.
- Save→load round-trip (build a `Workspace`, save to a `tempfile` path, load it back,
  assert equality) plus missing-file and malformed-JSON error paths. As implemented,
  these live as `#[cfg(test)]` unit tests in `src/storage.rs` (which can reach crate
  internals) rather than a separate `tests/core_behavior.rs` file.
- `tests/sample_data.rs`: load the committed `sample_data/demo_workspace.json`,
  assert it parses and `validate()`s. This promotes the sample data from
  "provisional" to a verified contract and lets us clear that flag in `STATUS.md`.

No test depends on network access or a fixed system clock.

## Out of Scope (YAGNI)

- Search / filtering (`search.rs`) — later phase.
- Import/export beyond load/save — later phase.
- Accessor methods / private fields — later refactoring chapter.
- Real timestamp generation (`chrono`) — deferred; timestamps are caller-supplied.
- Backups-before-overwrite — later persistence chapter.

## Follow-on Work After This Slice

- Workspace membership: add `crates/focus_forge_core` to root `Cargo.toml`.
- ADR `0002` + Dependency Introduction Plan update (see Dependency Ordering Decision).
- Clear the "provisional" note on `sample_data/demo_workspace.json` in `STATUS.md`
  once `tests/sample_data.rs` is green.
- Phase 2 continues with `focus_forge_cli` (separate spec) — not part of this slice.
