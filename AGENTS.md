# Agent Instructions

This repository is a Rust learning environment. It teaches a returning computer scientist Rust by building one useful application, **Focus Forge**, through small chapter labs and an evolving Cargo workspace.

Use this file as the quick operating guide. Use `IMPLEMENTATION.md` for the full technical plan and `MASTERPLAN.md` for the curriculum vision.

## First Steps

Before editing:

```powershell
git status --short --ignored
git branch --show-current
git rev-parse --abbrev-ref --symbolic-full-name '@{u}'
git log -5 --oneline
if (Get-Command rg -ErrorAction SilentlyContinue) { rg --files } else { Get-ChildItem -Recurse -File }
```

Then read:

- [ ] `README.md`
- [ ] `MASTERPLAN.md`
- [ ] `IMPLEMENTATION.md`
- [ ] `STATUS.md`, once it exists
- [ ] Files directly relevant to the task

## Safety Rules

- [ ] Do not read, stage, commit, or summarize `.env`, `.env.local`, or ignored secret files.
- [ ] Do not stage ignored learner data, logs, build output, or generated release artifacts.
- [ ] Do not overwrite unrelated user changes.
- [ ] Do not rewrite Git history unless explicitly asked.
- [ ] Do not add dependencies without documenting why they exist.
- [ ] Do not let CLI or GUI code own domain rules.
- [ ] Do not make core course progress depend on paid services or live network access.

## Architecture Rules

- [ ] Root folder owns orchestration, documentation, and tooling.
- [ ] `crates/focus_forge_core` owns domain logic, validation, persistence, search, and tests.
- [ ] `crates/focus_forge_cli` owns command parsing and terminal output.
- [ ] `crates/focus_forge_gui` owns desktop UI state and rendering.
- [ ] `crates/focus_forge_status` owns generating `site/static/status.json` from git/cargo (workspace facts, test counts, phase/chapter). It is a repo-maintenance tool, not a Focus Forge product feature — it depends on nothing else in the workspace and never touches `./site` beyond writing that one file.
- [ ] `labs/` owns focused chapter experiments.
- [ ] `assignments/` owns tasks, acceptance criteria, and verification.
- [ ] `chapters/` owns teaching narrative.
- [ ] `docs/` owns deeper references and troubleshooting.
- [ ] `sample_data/` owns committed curriculum data, not personal learner data.

## Task Selection

When the user does not specify a task, take the next smallest valid item from `IMPLEMENTATION.md`.

Default order:

1. [ ] Keep the repo safe and understandable.
2. [ ] Make root checks work.
3. [ ] Complete Phase 0 before expanding later chapters.
4. [ ] Add verification before adding more content.
5. [ ] Prefer a runnable vertical slice over broad scaffolding.
6. [ ] Commit and push each completed increment.

## Verification

Run the smallest useful verification for the change.

- Documentation-only: inspect the diff and run `git diff --check`.
- Rust workspace: run `cargo fmt --check`, `cargo check`, and relevant tests.
- CLI behavior: run the exact command the learner is expected to use.
- GUI behavior: run code checks and perform the documented manual check.

If a command cannot run because the required files do not exist yet, say that plainly.

## Git Workflow

After every completed increment:

```powershell
git status --short --ignored
git diff --stat
git diff
git add -- path/to/intended-file
git diff --cached --stat
git diff --cached --check
git commit -S --signoff -m "type: concise summary"
git push
git log -1 --show-signature --pretty=fuller
```

Use the user's global Git identity. Stage only intended files. Leave ignored `.env` and `.env.local` untouched.

## Handoff Response

End each increment with:

- Changed files or areas.
- Verification run.
- Commit hash and subject.
- Push destination.
- Ignored or unrelated files left untouched.
- Recommended next increment.
