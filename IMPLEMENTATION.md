# Implementation TODO: Technical Realisation

This document is the technical companion to `MASTERPLAN.md`. The masterplan describes the learning vision; this file describes how to turn the repository into the actual Rust learning environment.

The target is one GitHub repository that opens cleanly in VS Code, runs with Cargo from the root, teaches Rust through chapter labs, and grows one real application: **Focus Forge**.

## Implementation Principles

- [ ] Keep the repository runnable after every increment.
- [ ] Prefer one Cargo workspace over disconnected chapter projects.
- [ ] Keep the root folder as orchestration, documentation, and tooling.
- [ ] Keep real application code in `crates/`.
- [ ] Keep chapter experiments in `labs/`.
- [ ] Keep assignment text in `assignments/`.
- [ ] Keep longer reference material in `docs/`.
- [ ] Add infrastructure only when it supports the learner directly.
- [ ] Commit and push each completed increment with a signed Git commit using the global Git identity.

## Target Repository Structure

```text
LearnRust/
  .github/
    workflows/
      ci.yml

  .vscode/
    extensions.json
    settings.json
    tasks.json
    launch.json

  assignments/
    ch00-setup.md
    ch01-basics.md
    ch02-ownership.md

  chapters/
    00-setup.md
    01-basics.md
    02-ownership.md

  crates/
    focus_forge_core/
      Cargo.toml
      src/
        lib.rs
        project.rs
        task.rs
        note.rs
        workspace.rs
        error.rs
        storage.rs
        search.rs
      tests/

    focus_forge_cli/
      Cargo.toml
      src/
        main.rs
        commands.rs
        output.rs

    focus_forge_gui/
      Cargo.toml
      src/
        main.rs
        app.rs
        views/

  docs/
    compiler-errors.md
    dependencies.md
    getting-unstuck.md
    glossary.md
    math-track.md
    oop-to-rust.md
    platform-notes.md
    testing-assignments.md
    vscode-workflow.md

  examples/
    ownership_flow.rs
    result_flow.rs

  labs/
    ch00_setup/
    ch01_basics/
    ch02_ownership/

  sample_data/
    demo_workspace.json

  .gitignore
  Cargo.toml
  Cargo.lock
  IMPLEMENTATION.md
  MASTERPLAN.md
  README.md
  rust-toolchain.toml
```

## Files To Create First

- [x] Add `.gitignore`.
- [x] Add `IMPLEMENTATION.md`.
- [ ] Update `README.md`.
- [ ] Add `rust-toolchain.toml`.
- [ ] Add root `Cargo.toml`.
- [ ] Add `.vscode/extensions.json`.
- [ ] Add `.vscode/settings.json`.
- [ ] Add `.vscode/tasks.json`.
- [ ] Add `.vscode/launch.json`.
- [ ] Add `docs/getting-unstuck.md`.
- [ ] Add `docs/compiler-errors.md`.
- [ ] Add `labs/ch00_setup`.
- [ ] Add `chapters/00-setup.md`.
- [ ] Add `assignments/ch00-setup.md`.

## `.gitignore` Policy

- [x] Ignore Rust build output: `target/`.
- [x] Ignore local env files: `.env`, `.env.*`, except `.env.example`.
- [x] Ignore logs and temp files.
- [x] Ignore local learner/app data.
- [x] Ignore generated release artifacts.
- [x] Ignore OS/editor noise.
- [ ] Keep `Cargo.lock` committed because this repository builds runnable apps and a learning workspace.
- [ ] Keep `.vscode/` committed because the VS Code environment is part of the learning product.
- [ ] Keep `sample_data/` committed because it is curriculum material.
- [ ] Review `.gitignore` whenever persistence, packaging, or generated assets are added.

## Workspace Architecture

The root `Cargo.toml` should be a workspace manifest:

```toml
[workspace]
resolver = "2"
members = [
  "crates/focus_forge_core",
  "crates/focus_forge_cli",
  "crates/focus_forge_gui",
  "labs/ch00_setup",
  "labs/ch01_basics",
  "labs/ch02_ownership",
]

[workspace.package]
edition = "2021"
license = "MIT OR Apache-2.0"
repository = "https://github.com/soulwax/Learn2Rust"

[workspace.dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
```

TODO:

- [ ] Start with only the members that exist.
- [ ] Add lab members gradually.
- [ ] Move shared dependency versions into `[workspace.dependencies]`.
- [ ] Keep root package-free unless there is a clear need.
- [ ] Make `cargo check`, `cargo test`, and `cargo fmt` work from the root.

## Crate Architecture

