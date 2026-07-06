# focus_forge_core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `crates/focus_forge_core`, the domain crate that owns Focus Forge's types, validation, error type, and JSON persistence.

**Architecture:** A standalone library crate added to the Cargo workspace. Domain types live in one module each (`project`, `task`, `note`), an aggregate root (`workspace`) composes them, `storage` handles serde_json file I/O, and `error` defines the one public error type. Constructors validate and return `Result`, so invalid values cannot exist.

**Tech Stack:** Rust 2021, `serde` + `serde_json` (persistence), `thiserror` (errors), `tempfile` (test-only).

## Global Constraints

- Edition/repository come from the workspace: crate manifests use `edition.workspace = true`, `repository.workspace = true`.
- Shared deps live in root `[workspace.dependencies]`; the crate references them with `.workspace = true`.
- IDs are `String`; timestamps are `String` holding ISO-8601 text. No `chrono`/`uuid`.
- "Blank" means empty after `.trim()`.
- Enums serialize `rename_all = "snake_case"` so JSON matches `sample_data/demo_workspace.json` (`"active"`, `"low"`, etc.).
- Public API never leaks foreign error types; all fallible paths return `focus_forge_core::Result<T>` (= `Result<T, CoreError>`).
- All source is student-facing: follow the Teaching Comment Style in `IMPLEMENTATION.md` — `///` doc comments on every public item, inline `//` notes on Rust-specific lines with C#/Java/TypeScript bridges.
- Every task ends green on `cargo fmt --check`, `cargo clippy --workspace --all-targets --all-features`, and its tests.
- Commits are signed and signed-off: `git commit -S --signoff`. Stage only intended files.

## File Structure

- `Cargo.toml` (root) — add crate to `members`, add `[workspace.dependencies]`.
- `crates/focus_forge_core/Cargo.toml` — crate manifest.
- `crates/focus_forge_core/src/error.rs` — `CoreError`, `Result<T>` alias.
- `crates/focus_forge_core/src/project.rs` — `Project`, `ProjectStatus`.
- `crates/focus_forge_core/src/task.rs` — `Task`, `Priority`.
- `crates/focus_forge_core/src/note.rs` — `Note`.
- `crates/focus_forge_core/src/workspace.rs` — `Workspace` + operations.
- `crates/focus_forge_core/src/storage.rs` — `load_workspace`, `save_workspace`.
- `crates/focus_forge_core/src/lib.rs` — public re-exports, crate docs.
- `crates/focus_forge_core/tests/sample_data.rs` — loads committed sample JSON.
- `docs/decision-records/0002-build-core-end-state-first.md` — ADR.
- `IMPLEMENTATION.md`, `STATUS.md` — reconciliation updates.

---

### Task 1: Workspace scaffolding + error type

Create the crate, wire it into the workspace, and add the error type. Error comes first because every later module returns `Result<T>` = `Result<T, CoreError>`.

**Files:**
- Modify: `Cargo.toml` (root)
- Create: `crates/focus_forge_core/Cargo.toml`
- Create: `crates/focus_forge_core/src/lib.rs`
- Create: `crates/focus_forge_core/src/error.rs`

**Interfaces:**
- Produces: `CoreError` (enum, variants `EmptyName`, `EmptyTitle`, `EmptyText`, `BlankId`, `DuplicateId(String)`, `UnknownProject(String)`, `Io(String)`, `Json(String)`); `pub type Result<T> = std::result::Result<T, CoreError>`.

- [ ] **Step 1: Add crate to the workspace and declare shared deps**

Edit root `Cargo.toml` `members` to add the crate, and add a `[workspace.dependencies]` table:

```toml
[workspace]
resolver = "2"
members = [
    "crates/focus_forge_core",
    "labs/ch00_setup",
]

[workspace.package]
edition = "2021"
repository = "https://github.com/soulwax/Learn2Rust"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
tempfile = "3"
```

- [ ] **Step 2: Create the crate manifest**

`crates/focus_forge_core/Cargo.toml`:

