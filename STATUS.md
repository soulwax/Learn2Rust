---
phase: 2
phase_name: First Product Slice
chapter: ch01
chapter_name: Variables, Functions, And Basic Types
---

# Project Status

Current phase: Phase 2 in progress - First Product Slice (focus_forge_core and focus_forge_cli built and tested; Chapter 2 remains)
Current chapter: ch01 - Variables, Functions, And Basic Types (teaching progress; unaffected by core/CLI build-ahead, see ADR 0002)
Last verified commit: de490c5
Last verified commands:
- git status --short --ignored
- git log -5 --oneline
- cargo fmt --check
- cargo check
- cargo clippy --workspace --all-targets --all-features
- cargo test --workspace
- cargo test -p ch01_basics
- cargo run -p ch01_basics
- cargo run -p focus_forge_status -- --out <tmp>/status.json
- cargo run -p focus_forge_cli -- --file <tmp> project add/list/show, task add/done, note add (manual smoke test)
- timed Chapter 0 verification path:
  - cargo fmt --check
  - cargo check
  - cargo test
  - cargo clippy --workspace --all-targets --all-features
  - cargo run -p ch00_setup
  - cargo run -p ch00_setup -- Soulwax
  - result: 1.69 seconds on a warm local build, safely under the Phase 1 ten-minute target

Current repository state:
- Planning docs exist: README.md, MASTERPLAN.md, IMPLEMENTATION.md, AGENTS.md
- .gitignore exists
- Initial Cargo workspace exists
- Cargo.lock exists and should be committed
- First runnable lab exists: labs/ch00_setup
- VS Code workspace configuration exists
- Chapter 0 guide and assignment exist
- Chapter 1 guide, assignment, and runnable basics lab exist
- Getting-unstuck and compiler-error docs exist
- Phase 1 gate is closed: Chapter 0 timing is under ten minutes, tests pass,
  the deliberate compiler-error exercise exists, the VS Code check task maps
  to `cargo check`, and this file points to the next increment
- ch00_setup lab is packed with teaching comments (the reference style for all labs)
- Teaching Comment Style convention recorded in IMPLEMENTATION.md
- sample_data/demo_workspace.json is verified by crates/focus_forge_core/tests/sample_data.rs
- Chapter 0 checkpoint tagged: chapter-00-complete
- Product phase and teaching chapter are intentionally decoupled (ADR 0002, amended for clap): core and CLI crates are built ahead of the curriculum, so phase advances independently of chapter
- focus_forge_cli exists per docs/superpowers/specs/2026-07-07-focus-forge-cli-design.md:
  project add/list/show, task add/done, note add, all TDD'd (19 unit tests + 6 assert_cmd integration tests)

Next recommended increment:
- Chapter 2 (ownership, borrowing, and references) to close out the remaining Phase 2 gate items.
- Consider the workspace export/import slice (explicitly out of scope for the CLI slice just completed).

Known blockers:
- none

Ignored local files observed:
- .env
- .env.local
