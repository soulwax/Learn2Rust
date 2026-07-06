# Implementation TODO: Technical Realisation

This document is the technical companion to `MASTERPLAN.md`. The masterplan describes the learning vision; this file describes how to turn the repository into the actual Rust learning environment.

The target is one GitHub repository that opens cleanly in VS Code, runs with Cargo from the root, teaches Rust through chapter labs, and grows one real application: **Focus Forge**.

## Agent Takeover Contract

This section is the operational contract for any future agent entering the repository cold. An agent should be able to start here, determine the current state, choose the next safe increment, implement it, verify it, commit it, and leave the repository ready for the next agent.

### First Five Minutes

Run these commands before making changes:

```powershell
git status --short --ignored
git branch --show-current
git rev-parse --abbrev-ref --symbolic-full-name '@{u}'
git log -5 --oneline
if (Get-Command rg -ErrorAction SilentlyContinue) { rg --files -uu } else { Get-ChildItem -Recurse -Force -File }
```

Then read, in this order:

- [ ] `README.md` for learner-facing promise and setup state.
- [ ] `MASTERPLAN.md` for curriculum intent.
- [ ] `IMPLEMENTATION.md` for technical state and next steps.
- [ ] Existing chapter, assignment, and docs files relevant to the task.
- [ ] Existing Cargo manifests before editing Rust code.

Rules:

- [ ] Do not read `.env`, `.env.local`, or any ignored secret file unless the user explicitly asks and the task requires it.
- [ ] Do not stage ignored files.
- [ ] Do not delete or reset user changes.
- [ ] If the working tree is dirty, identify which changes are yours, which are user changes, and which are unrelated.
- [ ] Prefer small increments that leave the repo runnable.

### State Detection Checklist

Before choosing work, determine:

- [ ] Does root `Cargo.toml` exist?
- [ ] Does `rust-toolchain.toml` exist?
- [ ] Does `.vscode/` exist?
- [ ] Which `crates/` exist?
- [ ] Which `labs/` exist?
- [ ] Which `chapters/` and `assignments/` exist?
- [ ] Does `cargo check` work from the root?
- [ ] Does `cargo test` work from the root?
- [ ] Does CI exist?
- [ ] What was the most recent commit?
- [ ] Are there ignored local files, especially env files or learner data?

Use this state to select the smallest valid next increment from `Initial Implementation Order`, then from the phase-specific TODOs.

### Task Selection Algorithm

Use this order unless the user gives a more specific request:

1. [ ] Preserve safety: do not touch secrets, ignored learner data, or unrelated changes.
2. [ ] If the repo cannot be checked from the root, prioritize the minimal workspace/tooling fix.
3. [ ] If Chapter 0 is incomplete, prioritize Chapter 0 infrastructure and first-run workflow.
4. [ ] If a chapter exists but lacks verification, add verification before adding new content.
5. [ ] If the current chapter is complete, advance one chapter.
6. [ ] Prefer docs plus runnable examples before broader architecture.
7. [ ] Prefer product slices that can be tested through `focus_forge_core` or `focus_forge_cli`.
8. [ ] Add GUI work only after the core and CLI path exist.
9. [ ] Add CI only after local commands are stable.
10. [ ] Commit and push the completed increment.

### Increment Exit Criteria

An increment is complete only when:

- [ ] The requested change is implemented.
- [ ] Relevant docs or TODOs are updated.
- [ ] Relevant verification has run, or the limitation is documented.
- [ ] `git status --short` is understood.
- [ ] Only intended files are staged.
- [ ] The commit is signed and uses the global Git identity.
- [ ] The commit is pushed to the configured upstream.
- [ ] The final response states commit hash, verification, and any remaining risks.

## Agent Handoff Artifacts

To make handoff durable, the repository should eventually contain these files:

- [x] `AGENTS.md`: concise instructions for future coding agents.
- [x] `STATUS.md`: current phase, current chapter, last verified command, and next recommended increment.
- [ ] `docs/decision-records/`: short architecture decision records.
- [ ] `docs/task-template.md`: reusable task card format.
- [ ] `docs/chapter-template.md`: reusable chapter guide format.
- [ ] `docs/assignment-template.md`: reusable assignment format.