```toml
[package]
name = "focus_forge_core"
version = "0.1.0"
edition.workspace = true
repository.workspace = true

[dependencies]
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
tempfile = { workspace = true }
```

- [ ] **Step 3: Write the error type with a failing test**

`crates/focus_forge_core/src/error.rs`:

```rust
//! The one error type the crate exposes. Callers match on `CoreError`
//! instead of juggling `std::io::Error`, `serde_json::Error`, etc.
//! (Compare a single checked-exception type in Java, or a discriminated
//! union of error shapes in TypeScript.)

use thiserror::Error;

/// Every fallible operation in this crate returns this error on failure.
///
/// `#[derive(Error)]` + `#[error("...")]` come from the `thiserror` crate;
/// they generate the `Display` and `std::error::Error` impls for us, so a
/// `CoreError` prints a readable message automatically.
#[derive(Debug, Error, PartialEq)]
pub enum CoreError {
    #[error("name must not be blank")]
    EmptyName,
    #[error("title must not be blank")]
    EmptyTitle,
    #[error("text must not be blank")]
    EmptyText,
    #[error("id must not be blank")]
    BlankId,
    #[error("duplicate project id: {0}")]
    DuplicateId(String),
    #[error("unknown project id: {0}")]
    UnknownProject(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("json error: {0}")]
    Json(String),
}

/// Crate-wide result alias so signatures read `Result<Project>` rather than
/// `std::result::Result<Project, CoreError>`. (Like a project-wide type alias.)
pub type Result<T> = std::result::Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::CoreError;

    #[test]
    fn error_messages_are_human_readable() {
        assert_eq!(CoreError::EmptyName.to_string(), "name must not be blank");
        assert_eq!(
            CoreError::DuplicateId("proj-x".to_string()).to_string(),
            "duplicate project id: proj-x"
        );
    }
}
```

- [ ] **Step 4: Create lib.rs with module declarations and re-exports**

`crates/focus_forge_core/src/lib.rs` (modules for later tasks are declared as they are added; start with `error` only):

```rust
//! `focus_forge_core` owns the Focus Forge domain: the data types, their
//! validation rules, the crate error type, and JSON persistence. It depends
//! on no CLI or GUI code — those crates depend on this one, never the reverse.

mod error;

pub use error::{CoreError, Result};
```

- [ ] **Step 5: Run test to verify it passes**

Run: `cargo test -p focus_forge_core error_messages_are_human_readable`
Expected: PASS (1 test).

- [ ] **Step 6: Verify formatting and lint**

Run: `cargo fmt --check && cargo clippy --workspace --all-targets --all-features`
Expected: no errors, no warnings.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml Cargo.lock crates/focus_forge_core/Cargo.toml \
        crates/focus_forge_core/src/lib.rs crates/focus_forge_core/src/error.rs
git commit -S --signoff -m "feat(core): scaffold focus_forge_core crate and error type"
```

---

### Task 2: Project and ProjectStatus

**Files:**
- Create: `crates/focus_forge_core/src/project.rs`
- Modify: `crates/focus_forge_core/src/lib.rs`

**Interfaces:**
- Consumes: `crate::error::{CoreError, Result}`.
- Produces:
  - `enum ProjectStatus { Planned, Active, Paused, Done }` (serde snake_case; `Default` = `Planned`).
  - `struct Project { id, name, description, status, created_at, updated_at, tags: Vec<String>, tasks: Vec<Task>, notes: Vec<Note> }` — all `pub`.
  - `Project::new(id: &str, name: &str) -> Result<Project>`.

> **Execution ordering:** `Project` composes `Task` and `Note`, so those modules must exist before `project.rs` compiles. Implement in dependency order regardless of task numbers: **Task 1 (error) → Task 3 (Task/Priority) → Task 4 (Note) → Task 2 (Project) → Task 5 (Workspace) → Task 6 (storage) → Task 7 (sample-data test) → Task 8 (docs).** Tasks are numbered by domain concept, not execution order. When implementing `project.rs`, the `lib.rs` edit in this task's Step 2 declares all of `mod note; mod project; mod task;` together, so the module references resolve.