Dependency direction:

```text
focus_forge_cli ---> focus_forge_core
focus_forge_gui ---> focus_forge_core

labs/* may depend on focus_forge_core only when the chapter needs it.
focus_forge_core must not depend on CLI or GUI crates.
```

### `focus_forge_core`

Purpose:

- [ ] Own all domain types.
- [ ] Own validation rules.
- [ ] Own error types.
- [ ] Own search and filtering logic.
- [ ] Own persistence format.
- [ ] Own import/export behavior.
- [ ] Stay independent of CLI and GUI concerns.

Initial modules:

- [ ] `project.rs`: `Project`, `ProjectStatus`, project operations.
- [ ] `task.rs`: `Task`, `Priority`, task lifecycle.
- [ ] `note.rs`: `Note`, note validation.
- [ ] `workspace.rs`: top-level collection and operations.
- [ ] `error.rs`: domain and persistence errors.
- [ ] `storage.rs`: load/save abstractions and JSON file storage.
- [ ] `search.rs`: search query and ranking.

Testing:

- [ ] Unit tests for validation.
- [ ] Unit tests for status transitions.
- [ ] Unit tests for task completion.
- [ ] Round-trip JSON tests.
- [ ] Search behavior tests.
- [ ] Tests should not depend on live network access.

### `focus_forge_cli`

Purpose:

- [ ] Provide fast feedback before the GUI exists.
- [ ] Exercise real app behavior through commands.
- [ ] Keep command parsing and terminal output out of the core crate.

Likely commands:

- [ ] `project add <name>`.
- [ ] `project list`.
- [ ] `project show <id>`.
- [ ] `task add <project-id> <title>`.
- [ ] `task done <task-id>`.
- [ ] `note add <project-id> <text>`.
- [ ] `workspace export <path>`.
- [ ] `workspace import <path>`.

Testing:

- [ ] Use `assert_cmd` for command behavior.
- [ ] Use `predicates` for output.
- [ ] Use `tempfile` for isolated workspace files.
- [ ] Keep CLI tests focused on user-visible behavior.

### `focus_forge_gui`

Purpose:

- [ ] Provide the visual payoff of the course.
- [ ] Use `eframe` / `egui`.
- [ ] Keep UI state separate from domain logic.
- [ ] Use `focus_forge_core` for all real operations.

Initial UI:

- [ ] Project list panel.
- [ ] Project detail panel.
- [ ] Task list.
- [ ] Note list.
- [ ] New project input.
- [ ] Save/load feedback.

Later UI:

- [ ] Search box.
- [ ] Status and tag filters.
- [ ] Settings panel.
- [ ] Import/export actions.
- [ ] Today view.
- [ ] Progress dashboard.

Manual verification:

- [ ] Window opens.
- [ ] Sample data displays.
- [ ] Project can be created.
- [ ] Task can be completed.
- [ ] Data survives restart.
- [ ] Validation errors show without crashing.

## Labs Architecture

Labs are safe, focused practice areas.

- [ ] Each lab is a tiny Cargo package.
- [ ] Each lab has one chapter concept as its center.
- [ ] Labs may contain deliberately broken examples when the assignment asks for compiler-error practice.
- [ ] Labs should compile in their completed state.
- [ ] Compile-fail examples should use comments, separate files, or `trybuild` later so the workspace still passes.

Naming:

- [ ] Folder: `labs/ch02_ownership`.
- [ ] Package: `ch02_ownership` or `lab_ch02_ownership` if package-name collisions appear.
- [ ] Assignment: `assignments/ch02-ownership.md`.
- [ ] Chapter guide: `chapters/02-ownership.md`.

## Assignment Architecture

Every assignment should include:

- [ ] Goal.
- [ ] Starting checkpoint.
- [ ] Files to edit.
- [ ] Files to read.
- [ ] Quick win.
- [ ] Guided steps.
- [ ] Main build task.
- [ ] How to verify.
- [ ] Common compiler errors.
- [ ] Recovery path.
- [ ] Stretch tasks.
- [ ] Reflection prompts.

Verification formats:

- [ ] `cargo check -p ch00_setup`.
- [ ] `cargo test -p ch01_basics`.
- [ ] `cargo test -p focus_forge_core`.
- [ ] `cargo run -p focus_forge_cli -- project list`.
- [ ] Manual GUI checklist.

## VS Code Architecture

`.vscode/extensions.json`:

- [ ] Recommend `rust-lang.rust-analyzer`.
- [ ] Recommend `vadimcn.vscode-lldb`.
- [ ] Recommend `tamasfe.even-better-toml`.

