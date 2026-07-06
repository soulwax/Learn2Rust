# Learn2Rust

Learn2Rust is a hands-on Rust learning environment for a returning computer scientist or experienced developer who mostly knows C#, Java, TypeScript, or similar object-oriented languages.

The course builds one useful application over time: **Focus Forge**, a personal project, task, and learning workbench. The path starts with small Rust labs, moves through a CLI, and grows into a desktop GUI application.

## Current State

The repository is in **Phase 0: Foundation**.

Already present:

- [x] Curriculum vision in `MASTERPLAN.md`
- [x] Technical implementation plan in `IMPLEMENTATION.md`
- [x] Agent handoff rules in `AGENTS.md`
- [x] Current status in `STATUS.md`
- [x] Basic `.gitignore`

Coming next:

- [ ] Rust toolchain pin
- [ ] Cargo workspace
- [ ] First runnable setup lab
- [ ] VS Code tasks and extension recommendations
- [ ] Chapter 0 guide and assignment

## Prerequisites

Expected:

- [ ] Rust installed through `rustup`
- [ ] `cargo` available in the terminal
- [ ] VS Code
- [ ] `rust-analyzer` extension
- [ ] Some programming background in C#, Java, TypeScript, or a similar language

Check your Rust tools:

```powershell
rustc --version
cargo --version
```

## How To Start Right Now

For the moment, read these files in order:

1. `MASTERPLAN.md`
2. `IMPLEMENTATION.md`
3. `AGENTS.md`
4. `STATUS.md`

Once the Cargo workspace exists, the normal learner workflow will become:

```powershell
cargo check
cargo test
cargo run -p ch00_setup
```

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