- [ ] **Step 1: Write failing tests**

`crates/focus_forge_core/src/project.rs`:

```rust
//! A `Project` groups tasks and notes under a name and status. Compare a
//! small entity/aggregate class in an OOP app — but here validation lives in
//! the constructor and illegal states are rejected up front.

use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};
use crate::note::Note;
use crate::task::Task;

/// Lifecycle status of a project.
///
/// `rename_all = "snake_case"` makes the JSON read `"planned"`/`"active"`,
/// matching `sample_data/demo_workspace.json`. A Rust enum is like a C#/Java
/// enum, except variants can carry data (not needed here).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    /// Not started yet. `#[default]` marks the value `Default` returns.
    #[default]
    Planned,
    Active,
    Paused,
    Done,
}

/// A project and everything attached to it.
///
/// Fields are public for now (early-chapter simplicity). Prefer `Project::new`
/// to build a valid one; direct struct construction bypasses validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub tasks: Vec<Task>,
    pub notes: Vec<Note>,
}

impl Project {
    /// Creates a new, empty project after validating `id` and `name`.
    ///
    /// Returns `Err(CoreError::BlankId)` or `Err(CoreError::EmptyName)` for
    /// blank input. Timestamps start empty: a freshly created in-memory
    /// project has no persisted time yet. The `?`-friendly `Result` return is
    /// Rust's alternative to throwing an exception.
    pub fn new(id: &str, name: &str) -> Result<Project> {
        if id.trim().is_empty() {
            return Err(CoreError::BlankId);
        }
        if name.trim().is_empty() {
            return Err(CoreError::EmptyName);
        }

        Ok(Project {
            id: id.trim().to_string(),
            name: name.trim().to_string(),
            description: String::new(),
            status: ProjectStatus::Planned,
            created_at: String::new(),
            updated_at: String::new(),
            tags: Vec::new(),
            tasks: Vec::new(),
            notes: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Project, ProjectStatus};
    use crate::error::CoreError;

    #[test]
    fn new_project_starts_planned_and_empty() {
        let p = Project::new("proj-x", "Learn Rust").unwrap();
        assert_eq!(p.id, "proj-x");
        assert_eq!(p.name, "Learn Rust");
        assert_eq!(p.status, ProjectStatus::Planned);
        assert!(p.tasks.is_empty());
        assert!(p.notes.is_empty());
    }

    #[test]
    fn new_project_rejects_blank_id() {
        assert_eq!(Project::new("   ", "Name"), Err(CoreError::BlankId));
    }

    #[test]
    fn new_project_rejects_blank_name() {
        assert_eq!(Project::new("id", "  "), Err(CoreError::EmptyName));
    }

    #[test]
    fn status_serializes_snake_case() {
        let json = serde_json::to_string(&ProjectStatus::Active).unwrap();
        assert_eq!(json, "\"active\"");
    }
}
```

- [ ] **Step 2: Register the module and re-exports in lib.rs**

Add to `crates/focus_forge_core/src/lib.rs`:

```rust
mod note;
mod project;
mod task;

pub use error::{CoreError, Result};
pub use note::Note;
pub use project::{Project, ProjectStatus};
pub use task::{Priority, Task};
```

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test -p focus_forge_core project`
Expected: PASS (4 tests: the three `new_project*` plus `status_serializes_snake_case`).

- [ ] **Step 4: Verify formatting and lint**

Run: `cargo fmt --check && cargo clippy --workspace --all-targets --all-features`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/focus_forge_core/src/project.rs crates/focus_forge_core/src/lib.rs
git commit -S --signoff -m "feat(core): add Project and ProjectStatus with validation"
```

---

### Task 3: Task and Priority

Implement BEFORE Task 2 (Project composes `Task`).

**Files:**
- Create: `crates/focus_forge_core/src/task.rs`

**Interfaces:**
- Consumes: `crate::error::{CoreError, Result}`.
- Produces:
  - `enum Priority { Low, Medium, High }` (serde snake_case; `Default` = `Medium`).
  - `struct Task { id: String, title: String, priority: Priority, done: bool }` — all `pub`.
  - `Task::new(id: &str, title: &str, priority: Priority) -> Result<Task>`.
  - `Task::complete(&mut self)`.

- [ ] **Step 1: Write failing tests**

`crates/focus_forge_core/src/task.rs`:

```rust
//! A `Task` is one actionable item inside a project. It carries a priority and
//! a done flag. `Task::complete` shows Rust's `&mut self` — an explicit,
//! compiler-checked mutable borrow, unlike freely mutating a field in C#/Java.

use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};

/// How urgent a task is. Serializes as `"low"`/`"medium"`/`"high"`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

/// A single task. Prefer `Task::new`; fields are public for early chapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub priority: Priority,
    pub done: bool,
}

