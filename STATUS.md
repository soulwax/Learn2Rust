# Project Status

Current phase: Phase 2 in progress - First Product Slice (focus_forge_core built and tested; focus_forge_cli not started)
Current chapter: ch00 - Setup And First Run (teaching progress; unaffected by core build-ahead, see ADR 0002)
Last verified commit: ead2247
Last verified commands:
- git status --short --ignored
- git log -5 --oneline
- cargo fmt --check
- cargo check
- cargo test -p ch00_setup
- cargo clippy --workspace --all-targets --all-features
- cargo run -p ch00_setup -- Soulwax
- node JSON.parse validation of sample_data/demo_workspace.json
- cargo test -p focus_forge_core
- cargo test --workspace

Current repository state:
- Planning docs exist: README.md, MASTERPLAN.md, IMPLEMENTATION.md, AGENTS.md
- .gitignore exists
- Initial Cargo workspace exists
- Cargo.lock exists and should be committed
- First runnable lab exists: labs/ch00_setup
- VS Code workspace configuration exists
- Chapter 0 guide and assignment exist
- Getting-unstuck and compiler-error docs exist
- ch00_setup lab is packed with teaching comments (the reference style for all labs)
- Teaching Comment Style convention recorded in IMPLEMENTATION.md
- sample_data/demo_workspace.json is verified by crates/focus_forge_core/tests/sample_data.rs
- Chapter 0 checkpoint tagged: chapter-00-complete
- Product phase and teaching chapter are intentionally decoupled (ADR 0002): core crate is built ahead of the curriculum, so phase advances independently of chapter

Next recommended increment:
- Time the Chapter 0 path (Phase 1 gate target: under ten minutes) and record it.
- Begin the focus_forge_cli slice (separate spec) that drives focus_forge_core:
  project add/list/show, task add/done, note add, workspace export/import.

Known blockers:
- none

Ignored local files observed:
- .env
- .env.local