Until those files exist, `IMPLEMENTATION.md` is the handoff source of truth.

### `STATUS.md` Template

When created, `STATUS.md` should stay short:

```markdown
# Project Status

Current phase: Phase N - Name
Current chapter: chXX - Title
Last verified commit: <hash>
Last verified commands:
- command

Next recommended increment:
- task

Known blockers:
- none

Ignored local files observed:
- .env
- .env.local
```

### Task Card Template

Every future issue, TODO card, or agent task should be expressible as:

```markdown
## Task

Goal:

Context files:

Files expected to change:

Files that must not change:

Commands before:

Implementation steps:

Verification:

Commit message:

Handoff notes:
```

## Non-Negotiable Safety Rules

- [ ] Never commit `.env`, `.env.local`, secrets, tokens, local learner data, logs, or generated build output.
- [ ] Never rewrite Git history unless explicitly asked.
- [ ] Never hide failing verification.
- [ ] Never make a chapter depend on a paid service or live network access.
- [ ] Never make the root crate an accidental dumping ground.
- [ ] Never let GUI code become the owner of domain rules.
- [ ] Never add a dependency without documenting why it exists.
- [ ] Never introduce advanced Rust features earlier than the curriculum can explain them.
- [ ] Never remove a learner recovery path while refactoring.

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
  AGENTS.md
  Cargo.toml
  Cargo.lock
  IMPLEMENTATION.md
  MASTERPLAN.md
  README.md
  STATUS.md
  rust-toolchain.toml
```

## Repository Ownership Map

Use this map to decide where a change belongs.

| Area | Owns | Does not own |
| --- | --- | --- |
| `README.md` | First impression, setup, how to start | Deep curriculum rationale |
| `MASTERPLAN.md` | Vision, pedagogy, chapter arc | Low-level command details |
| `IMPLEMENTATION.md` | Technical realization and handoff | Learner-facing tutorial prose |
| `STATUS.md` | Current state and next step | Full history or architecture rationale |
| `AGENTS.md` | Short agent operating rules | Long explanations |
| `chapters/` | Teaching narrative | Assignment checklists |
| `assignments/` | Tasks, acceptance criteria, verification | Long concept essays |
| `docs/` | References, troubleshooting, deeper explanations | Step-by-step chapter tasks |
| `labs/` | Focused experiments | Product architecture |
| `crates/focus_forge_core/` | Domain logic and tests | CLI/GUI formatting |
| `crates/focus_forge_cli/` | Command parsing and terminal UX | Domain rules |
| `crates/focus_forge_gui/` | Desktop UI state and rendering | Domain rules |
| `sample_data/` | Committed curriculum data | Personal learner data |

## Command Catalog

Agents should keep these commands working as soon as the relevant files exist.

Workspace:

```powershell
cargo check
cargo test
cargo fmt --check
cargo clippy --workspace --all-targets --all-features
```

Focused packages:

```powershell
cargo check -p ch00_setup
cargo test -p ch01_basics
cargo test -p focus_forge_core
cargo run -p focus_forge_cli -- project list
cargo run -p focus_forge_gui
```

Git and handoff:

```powershell
git status --short --ignored
git diff --stat
git diff
git diff --cached --stat
git log -5 --oneline
```

When commands are not available yet:

- [ ] Say which missing file or phase blocks the command.
- [ ] Add the command to the relevant TODO section.
- [ ] Do not pretend verification ran.

## Files To Create First

- [x] Add `.gitignore`.
- [x] Add `IMPLEMENTATION.md`.
- [x] Add `AGENTS.md`.
- [x] Add `STATUS.md`.
- [x] Update `README.md`.
- [x] Add `rust-toolchain.toml`.
- [x] Add root `Cargo.toml`.
- [x] Add `.vscode/extensions.json`.
- [x] Add `.vscode/settings.json`.
- [x] Add `.vscode/tasks.json`.
- [x] Add `.vscode/launch.json`.
- [x] Add `docs/decision-records/0001-use-cargo-workspace.md`.
- [x] Add `docs/getting-unstuck.md`.
- [x] Add `docs/compiler-errors.md`.
- [x] Add `labs/ch00_setup`.
- [x] Add `chapters/00-setup.md`.
- [x] Add `assignments/ch00-setup.md`.

## `.gitignore` Policy

- [x] Ignore Rust build output: `target/`.
- [x] Ignore local env files: `.env`, `.env.*`, except `.env.example`.
- [x] Ignore logs and temp files.
- [x] Ignore local learner/app data.
- [x] Ignore generated release artifacts.
- [x] Ignore OS/editor noise.
- [x] Keep `Cargo.lock` committed because this repository builds runnable apps and a learning workspace.
- [x] Keep `.vscode/` committed because the VS Code environment is part of the learning product.
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

### Lab Package Contract

Each lab package should contain:

```text
labs/chXX_topic/
  Cargo.toml
  src/
    main.rs
  tests/
    chapter_checks.rs
  README.md