impl Task {
    /// Creates a task after validating `id` and `title`. Starts not done.
    pub fn new(id: &str, title: &str, priority: Priority) -> Result<Task> {
        if id.trim().is_empty() {
            return Err(CoreError::BlankId);
        }
        if title.trim().is_empty() {
            return Err(CoreError::EmptyTitle);
        }

        Ok(Task {
            id: id.trim().to_string(),
            title: title.trim().to_string(),
            priority,
            done: false,
        })
    }

    /// Marks the task done. `&mut self` = "I need a mutable borrow of this
    /// task"; the borrow checker guarantees no one else is reading it meanwhile.
    pub fn complete(&mut self) {
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::{Priority, Task};
    use crate::error::CoreError;

    #[test]
    fn new_task_starts_not_done() {
        let t = Task::new("t1", "Write tests", Priority::High).unwrap();
        assert_eq!(t.priority, Priority::High);
        assert!(!t.done);
    }

    #[test]
    fn new_task_rejects_blank_id() {
        assert_eq!(
            Task::new(" ", "title", Priority::Low),
            Err(CoreError::BlankId)
        );
    }

    #[test]
    fn new_task_rejects_blank_title() {
        assert_eq!(Task::new("t1", "  ", Priority::Low), Err(CoreError::EmptyTitle));
    }

    #[test]
    fn complete_sets_done() {
        let mut t = Task::new("t1", "x", Priority::Medium).unwrap();
        t.complete();
        assert!(t.done);
    }

    #[test]
    fn priority_serializes_snake_case() {
        assert_eq!(serde_json::to_string(&Priority::Medium).unwrap(), "\"medium\"");
    }
}
```

- [ ] **Step 2: Ensure `mod task;` and re-exports exist in lib.rs**

(Added in Task 2 Step 2; if implementing task.rs first, add `mod task;` and `pub use task::{Priority, Task};` to lib.rs now.)

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test -p focus_forge_core task`
Expected: PASS (5 tests).

- [ ] **Step 4: Verify formatting and lint**

Run: `cargo fmt --check && cargo clippy --workspace --all-targets --all-features`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/focus_forge_core/src/task.rs crates/focus_forge_core/src/lib.rs
git commit -S --signoff -m "feat(core): add Task and Priority with completion"
```

---

### Task 4: Note

Implement BEFORE Task 2 (Project composes `Note`).

**Files:**
- Create: `crates/focus_forge_core/src/note.rs`

**Interfaces:**
- Consumes: `crate::error::{CoreError, Result}`.
- Produces:
  - `struct Note { id: String, text: String, created_at: String }` — all `pub`.
  - `Note::new(id: &str, text: &str, created_at: &str) -> Result<Note>`.

- [ ] **Step 1: Write failing tests**

`crates/focus_forge_core/src/note.rs`:

```rust
//! A `Note` is free text attached to a project. `created_at` is a
//! caller-supplied ISO-8601 string (the crate deliberately avoids a time
//! dependency at this stage).

use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};

/// A note. Prefer `Note::new`; fields are public for early chapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub text: String,
    pub created_at: String,
}