`.vscode/settings.json`:

- [ ] Enable rust-analyzer check on save.
- [ ] Use `cargo check` by default.
- [ ] Consider format on save for Rust.
- [ ] Keep settings minimal and non-invasive.

`.vscode/tasks.json`:

- [ ] `cargo: check workspace`.
- [ ] `cargo: test workspace`.
- [ ] `cargo: fmt`.
- [ ] `cargo: clippy`.
- [ ] `run: current lab`.
- [ ] `run: focus forge cli`.
- [ ] `run: focus forge gui`.

`.vscode/launch.json`:

- [ ] Debug current lab.
- [ ] Debug CLI.
- [ ] Debug GUI.

## CI Architecture

Add `.github/workflows/ci.yml` after the workspace exists.

CI should run:

- [ ] `cargo fmt --check`.
- [ ] `cargo check --workspace`.
- [ ] `cargo test --workspace`.
- [ ] `cargo clippy --workspace --all-targets --all-features`.

Matrix:

- [ ] Windows.
- [ ] Linux.
- [ ] macOS later if GUI/platform behavior requires it.

Rules:

- [ ] CI should mirror local commands.
- [ ] CI should not require network services beyond dependency download.
- [ ] CI should not run GUI manual tests.
- [ ] CI should stay fast enough to be encouraging.

## Data And Persistence Architecture

Early chapters:

- [ ] Keep data in memory.
- [ ] Use hard-coded sample projects.
- [ ] Avoid file paths until the learner has enough Rust context.

Persistence chapters:

- [ ] Add `Workspace`.
- [ ] Add JSON serialization with `serde`.
- [ ] Save to a repo-local data path during learning.
- [ ] Use `tempfile` in tests.
- [ ] Add backups before overwrite in later chapters.

Later chapters:

- [ ] Move real app data to platform-specific directories with `directories`.
- [ ] Keep sample data in `sample_data/`.
- [ ] Keep learner local data ignored by Git.
- [ ] Add import/export for portability.

## Dependency Introduction Plan

- [ ] Chapter 0-4: standard library first.
- [ ] Chapter 5: consider hand-written errors before `thiserror`.
- [ ] Chapter 7: add `serde` and `serde_json`.
- [ ] Chapter 8: add `clap`.
- [ ] Chapter 9: add test fakes before adding abstraction crates.
- [ ] Chapter 10: add `assert_cmd`, `predicates`, and `tempfile` if not already needed.
- [ ] Chapter 11: add `eframe` / `egui`.
- [ ] Chapter 14: add `time` or `chrono`.
- [ ] Chapter 15: add `tokio` and `reqwest`.
- [ ] Chapter 17: add `directories` and `tracing`.

Policy:

- [ ] Explain every dependency in `docs/dependencies.md`.
- [ ] Avoid crates that hide the concept currently being taught.
- [ ] Keep versions centralized in the workspace when practical.

## Git And Release Workflow

- [ ] Work in small increments.
- [ ] After each completed increment, run relevant verification.
- [ ] Stage only relevant files.
- [ ] Commit with a signed commit using global Git identity.
- [ ] Push to the configured upstream.
- [ ] Use concise conventional-style commit messages.
- [ ] Tag stable chapter checkpoints:
  - [ ] `chapter-00-start`.
  - [ ] `chapter-00-complete`.
  - [ ] `chapter-01-complete`.

## Initial Implementation Order

1. [x] Add `.gitignore`.
2. [x] Add `IMPLEMENTATION.md`.
3. [ ] Update `README.md` with setup and course promise.
4. [ ] Add `rust-toolchain.toml`.
5. [ ] Add root `Cargo.toml`.
6. [ ] Add `labs/ch00_setup`.
7. [ ] Add VS Code recommendations and tasks.
8. [ ] Add Chapter 0 guide and assignment.
9. [ ] Add getting-unstuck and compiler-error docs.
10. [ ] Verify clean checkout workflow.
11. [ ] Commit and push Phase 0.

## Technical Definition Of Done

- [ ] `git clone` gives a usable repository.
- [ ] VS Code recommends the intended extensions.
- [ ] `cargo check` works from the root.
- [ ] `cargo test` works from the root.
- [ ] `cargo fmt --check` works from the root.
- [ ] Chapter 0 can be completed in under ten minutes.
- [ ] Assignment verification commands are explicit.
- [ ] Learner data is ignored by Git.
- [ ] Sample data is committed.
- [ ] CI passes once added.
- [ ] The architecture supports Focus Forge growing from CLI to GUI without rewrites.
