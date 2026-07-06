# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What This Repository Is

Learn2Rust is a **course-as-repository**: a clone-and-learn Rust curriculum for a returning computer scientist coming from C#/Java/TypeScript. It teaches Rust by building one real application, **Focus Forge** (a personal project/task/note workbench), across chapters — starting as CLI labs and growing into an `egui`/`eframe` desktop GUI.

Most work here is authoring curriculum and scaffolding the workspace incrementally, not shipping a product. The repository is currently in **Phase 0 (Foundation)**; only `labs/ch00_setup` exists as runnable code.

## Common Commands

```powershell
cargo check                                              # check whole workspace
cargo test                                               # test whole workspace
cargo fmt --check                                        # verify formatting (CI-equivalent)
cargo clippy --workspace --all-targets --all-features    # lint

cargo run -p ch00_setup                                  # run a lab (prints greeting + status)
cargo run -p ch00_setup -- Soulwax                       # run a lab with an arg

cargo check -p ch00_setup                                # scope a command to one package
cargo test -p ch00_setup                                 # run one package's tests
cargo test -p ch00_setup greeting_uses_provided_name     # run a single test by name
```

VS Code equivalents live in `.vscode/tasks.json` (Terminal > Run Task): `cargo: check workspace`, `cargo: test workspace`, `cargo: fmt check`, `cargo: clippy workspace`, `run: ch00 setup lab`.

Toolchain is pinned to stable in `rust-toolchain.toml` (with `rustfmt` + `clippy`). `Cargo.lock` **is committed** (this repo builds runnable apps, not just a library).

## Architecture: Directory Ownership

Every change belongs to exactly one area. Respect these boundaries — the curriculum's value depends on separation staying clean as it grows:

| Area | Owns | Does NOT own |
| --- | --- | --- |
| root (`Cargo.toml`, docs) | orchestration, workspace manifest, tooling | application code (keep the root package-free) |
| `crates/focus_forge_core` | domain types, validation, errors, search, persistence, tests | any CLI/GUI formatting |
| `crates/focus_forge_cli` | command parsing, terminal output | domain rules |
| `crates/focus_forge_gui` | desktop UI state and rendering (`egui`/`eframe`) | domain rules |
| `labs/chXX_*` | small, focused chapter experiments | product architecture |
| `assignments/` | tasks, acceptance criteria, verification steps | long concept prose |
| `chapters/` | teaching narrative | assignment checklists |
| `docs/` | references, troubleshooting, decision records | step-by-step chapter tasks |
| `sample_data/` | committed curriculum data | personal learner data (must stay ignored) |

**Crate dependency direction is one-way:** `focus_forge_cli` and `focus_forge_gui` depend on `focus_forge_core`; `focus_forge_core` must never depend on CLI or GUI. Labs may depend on `focus_forge_core` only when a chapter needs it.

The workspace manifest lists **only members that exist** — add lab/crate members as they are created, not ahead of time. Shared dependency versions go in `[workspace.dependencies]`; crate manifests use `edition.workspace = true` etc.

## Working In This Repo: The Handoff Model

This repo is designed for cold-start agent handoff. `IMPLEMENTATION.md` is the technical source of truth and `MASTERPLAN.md` is the curriculum vision. Before starting work, read `STATUS.md` (current phase, chapter, last-verified commands, next increment), then the files relevant to the task.

**Task selection when the user doesn't specify one:** take the next smallest valid item. Priority order — keep the repo safe and root commands working → complete the current phase before expanding later chapters → add verification before adding more content → prefer a runnable vertical slice (testable through `focus_forge_core`/`focus_forge_cli`) over broad scaffolding. GUI work comes only after the core+CLI path exists; CI comes only after local commands are stable. Phase gates are defined in `IMPLEMENTATION.md` — do not mark a phase complete until its gate passes.

**Update `STATUS.md`** to point at the next increment as part of completing work.

## Non-Negotiable Constraints

- **Never** read, stage, or commit `.env` / `.env.local` / ignored secret or learner-data files. They exist locally and are git-ignored; leave them untouched.
- **Never** make a chapter/assignment depend on a paid service or live network access — the core course must work offline. HTTP features are optional and late.
- **Never** introduce a Rust feature earlier than the curriculum can explain it. Dependencies are introduced on a schedule (see the Dependency Introduction Plan in `IMPLEMENTATION.md`): std-only through ch4, `serde` at ch7, `clap` at ch8, `eframe`/`egui` at ch11, etc. Document every new dependency's rationale.
- **Never** let GUI or CLI code own domain rules, and never let the root crate become a dumping ground.
- A completed lab **must compile**. Compile-fail teaching examples go in Markdown or comments (or `trybuild` later) so the workspace stays green.

## Verification & Git

Run the smallest useful verification for the change:
- Docs-only: inspect the diff and run `git diff --check`.
- Rust: `cargo fmt --check`, `cargo check`, and the relevant tests.
- CLI behavior: run the exact command the learner is expected to use.

Commits use the user's global Git identity, are **signed and signed-off** (`git commit -S --signoff`), stage only intended files (path-specific `git add`, never blanket), and use conventional-style messages (`docs:`, `feat:`, `test:`, `chore:`). Stable chapters are tagged (e.g. `chapter-00-complete`). Do not hide failing verification or claim a command ran when it did not — if a command can't run because required files don't exist yet, say so plainly.