impl Note {
    /// Creates a note after validating `id` and `text`. `created_at` is stored
    /// as given (empty is allowed; only blank id/text are rejected).
    pub fn new(id: &str, text: &str, created_at: &str) -> Result<Note> {
        if id.trim().is_empty() {
            return Err(CoreError::BlankId);
        }
        if text.trim().is_empty() {
            return Err(CoreError::EmptyText);
        }

        Ok(Note {
            id: id.trim().to_string(),
            text: text.trim().to_string(),
            created_at: created_at.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Note;
    use crate::error::CoreError;

    #[test]
    fn new_note_keeps_text_and_timestamp() {
        let n = Note::new("n1", "first win", "2026-07-06T09:15:00Z").unwrap();
        assert_eq!(n.text, "first win");
        assert_eq!(n.created_at, "2026-07-06T09:15:00Z");
    }

    #[test]
    fn new_note_rejects_blank_id() {
        assert_eq!(Note::new(" ", "text", ""), Err(CoreError::BlankId));
    }

    #[test]
    fn new_note_rejects_blank_text() {
        assert_eq!(Note::new("n1", "   ", ""), Err(CoreError::EmptyText));
    }
}
```

- [ ] **Step 2: Ensure `mod note;` and re-export exist in lib.rs**

(Add `mod note;` and `pub use note::Note;` if not already present.)

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test -p focus_forge_core note`
Expected: PASS (3 tests).

- [ ] **Step 4: Verify formatting and lint**

Run: `cargo fmt --check && cargo clippy --workspace --all-targets --all-features`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/focus_forge_core/src/note.rs crates/focus_forge_core/src/lib.rs
git commit -S --signoff -m "feat(core): add Note with validation"
```

---

### Task 5: Workspace aggregate root

**Files:**
- Create: `crates/focus_forge_core/src/workspace.rs`
- Modify: `crates/focus_forge_core/src/lib.rs`

**Interfaces:**
- Consumes: `Project`, `Task`, `Note`, `CoreError`, `Result`.
- Produces:
  - `struct Workspace { version: u32, name: String, projects: Vec<Project> }` — all `pub`.
  - `Workspace::new(name: &str) -> Workspace`.
  - `Workspace::add_project(&mut self, project: Project) -> Result<()>`.
  - `Workspace::project(&self, id: &str) -> Option<&Project>`.
  - `Workspace::add_task(&mut self, project_id: &str, task: Task) -> Result<()>`.
  - `Workspace::add_note(&mut self, project_id: &str, note: Note) -> Result<()>`.
  - `Workspace::validate(&self) -> Result<()>`.
  - Constant `CURRENT_VERSION: u32 = 1`.

- [ ] **Step 1: Write failing tests**

`crates/focus_forge_core/src/workspace.rs`:

```rust
//! The `Workspace` is the aggregate root: the single top-level object that
//! owns every project. Operations that add children enforce invariants
//! (unique project ids, references resolve) so the whole graph stays valid.

use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};
use crate::note::Note;
use crate::project::Project;
use crate::task::Task;

/// On-disk format version. Bump when the JSON shape changes incompatibly.
pub const CURRENT_VERSION: u32 = 1;

/// Top-level container for all Focus Forge data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    pub version: u32,
    pub name: String,
    pub projects: Vec<Project>,
}

impl Workspace {
    /// Creates an empty workspace at the current format version.
    pub fn new(name: &str) -> Workspace {
        Workspace {
            version: CURRENT_VERSION,
            name: name.to_string(),
            projects: Vec::new(),
        }
    }

    /// Adds a project, rejecting a duplicate id. `iter().any(...)` is Rust's
    /// LINQ/stream `Any` — it short-circuits on the first match.
    pub fn add_project(&mut self, project: Project) -> Result<()> {
        if self.projects.iter().any(|p| p.id == project.id) {
            return Err(CoreError::DuplicateId(project.id));
        }
        self.projects.push(project);
        Ok(())
    }

    /// Borrows the project with `id`, or `None`. Returning `Option<&Project>`
    /// (a borrow, not a clone) is the idiomatic, allocation-free lookup.
    pub fn project(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }

