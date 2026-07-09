# Learn2Rust

Learn2Rust is a hands-on Rust learning environment for a returning computer scientist or experienced developer who mostly knows C#, Java, TypeScript, or similar object-oriented languages.

The course builds one useful application over time: **Focus Forge**, a personal project, task, and learning workbench. The path starts with small Rust labs, moves through a CLI, and grows into a desktop GUI application.

## Current State

The repository has completed the **Phase 0: Foundation** work and has a tested
first Focus Forge product slice built ahead of the teaching track.

Teaching progress is currently at **Chapter 2: Ownership, Borrowing, And References**.

Already present:

- [x] Curriculum vision in `MASTERPLAN.md`
- [x] Technical implementation plan in `IMPLEMENTATION.md`
- [x] Agent handoff rules in `AGENTS.md`
- [x] Current status in `STATUS.md`
- [x] Basic `.gitignore`
- [x] Stable Rust toolchain pin
- [x] Initial Cargo workspace
- [x] First runnable setup lab
- [x] VS Code extension recommendations and tasks
- [x] Chapter 0 guide and assignment
- [x] Chapter 1 guide, assignment, and runnable basics lab
- [x] Chapter 2 guide, assignment, and runnable ownership lab
- [x] Focus Forge core crate
- [x] Focus Forge CLI crate with project, task, and note commands
- [x] Committed sample workspace data

Coming next:

- [ ] Chapter 3: structs, enums, and domain modeling
- [ ] Workspace export/import slice

## Prerequisites

Expected:

- [ ] Rust installed through `rustup`
- [ ] `cargo` available in the terminal
- [ ] VS Code
- [ ] Recommended VS Code extensions from `.vscode/extensions.json`
- [ ] Some programming background in C#, Java, TypeScript, or a similar language

Check your Rust tools:

```powershell
rustc --version
cargo --version
```

## How To Start Right Now To Develop The Course - Not for Students

For the moment, read these files in order:

1. `MASTERPLAN.md`
2. `IMPLEMENTATION.md`
3. `AGENTS.md`
4. `STATUS.md`

Run the first lab:

```powershell
cargo check
cargo test
cargo run -p ch00_setup
cargo run -p ch01_basics
cargo run -p ch02_ownership
```

Try giving it your name:

```powershell
cargo run -p ch00_setup -- Soulwax
```

In VS Code, use **Terminal > Run Task...** for:

- `cargo: check workspace`
- `cargo: test workspace`
- `cargo: fmt check`
- `cargo: clippy workspace`
- `run: ch00 setup lab`
- `run: ch01 basics lab`
- `run: ch02 ownership lab`

## Learning Shape

This repository is designed as one coherent workshop:

- `crates/` will contain the real Focus Forge application.
- `labs/` will contain focused chapter exercises.
- `assignments/` will contain task checklists and verification steps.
- `chapters/` will contain the teaching narrative.
- `docs/` will contain deeper references and troubleshooting.
- `sample_data/` will contain committed curriculum data.

The course uses an agile spiral: learn a small Rust concept, build something visible, test it, then revisit the same concept later in a richer setting.

## Safety Note

Local environment files such as `.env` and `.env.local` are ignored by Git and should not be committed.