```

Rules:

- [ ] `Cargo.toml` package name matches the lab folder unless there is a collision.
- [ ] `src/main.rs` gives a fast visible result.
- [ ] Tests verify the learning objective without becoming puzzle gates.
- [ ] Comments may show broken examples, but the completed lab must compile.
- [ ] If a compile-fail lesson is needed, use `trybuild` later or place the broken code in Markdown.
- [ ] The lab README should be short; the real assignment lives in `assignments/`.

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

### Assignment Completion Contract

An assignment is complete when:

- [ ] The learner can identify the files they changed.
- [ ] The learner can run the verification command.
- [ ] The learner can explain the new Rust concept in one paragraph.
- [ ] The learner can name the closest C#, Java, or TypeScript analogy.
- [ ] The learner can describe one way the analogy breaks down.
- [ ] The learner has either completed or consciously skipped the stretch tasks.
- [ ] The product artifact or lab artifact is visible.

### Common Assignment Failure Modes

- [ ] Too much concept prose before the first command.
- [ ] Verification depends on hidden state.
- [ ] The assignment asks for design choices before the learner has examples.
- [ ] The stretch task silently becomes required.
- [ ] The OOP analogy is missing where it would reduce friction.
- [ ] The math warm-up is unrelated to the app.
- [ ] The app improvement is too invisible to feel rewarding.

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

### Signed Increment Protocol

For every completed increment:

- [ ] Run `git status --short --ignored`.
- [ ] Review `git diff --stat` and `git diff`.
- [ ] Stage only intended files with path-specific `git add`.
- [ ] Review `git diff --cached --stat`.
- [ ] Commit with `git commit -S --signoff`.
- [ ] Use the user's global `user.name` and `user.email`.
- [ ] Push with `git push`.
- [ ] Verify the latest commit with `git log -1 --show-signature --pretty=fuller`.
- [ ] Leave ignored `.env` and local data files untouched.

Commit message examples:

- [ ] `docs: add implementation handoff protocol`
- [ ] `chore: scaffold rust workspace`
- [ ] `docs: add chapter 0 setup assignment`
- [ ] `feat: add focus forge core project model`
- [ ] `test: cover workspace persistence`

### Handoff Response Template

At the end of an increment, report:

```text
Changed:
- file or area

Verified:
- command or manual check

Commit:
- hash subject

Pushed:
- remote/branch

Left untouched:
- ignored or unrelated files, if any