    /// Adds a task to an existing project, or errors if the project is unknown.
    pub fn add_task(&mut self, project_id: &str, task: Task) -> Result<()> {
        let project = self
            .projects
            .iter_mut()
            .find(|p| p.id == project_id)
            .ok_or_else(|| CoreError::UnknownProject(project_id.to_string()))?;
        project.tasks.push(task);
        Ok(())
    }

    /// Adds a note to an existing project, or errors if the project is unknown.
    pub fn add_note(&mut self, project_id: &str, note: Note) -> Result<()> {
        let project = self
            .projects
            .iter_mut()
            .find(|p| p.id == project_id)
            .ok_or_else(|| CoreError::UnknownProject(project_id.to_string()))?;
        project.notes.push(note);
        Ok(())
    }

    /// Re-checks invariants: non-blank project ids and no duplicates. Called
    /// after loading untrusted JSON from disk.
    pub fn validate(&self) -> Result<()> {
        let mut seen: Vec<&str> = Vec::new();
        for project in &self.projects {
            if project.id.trim().is_empty() {
                return Err(CoreError::BlankId);
            }
            if seen.contains(&project.id.as_str()) {
                return Err(CoreError::DuplicateId(project.id.clone()));
            }
            seen.push(&project.id);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Workspace, CURRENT_VERSION};
    use crate::error::CoreError;
    use crate::note::Note;
    use crate::project::Project;
    use crate::task::{Priority, Task};

    fn project(id: &str) -> Project {
        Project::new(id, "A project").unwrap()
    }

    #[test]
    fn new_workspace_has_current_version_and_no_projects() {
        let ws = Workspace::new("Demo");
        assert_eq!(ws.version, CURRENT_VERSION);
        assert!(ws.projects.is_empty());
    }

    #[test]
    fn add_project_then_look_it_up() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(project("p1")).unwrap();
        assert!(ws.project("p1").is_some());
        assert!(ws.project("missing").is_none());
    }

    #[test]
    fn add_duplicate_project_is_rejected() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(project("p1")).unwrap();
        assert_eq!(
            ws.add_project(project("p1")),
            Err(CoreError::DuplicateId("p1".to_string()))
        );
    }

    #[test]
    fn add_task_to_known_project() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(project("p1")).unwrap();
        let t = Task::new("t1", "do it", Priority::Low).unwrap();
        ws.add_task("p1", t).unwrap();
        assert_eq!(ws.project("p1").unwrap().tasks.len(), 1);
    }

    #[test]
    fn add_task_to_unknown_project_errors() {
        let mut ws = Workspace::new("Demo");
        let t = Task::new("t1", "do it", Priority::Low).unwrap();
        assert_eq!(
            ws.add_task("nope", t),
            Err(CoreError::UnknownProject("nope".to_string()))
        );
    }

    #[test]
    fn add_note_to_unknown_project_errors() {
        let mut ws = Workspace::new("Demo");
        let n = Note::new("n1", "hi", "").unwrap();
        assert_eq!(
            ws.add_note("nope", n),
            Err(CoreError::UnknownProject("nope".to_string()))
        );
    }

    #[test]
    fn validate_passes_for_unique_ids() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(project("p1")).unwrap();
        ws.add_project(project("p2")).unwrap();
        assert!(ws.validate().is_ok());
    }
}
```

- [ ] **Step 2: Register the module and re-export in lib.rs**

Add `mod workspace;` and `pub use workspace::{Workspace, CURRENT_VERSION};` to `crates/focus_forge_core/src/lib.rs`.

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test -p focus_forge_core workspace`
Expected: PASS (7 tests).

- [ ] **Step 4: Verify formatting and lint**

Run: `cargo fmt --check && cargo clippy --workspace --all-targets --all-features`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/focus_forge_core/src/workspace.rs crates/focus_forge_core/src/lib.rs
git commit -S --signoff -m "feat(core): add Workspace aggregate root and operations"
```

---

### Task 6: Storage (serde_json load/save)

**Files:**
- Create: `crates/focus_forge_core/src/storage.rs`
- Modify: `crates/focus_forge_core/src/lib.rs`

**Interfaces:**
- Consumes: `Workspace`, `CoreError`, `Result`.
- Produces:
  - `load_workspace(path: &Path) -> Result<Workspace>`.
  - `save_workspace(workspace: &Workspace, path: &Path) -> Result<()>`.

- [ ] **Step 1: Write failing tests**

`crates/focus_forge_core/src/storage.rs`:

```rust
//! JSON persistence. `load_workspace` reads and re-validates; `save_workspace`
//! writes pretty JSON. Foreign errors (`std::io::Error`, `serde_json::Error`)
//! are converted into `CoreError` so callers only ever see our error type.

use std::path::Path;

use crate::error::{CoreError, Result};
use crate::workspace::Workspace;

/// Loads a workspace from `path`, then re-validates it. `map_err` converts the
/// foreign error into a `CoreError` — the `?` then propagates it. This is the
/// Rust equivalent of catching a low-level exception and rethrowing a domain one.
pub fn load_workspace(path: &Path) -> Result<Workspace> {
    let text = std::fs::read_to_string(path).map_err(|e| CoreError::Io(e.to_string()))?;
    let workspace: Workspace =
        serde_json::from_str(&text).map_err(|e| CoreError::Json(e.to_string()))?;
    workspace.validate()?;
    Ok(workspace)
}

/// Saves `workspace` to `path` as pretty-printed JSON.
pub fn save_workspace(workspace: &Workspace, path: &Path) -> Result<()> {
    let text =
        serde_json::to_string_pretty(workspace).map_err(|e| CoreError::Json(e.to_string()))?;
    std::fs::write(path, text).map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{load_workspace, save_workspace};
    use crate::error::CoreError;
    use crate::project::Project;
    use crate::workspace::Workspace;

    #[test]
    fn save_then_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ws.json");

        let mut ws = Workspace::new("Round Trip");
        ws.add_project(Project::new("p1", "One").unwrap()).unwrap();

        save_workspace(&ws, &path).unwrap();
        let loaded = load_workspace(&path).unwrap();

        assert_eq!(ws, loaded);
    }

    #[test]
    fn load_missing_file_is_io_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nope.json");
        match load_workspace(&path) {
            Err(CoreError::Io(_)) => {}
            other => panic!("expected Io error, got {other:?}"),
        }
    }

    #[test]
    fn load_malformed_json_is_json_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "{ not valid json").unwrap();
        match load_workspace(&path) {
            Err(CoreError::Json(_)) => {}
            other => panic!("expected Json error, got {other:?}"),
        }
    }
}
```

- [ ] **Step 2: Register the module and re-export in lib.rs**

Add `mod storage;` and `pub use storage::{load_workspace, save_workspace};` to `crates/focus_forge_core/src/lib.rs`.

- [ ] **Step 3: Run tests to verify they pass**

Run: `cargo test -p focus_forge_core storage`
Expected: PASS (3 tests).

- [ ] **Step 4: Verify formatting and lint**

Run: `cargo fmt --check && cargo clippy --workspace --all-targets --all-features`
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/focus_forge_core/src/storage.rs crates/focus_forge_core/src/lib.rs
git commit -S --signoff -m "feat(core): add JSON load/save storage"
```

---

### Task 7: Sample-data contract test

Promotes `sample_data/demo_workspace.json` from provisional to a verified contract by loading it through the real types.

**Files:**
- Create: `crates/focus_forge_core/tests/sample_data.rs`

**Interfaces:**
- Consumes: `focus_forge_core::{load_workspace, ProjectStatus, Priority}` (public API).

- [ ] **Step 1: Write the failing test**

`crates/focus_forge_core/tests/sample_data.rs`:

```rust
//! Integration test: the committed curriculum sample data must load and
//! validate through the real domain types. This freezes the JSON format as a
//! contract — if the model and the sample ever drift, this test fails.
//!
//! `env!("CARGO_MANIFEST_DIR")` is the crate directory at compile time; we walk
//! up to the repo root to reach `sample_data/`.

use std::path::PathBuf;

use focus_forge_core::{load_workspace, Priority, ProjectStatus};

fn sample_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("sample_data")
        .join("demo_workspace.json")
}

#[test]
fn demo_workspace_loads_and_validates() {
    let ws = load_workspace(&sample_path()).expect("sample data should load and validate");
    assert_eq!(ws.name, "Focus Forge Demo Workspace");
    assert_eq!(ws.projects.len(), 2);

    let learn = ws.project("proj-learn-rust").expect("proj-learn-rust present");
    assert_eq!(learn.status, ProjectStatus::Active);
    assert_eq!(learn.tasks.len(), 3);

    let high = learn
        .tasks
        .iter()
        .find(|t| t.id == "task-run-ch00")
        .expect("task-run-ch00 present");
    assert_eq!(high.priority, Priority::High);
    assert!(high.done);
}
```

- [ ] **Step 2: Run test to verify it passes**

Run: `cargo test -p focus_forge_core --test sample_data`
Expected: PASS (1 test). If it fails on a serde field mismatch, the JSON and the model have drifted — reconcile `sample_data/demo_workspace.json` to the struct field names rather than loosening the model.

- [ ] **Step 3: Verify formatting and lint**

Run: `cargo fmt --check && cargo clippy --workspace --all-targets --all-features`
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add crates/focus_forge_core/tests/sample_data.rs
git commit -S --signoff -m "test(core): verify sample_data loads through domain types"
```

---

### Task 8: Docs reconciliation (ADR + plan + status)

Resolves the dependency-ordering tension in the docs and clears the "provisional" flag on the sample data.

**Files:**
- Create: `docs/decision-records/0002-build-core-end-state-first.md`
- Modify: `IMPLEMENTATION.md` (Dependency Introduction Plan section)
- Modify: `STATUS.md`

- [ ] **Step 1: Write the ADR**

`docs/decision-records/0002-build-core-end-state-first.md`:

```markdown
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
```

- [ ] **Step 2: Update the Dependency Introduction Plan in IMPLEMENTATION.md**

Under the `## Dependency Introduction Plan` section, add a policy line:

```markdown
- [ ] `focus_forge_core` introduces `serde`, `serde_json`, and `thiserror` at
  Phase 2 (see ADR 0002). Chapter numbers below describe when the learner is
  taught a dependency, not when it first appears in product code.
```

- [ ] **Step 3: Update STATUS.md**

Change the "provisional" sample-data note and the next-increment section to reflect that the core crate exists and the sample data is verified. Replace the sample-data provisional line with:

```markdown
- sample_data/demo_workspace.json is verified by crates/focus_forge_core/tests/sample_data.rs
```

And set the next recommended increment to:

```markdown
Next recommended increment:
- Time the Chapter 0 path (Phase 1 gate target: under ten minutes) and record it.
- Begin the focus_forge_cli slice (separate spec) that drives focus_forge_core:
  project add/list/show, task add/done, note add, workspace export/import.
```

Also update `Last verified commit` to the storage/test commit hash and add the
new verification commands.

- [ ] **Step 4: Verify the whole workspace is green**

Run: `cargo fmt --check && cargo check && cargo test && cargo clippy --workspace --all-targets --all-features`
Expected: all pass.

- [ ] **Step 5: Commit**

```bash
git add docs/decision-records/0002-build-core-end-state-first.md IMPLEMENTATION.md STATUS.md
git commit -S --signoff -m "docs: reconcile dependency plan and status with core crate"
```

---

## Final Verification

After all tasks:

```bash
cargo fmt --check
cargo check
cargo test
cargo clippy --workspace --all-targets --all-features
git status --short --ignored
```

Expected: all commands pass; only ignored files (`.env`, `.env.local`, `target/`) untracked. Then push all commits and, if desired, tag the Phase 2 checkpoint.