Next:
- recommended next increment
```

## Phase Gates

Do not mark a phase complete until its gate passes.

### Phase 0 Gate: Foundation

- [ ] `README.md` explains the project and first command.
- [ ] `.gitignore` exists and protects local files.
- [ ] `rust-toolchain.toml` exists.
- [ ] Root `Cargo.toml` exists.
- [ ] VS Code extension recommendations exist.
- [ ] `labs/ch00_setup` runs.
- [ ] `cargo check` works from the root.
- [ ] Chapter 0 assignment exists.
- [ ] Getting-unstuck docs exist.

### Phase 1 Gate: First Feedback Loop

- [ ] Chapter 0 can be completed in under ten minutes.
- [ ] One test passes.
- [ ] One deliberate compiler-error exercise exists.
- [ ] VS Code check task works.
- [ ] `STATUS.md` points to the next increment.

### Phase 2 Gate: First Product Slice

- [ ] `focus_forge_core` exists.
- [ ] `focus_forge_cli` exists.
- [ ] CLI can print a project summary.
- [ ] Core validation has tests.
- [ ] Chapters 1 and 2 exist.
- [ ] Ownership is introduced gently.

### Phase 3 Gate: Durable App Core

- [ ] Domain models exist.
- [ ] JSON persistence works.
- [ ] CLI can add/list projects and tasks.
- [ ] Sample data exists.
- [ ] Persistence tests use temp directories.
- [ ] Chapters 3-8 exist.

### Phase 4 Gate: Abstraction And Refactoring

- [ ] Traits exist only at real boundaries.
- [ ] Fake storage exists for tests.
- [ ] CLI and persistence integration tests exist.
- [ ] Refactoring chapter uses code already built by the learner.
- [ ] Chapters 9-10 exist.

### Phase 5 Gate: GUI Payoff

- [ ] `focus_forge_gui` exists.
- [ ] GUI opens on a clean checkout.
- [ ] GUI displays real workspace data.
- [ ] GUI can edit and save at least one product concept.
- [ ] GUI manual verification checklist exists.
- [ ] Chapters 11-14 exist.

### Phase 6 Gate: Advanced Practical Rust

- [ ] Async and HTTP are optional.
- [ ] Network behavior has offline fixtures or mocks.
- [ ] Import/export and backups exist.
- [ ] Settings and data directories exist.
- [ ] Release build instructions exist.
- [ ] Chapters 15-18 exist.

### Phase 7 Gate: Curriculum Hardening

- [ ] Clean clone walkthrough completed.
- [ ] CI passes.
- [ ] Chapter timing has been recorded.
- [ ] Chapter checkpoints are tagged.
- [ ] Solutions policy is decided.
- [ ] Known rough edges are documented.

## Architecture Decision Records

When an agent makes a durable architecture choice, add a short ADR under `docs/decision-records/`.

ADR naming:

```text
docs/decision-records/0001-use-cargo-workspace.md
docs/decision-records/0002-use-egui-for-gui.md
```

ADR template:

```markdown
# ADR N: Title

Status: Accepted
Date: YYYY-MM-DD

## Context

## Decision

## Consequences

## Alternatives Considered
```

Create ADRs for:

- [ ] Cargo workspace layout.
- [ ] `egui` / `eframe` as GUI stack.
- [ ] JSON before SQLite.
- [ ] CLI before GUI.
- [ ] Optional HTTP only.
- [ ] Signed incremental commit workflow.

## Initial Implementation Order

1. [x] Add `.gitignore`.
2. [x] Add `IMPLEMENTATION.md`.
3. [x] Add `AGENTS.md`.
4. [x] Add `STATUS.md`.
5. [x] Update `README.md` with setup and course promise.
6. [x] Add `rust-toolchain.toml`.
7. [x] Add root `Cargo.toml`.
8. [x] Add `labs/ch00_setup`.
9. [x] Add VS Code recommendations and tasks.
10. [x] Add Chapter 0 guide and assignment.
11. [x] Add getting-unstuck and compiler-error docs.
12. [x] Add first ADR for workspace layout.
13. [ ] Verify clean checkout workflow.
14. [ ] Commit and push Phase 0.

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
- [ ] A new agent can determine current state by reading `README.md`, `MASTERPLAN.md`, `IMPLEMENTATION.md`, and `STATUS.md`.
- [ ] A new agent can identify the next safe increment without asking for hidden context.
- [ ] Every completed increment leaves verification notes and a signed pushed commit.
- [ ] Ignored secrets and local data remain untouched.
