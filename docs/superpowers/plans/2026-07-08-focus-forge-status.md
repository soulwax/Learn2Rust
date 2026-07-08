# focus_forge_status Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build `crates/focus_forge_status`, a standalone binary that shells out to `git`/`cargo` and reads `STATUS.md` front matter to generate a static `status.json` describing workspace crates/labs, their test counts, the last commit, and the current phase/chapter.

**Architecture:** One new binary-only crate with no path dependency on `focus_forge_core` or `focus_forge_cli`. Five small modules each owning one data source (`git.rs`, `cargo_meta.rs`, `test_counts.rs`, `status_md.rs`), a `model.rs` holding the `Serialize`able output shape, and a `lib.rs` that wires them together behind `pub fn run(out_path: &Path) -> Result<(), StatusError>`. `main.rs` stays a thin `clap`-based entry point matching `focus_forge_cli`'s existing pattern.

**Tech Stack:** Rust std (`std::process::Command`, `std::fs`, `std::time::SystemTime`), `serde`/`serde_json` (already workspace dependencies), `clap` (already a workspace dependency), `thiserror` (already a workspace dependency). No new dependencies.

## Global Constraints

- No new crates in `[workspace.dependencies]` — reuse `serde`, `serde_json`, `clap`, `thiserror` exactly as already pinned in root `Cargo.toml`.
- `focus_forge_status` must not depend on `focus_forge_core` or `focus_forge_cli` (spec: "Dependency direction... has no path dependency on focus_forge_core or focus_forge_cli").
- No `chrono`/`time` crate — `generated_at` is hand-formatted RFC 3339 from `std::time::SystemTime` (spec: "Output Shape").
- No test in the default `cargo test` run shells out to real `git`/`cargo` — all parser unit tests run against fixture strings/files (spec: "Testing").
- Do not touch anything under `./site` — that tree is owned by concurrent, separate work (spec: "Non-Goals").
- Default output path is `site/static/status.json`; `--out <path>` overrides it (spec: "CLI Surface").
- `STATUS.md` keeps its existing prose body untouched; only a YAML front-matter block is prepended (spec: "STATUS.md Front Matter").
- Follow this repo's Teaching Comment Style is NOT required here — `focus_forge_status` is a maintenance tool, not learner-facing lab/product code, so it does not need the rich `///`/`//` C#/Java bridge comments `IMPLEMENTATION.md` mandates for `focus_forge_cli`/labs. Keep comments to non-obvious rationale only, per this repo's default CLAUDE.md comment policy.
- Commits are signed and signed-off (`git commit -S --signoff`), stage only intended files, use conventional-style messages, per `IMPLEMENTATION.md`'s Signed Increment Protocol.

---

## Task 1: Scaffold the crate and add STATUS.md front matter

**Files:**
- Create: `crates/focus_forge_status/Cargo.toml`
- Create: `crates/focus_forge_status/src/main.rs`
- Create: `crates/focus_forge_status/src/lib.rs`
- Modify: `Cargo.toml:1-7` (add workspace member)
- Modify: `STATUS.md:1` (prepend front matter)

**Interfaces:**
- Produces: crate `focus_forge_status` exists, compiles, and `cargo run -p focus_forge_status` runs (does nothing yet but exits 0). Later tasks add real logic behind `pub fn run`.
- Produces: `STATUS.md` now starts with a parseable front-matter block that Task 3 will consume.

This task has no failing test to write first — it's scaffolding (per Task Right-Sizing: "fold setup, configuration, scaffolding... into the task whose deliverable needs them"). The deliverable it unblocks is "the crate exists and the workspace still builds," verified by `cargo check`.

- [ ] **Step 1: Create the crate manifest**

Create `crates/focus_forge_status/Cargo.toml`:

```toml
[package]
name = "focus_forge_status"
version = "0.1.0"
edition.workspace = true
repository.workspace = true

[[bin]]
name = "focus-forge-status"
path = "src/main.rs"

[dependencies]
clap = { workspace = true }
serde = { workspace = true }
serde_json = { workspace = true }
thiserror = { workspace = true }

[dev-dependencies]
tempfile = { workspace = true }
```

- [ ] **Step 2: Add the crate to the workspace members**

In `Cargo.toml`, change:

```toml
[workspace]
resolver = "2"
members = [
    "crates/focus_forge_core",
    "crates/focus_forge_cli",
    "labs/ch00_setup",
]
```

to:

```toml
[workspace]
resolver = "2"
members = [
    "crates/focus_forge_core",
    "crates/focus_forge_cli",
    "crates/focus_forge_status",
    "labs/ch00_setup",
]
```

- [ ] **Step 3: Create a minimal `lib.rs` and `main.rs`**

Create `crates/focus_forge_status/src/lib.rs`:

```rust
//! Generates a static status.json snapshot of the workspace (crates/labs,
//! test counts, last commit, phase/chapter) by shelling out to git and
//! cargo. This is a repo-maintenance tool, not part of the Focus Forge
//! product surface — see docs/superpowers/specs/2026-07-08-focus-forge-status-design.md.

use std::path::Path;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StatusError {
    #[error("git command failed: {0}")]
    Git(String),
    #[error("cargo metadata failed: {0}")]
    CargoMetadata(String),
    #[error("cargo test failed for {0}: {1}")]
    CargoTest(String, String),
    #[error("could not parse STATUS.md front matter: {0}")]
    StatusMd(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("json error: {0}")]
    Json(String),
}

pub fn run(_out_path: &Path) -> Result<(), StatusError> {
    Ok(())
}
```

Create `crates/focus_forge_status/src/main.rs`:

```rust
//! Thin binary entry point: parse args, call `run`, map any error to a
//! nonzero exit code. Mirrors focus_forge_cli's main.rs shape.

use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "focus-forge-status")]
struct Cli {
    #[arg(long, default_value = "site/static/status.json")]
    out: PathBuf,
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = focus_forge_status::run(&cli.out) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
```

- [ ] **Step 4: Verify the workspace still builds**

Run: `cargo check --workspace`
Expected: `Finished` with no errors. This confirms the new crate compiles and the workspace member list is valid.

- [ ] **Step 5: Add front matter to STATUS.md**

At the very top of `STATUS.md` (before the `# Project Status` heading), insert:

```markdown
---
phase: 2
phase_name: First Product Slice
chapter: ch00
chapter_name: Setup And First Run
---

```

So the file starts:

```markdown
---
phase: 2
phase_name: First Product Slice
chapter: ch00
chapter_name: Setup And First Run
---

# Project Status

Current phase: Phase 2 gate met - First Product Slice (focus_forge_core and focus_forge_cli both built and tested)
...
```

(Leave the rest of the file — every line from `# Project Status` onward — exactly as it is today. This is a prepend, not a rewrite.)

- [ ] **Step 6: Verify nothing else broke**

Run: `cargo fmt --check && cargo check --workspace`
Expected: both succeed. `STATUS.md` is not Rust so `cargo fmt` does not touch it.

- [ ] **Step 7: Commit**

```bash
git add Cargo.toml STATUS.md crates/focus_forge_status/Cargo.toml crates/focus_forge_status/src/main.rs crates/focus_forge_status/src/lib.rs
git commit -S --signoff -m "chore: scaffold focus_forge_status crate and STATUS.md front matter"
```

---

## Task 2: `model.rs` — the JSON output shape

**Files:**
- Create: `crates/focus_forge_status/src/model.rs`
- Modify: `crates/focus_forge_status/src/lib.rs` (add `mod model;`)

**Interfaces:**
- Consumes: nothing (pure data types).
- Produces: `pub struct Status { generated_at: String, last_commit: CommitInfo, phase: u32, phase_name: String, chapter: String, chapter_name: String, crates: Vec<CrateStatus> }`, `pub struct CommitInfo { hash: String, date: String }`, `pub struct CrateStatus { name: String, kind: String, tests_passed: u32, tests_failed: u32 }`. All `#[derive(Debug, Clone, Serialize)]`. Later tasks (git.rs, cargo_meta.rs, test_counts.rs, status_md.rs) construct these; `lib.rs`'s `run` assembles and serializes a `Status`.

- [ ] **Step 1: Write the failing test**

Create `crates/focus_forge_status/src/model.rs`:

```rust
//! The status.json output shape. See the "Output Shape" section of
//! docs/superpowers/specs/2026-07-08-focus-forge-status-design.md.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CommitInfo {
    pub hash: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateStatus {
    pub name: String,
    pub kind: String,
    pub tests_passed: u32,
    pub tests_failed: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub generated_at: String,
    pub last_commit: CommitInfo,
    pub phase: u32,
    pub phase_name: String,
    pub chapter: String,
    pub chapter_name: String,
    pub crates: Vec<CrateStatus>,
}

#[cfg(test)]
mod tests {
    use super::{CommitInfo, CrateStatus, Status};

    fn sample_status() -> Status {
        Status {
            generated_at: "2026-07-08T00:00:00Z".to_string(),
            last_commit: CommitInfo {
                hash: "abc123".to_string(),
                date: "2026-07-08T14:17:57+02:00".to_string(),
            },
            phase: 2,
            phase_name: "First Product Slice".to_string(),
            chapter: "ch00".to_string(),
            chapter_name: "Setup And First Run".to_string(),
            crates: vec![CrateStatus {
                name: "focus_forge_core".to_string(),
                kind: "crate".to_string(),
                tests_passed: 23,
                tests_failed: 0,
            }],
        }
    }

    #[test]
    fn status_serializes_to_documented_shape() {
        let status = sample_status();
        let json = serde_json::to_value(&status).unwrap();

        assert_eq!(json["phase"], 2);
        assert_eq!(json["phase_name"], "First Product Slice");
        assert_eq!(json["chapter"], "ch00");
        assert_eq!(json["last_commit"]["hash"], "abc123");
        assert_eq!(json["crates"][0]["name"], "focus_forge_core");
        assert_eq!(json["crates"][0]["kind"], "crate");
        assert_eq!(json["crates"][0]["tests_passed"], 23);
        assert_eq!(json["crates"][0]["tests_failed"], 0);
    }
}
```

Add `mod model;` to `crates/focus_forge_status/src/lib.rs` (below the existing `use` line, above `pub fn run`):

```rust
mod model;
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p focus_forge_status`
Expected: FAILS to compile — this is expected because `model` is a brand new module being added; if it instead compiles and the test passes immediately, something is wrong (re-check the struct fields match the test). In this case the module is written in the same step as its test because the "implementation" is a plain data struct with no behavior to get wrong — there is no meaningful RED state for a struct definition. Proceed to Step 3 to confirm GREEN.

- [ ] **Step 3: Run test to verify it passes**

Run: `cargo test -p focus_forge_status`
Expected: `test model::tests::status_serializes_to_documented_shape ... ok`

- [ ] **Step 4: Commit**

```bash
git add crates/focus_forge_status/src/model.rs crates/focus_forge_status/src/lib.rs
git commit -S --signoff -m "feat(status): add Status/CrateStatus/CommitInfo model types"
```

---

## Task 3: `status_md.rs` — parse STATUS.md front matter

**Files:**
- Create: `crates/focus_forge_status/src/status_md.rs`
- Create: `crates/focus_forge_status/tests/fixtures/status_with_front_matter.md`
- Create: `crates/focus_forge_status/tests/fixtures/status_missing_front_matter.md`
- Modify: `crates/focus_forge_status/src/lib.rs` (add `mod status_md;`)

**Interfaces:**
- Consumes: `crate::StatusError` (from Task 1).
- Produces: `pub struct FrontMatter { pub phase: u32, pub phase_name: String, pub chapter: String, pub chapter_name: String }` and `pub fn parse_front_matter(text: &str) -> Result<FrontMatter, StatusError>`. Task 6 (`lib.rs` wiring) calls `parse_front_matter(&std::fs::read_to_string("STATUS.md")?)`.

- [ ] **Step 1: Write the failing tests**

Create fixture `crates/focus_forge_status/tests/fixtures/status_with_front_matter.md`:

```markdown
---
phase: 2
phase_name: First Product Slice
chapter: ch00
chapter_name: Setup And First Run
---

# Project Status

Current phase: Phase 2 gate met - First Product Slice
```

Create fixture `crates/focus_forge_status/tests/fixtures/status_missing_front_matter.md`:

```markdown
# Project Status

No front matter here.
```

Create `crates/focus_forge_status/src/status_md.rs`:

```rust
//! Parses the small YAML-ish front-matter block at the top of STATUS.md
//! into phase/chapter fields. Not a general YAML parser — four scalar
//! `key: value` lines between two `---` delimiters is all this needs to
//! handle, so a full serde_yaml dependency would be more than this job
//! requires.

use crate::StatusError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontMatter {
    pub phase: u32,
    pub phase_name: String,
    pub chapter: String,
    pub chapter_name: String,
}

pub fn parse_front_matter(text: &str) -> Result<FrontMatter, StatusError> {
    let mut lines = text.lines();

    let first = lines.next().unwrap_or("");
    if first.trim() != "---" {
        return Err(StatusError::StatusMd(
            "file does not start with a '---' front matter delimiter".to_string(),
        ));
    }

    let mut phase: Option<u32> = None;
    let mut phase_name: Option<String> = None;
    let mut chapter: Option<String> = None;
    let mut chapter_name: Option<String> = None;

    for line in lines.by_ref() {
        if line.trim() == "---" {
            break;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().to_string();
        match key {
            "phase" => {
                phase = Some(value.parse::<u32>().map_err(|_| {
                    StatusError::StatusMd(format!("phase is not a number: {value}"))
                })?)
            }
            "phase_name" => phase_name = Some(value),
            "chapter" => chapter = Some(value),
            "chapter_name" => chapter_name = Some(value),
            _ => {}
        }
    }

    Ok(FrontMatter {
        phase: phase.ok_or_else(|| StatusError::StatusMd("missing 'phase' field".to_string()))?,
        phase_name: phase_name
            .ok_or_else(|| StatusError::StatusMd("missing 'phase_name' field".to_string()))?,
        chapter: chapter
            .ok_or_else(|| StatusError::StatusMd("missing 'chapter' field".to_string()))?,
        chapter_name: chapter_name
            .ok_or_else(|| StatusError::StatusMd("missing 'chapter_name' field".to_string()))?,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_front_matter, FrontMatter};

    #[test]
    fn parses_well_formed_front_matter() {
        let text = std::fs::read_to_string("tests/fixtures/status_with_front_matter.md").unwrap();

        let front_matter = parse_front_matter(&text).unwrap();

        assert_eq!(
            front_matter,
            FrontMatter {
                phase: 2,
                phase_name: "First Product Slice".to_string(),
                chapter: "ch00".to_string(),
                chapter_name: "Setup And First Run".to_string(),
            }
        );
    }

    #[test]
    fn missing_front_matter_errors() {
        let text =
            std::fs::read_to_string("tests/fixtures/status_missing_front_matter.md").unwrap();

        let err = parse_front_matter(&text).unwrap_err();

        assert!(matches!(err, crate::StatusError::StatusMd(_)));
    }
}
```

Add `mod status_md;` to `crates/focus_forge_status/src/lib.rs`:

```rust
mod status_md;
```

- [ ] **Step 2: Run test to verify it fails for the right reason**

Run: `cargo test -p focus_forge_status status_md`
Expected: at this point the module and its implementation were written together (same reasoning as Task 2 — the parsing logic is a single cohesive unit whose correctness the two tests together establish; there's no smaller RED slice that isn't "the whole function"). Confirm it currently compiles and passes in Step 3 rather than expecting a prior RED state.

- [ ] **Step 3: Run test to verify it passes**

Run: `cargo test -p focus_forge_status status_md`
Expected:
```
test status_md::tests::parses_well_formed_front_matter ... ok
test status_md::tests::missing_front_matter_errors ... ok
```

- [ ] **Step 4: Commit**

```bash
git add crates/focus_forge_status/src/status_md.rs crates/focus_forge_status/src/lib.rs crates/focus_forge_status/tests/fixtures/status_with_front_matter.md crates/focus_forge_status/tests/fixtures/status_missing_front_matter.md
git commit -S --signoff -m "feat(status): parse STATUS.md front matter"
```

---

## Task 4: `test_counts.rs` — parse `cargo test` summary lines

**Files:**
- Create: `crates/focus_forge_status/src/test_counts.rs`
- Modify: `crates/focus_forge_status/src/lib.rs` (add `mod test_counts;`)

**Interfaces:**
- Consumes: nothing external (pure string parsing).
- Produces: `pub fn parse_test_summary(stdout: &str) -> (u32, u32)` returning `(passed, failed)` summed across every `test result: ...` line found. Task 6 pairs this with the real `cargo test -p <member>` subprocess call.

This is the RED-first part of the plan: the parser has real branching logic (regex-free line scanning, summing across multiple lines, tolerating a `FAILED` variant) where a wrong-first-draft is plausible, so we write the test, watch it fail, then implement.

- [ ] **Step 1: Write the failing test**

Create `crates/focus_forge_status/src/test_counts.rs`:

```rust
//! Parses `cargo test` stdout for `test result: ...` summary lines and
//! sums pass/fail counts across all of them (a crate can print one such
//! line per test binary: unit tests, each integration test file, doctests).
```

Add `mod test_counts;` to `crates/focus_forge_status/src/lib.rs`:

```rust
mod test_counts;
```

Now add the test module at the bottom of `crates/focus_forge_status/src/test_counts.rs`:

```rust

#[cfg(test)]
mod tests {
    use super::parse_test_summary;

    #[test]
    fn parses_single_passing_summary_line() {
        let stdout = "running 3 tests\ntest a ... ok\n\ntest result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n";

        assert_eq!(parse_test_summary(stdout), (3, 0));
    }

    #[test]
    fn parses_summary_line_with_failures() {
        let stdout = "test result: FAILED. 2 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n";

        assert_eq!(parse_test_summary(stdout), (2, 1));
    }

    #[test]
    fn sums_multiple_summary_lines() {
        let stdout = "\
Running unittests src\\lib.rs
test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

Running tests\\cli.rs
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

Doc-tests focus_forge_status
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
";

        assert_eq!(parse_test_summary(stdout), (7, 0));
    }

    #[test]
    fn no_summary_lines_returns_zero() {
        assert_eq!(parse_test_summary("nothing relevant here\n"), (0, 0));
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p focus_forge_status test_counts`
Expected: FAIL with `cannot find function 'parse_test_summary' in module 'test_counts'` (or similar compile error) — `parse_test_summary` does not exist yet.

- [ ] **Step 3: Write minimal implementation**

At the top of `crates/focus_forge_status/src/test_counts.rs`, above the `#[cfg(test)]` block, add:

```rust
/// Sums `passed`/`failed` counts across every `test result: ...` line in
/// `stdout`. Lines that don't match the pattern are ignored.
pub fn parse_test_summary(stdout: &str) -> (u32, u32) {
    let mut passed = 0;
    let mut failed = 0;

    for line in stdout.lines() {
        let Some(rest) = line.trim_start().strip_prefix("test result:") else {
            continue;
        };
        // rest looks like " ok. 3 passed; 0 failed; 0 ignored; ..."
        for part in rest.split(';') {
            let part = part.trim();
            if let Some(n) = part.strip_suffix(" passed") {
                if let Some(n) = n.trim().rsplit(' ').next() {
                    passed += n.parse::<u32>().unwrap_or(0);
                }
            } else if let Some(n) = part.strip_suffix(" failed") {
                if let Some(n) = n.trim().rsplit(' ').next() {
                    failed += n.parse::<u32>().unwrap_or(0);
                }
            }
        }
    }

    (passed, failed)
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p focus_forge_status test_counts`
Expected:
```
test test_counts::tests::parses_single_passing_summary_line ... ok
test test_counts::tests::parses_summary_line_with_failures ... ok
test test_counts::tests::sums_multiple_summary_lines ... ok
test test_counts::tests::no_summary_lines_returns_zero ... ok
```

If `parses_single_passing_summary_line` or `parses_summary_line_with_failures` fail because of how " ok. 3 passed" splits — debug by checking that `rest.split(';')`'s first element is `" ok. 3 passed"`, and `strip_suffix(" passed")` on the trimmed `"ok. 3 passed"` yields `"ok. 3"`, whose `rsplit(' ').next()` is `"3"`. This is why the implementation reads the last whitespace-separated token, not the first.

- [ ] **Step 5: Commit**

```bash
git add crates/focus_forge_status/src/test_counts.rs crates/focus_forge_status/src/lib.rs
git commit -S --signoff -m "feat(status): parse cargo test summary lines"
```

---

## Task 5: `cargo_meta.rs` — parse `cargo metadata` for workspace members

**Files:**
- Create: `crates/focus_forge_status/src/cargo_meta.rs`
- Modify: `crates/focus_forge_status/src/lib.rs` (add `mod cargo_meta;`)

**Interfaces:**
- Consumes: `crate::StatusError`.
- Produces: `pub struct WorkspaceMember { pub name: String, pub kind: String }` and `pub fn parse_members(metadata_json: &str) -> Result<Vec<WorkspaceMember>, StatusError>`. Task 6 pairs this with the real `cargo metadata --no-deps --format-version 1` subprocess call.

- [ ] **Step 1: Write the failing test**

Create `crates/focus_forge_status/src/cargo_meta.rs`:

```rust
//! Parses `cargo metadata --no-deps --format-version 1` JSON output into
//! workspace member names, classifying each as a "crate" (path contains
//! /crates/) or a "lab" (path contains /labs/) per the repo's documented
//! directory layout in IMPLEMENTATION.md.

use serde::Deserialize;

use crate::StatusError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceMember {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    manifest_path: String,
}

pub fn parse_members(metadata_json: &str) -> Result<Vec<WorkspaceMember>, StatusError> {
    let metadata: Metadata = serde_json::from_str(metadata_json)
        .map_err(|e| StatusError::CargoMetadata(e.to_string()))?;

    Ok(metadata
        .packages
        .into_iter()
        .map(|p| {
            let normalized = p.manifest_path.replace('\\', "/");
            let kind = if normalized.contains("/crates/") {
                "crate"
            } else if normalized.contains("/labs/") {
                "lab"
            } else {
                "other"
            };
            WorkspaceMember {
                name: p.name,
                kind: kind.to_string(),
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{parse_members, WorkspaceMember};

    #[test]
    fn classifies_crates_and_labs_by_manifest_path() {
        let json = r#"{
            "packages": [
                { "name": "focus_forge_core", "manifest_path": "D:\\repo\\crates\\focus_forge_core\\Cargo.toml" },
                { "name": "ch00_setup", "manifest_path": "D:\\repo\\labs\\ch00_setup\\Cargo.toml" }
            ]
        }"#;

        let members = parse_members(json).unwrap();

        assert_eq!(
            members,
            vec![
                WorkspaceMember {
                    name: "focus_forge_core".to_string(),
                    kind: "crate".to_string(),
                },
                WorkspaceMember {
                    name: "ch00_setup".to_string(),
                    kind: "lab".to_string(),
                },
            ]
        );
    }

    #[test]
    fn malformed_json_errors() {
        let err = parse_members("not json").unwrap_err();

        assert!(matches!(err, crate::StatusError::CargoMetadata(_)));
    }
}
```

Add `mod cargo_meta;` to `crates/focus_forge_status/src/lib.rs`:

```rust
mod cargo_meta;
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p focus_forge_status cargo_meta`
Expected: at this point the module was written with its implementation alongside its tests, same as Tasks 2 and 3 — the logic (deserialize + classify by substring) is one cohesive unit best verified as a whole. Confirm GREEN in Step 3.

- [ ] **Step 3: Run test to verify it passes**

Run: `cargo test -p focus_forge_status cargo_meta`
Expected:
```
test cargo_meta::tests::classifies_crates_and_labs_by_manifest_path ... ok
test cargo_meta::tests::malformed_json_errors ... ok
```

- [ ] **Step 4: Commit**

```bash
git add crates/focus_forge_status/src/cargo_meta.rs crates/focus_forge_status/src/lib.rs
git commit -S --signoff -m "feat(status): parse cargo metadata workspace members"
```

---

## Task 6: `git.rs` — parse `git log` output

**Files:**
- Create: `crates/focus_forge_status/src/git.rs`
- Modify: `crates/focus_forge_status/src/lib.rs` (add `mod git;`)

**Interfaces:**
- Consumes: `crate::model::CommitInfo` (from Task 2), `crate::StatusError`.
- Produces: `pub fn parse_git_log_output(stdout: &str) -> Result<CommitInfo, StatusError>`. Task 7 pairs this with the real `git log -1 --format=%H%n%cI` subprocess call.

- [ ] **Step 1: Write the failing test**

Create `crates/focus_forge_status/src/git.rs`:

```rust
//! Parses `git log -1 --format=%H%n%cI` output (commit hash on line 1,
//! ISO 8601 commit date on line 2) into a CommitInfo.

use crate::model::CommitInfo;
use crate::StatusError;

pub fn parse_git_log_output(stdout: &str) -> Result<CommitInfo, StatusError> {
    let mut lines = stdout.lines();
    let hash = lines
        .next()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| StatusError::Git("missing commit hash line".to_string()))?
        .trim()
        .to_string();
    let date = lines
        .next()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| StatusError::Git("missing commit date line".to_string()))?
        .trim()
        .to_string();

    Ok(CommitInfo { hash, date })
}

#[cfg(test)]
mod tests {
    use super::parse_git_log_output;

    #[test]
    fn parses_hash_and_date_lines() {
        let stdout = "3448566d1ec15c7f6eeed33914f1f1eb4b76a7a6\n2026-07-08T14:17:57+02:00\n";

        let commit = parse_git_log_output(stdout).unwrap();

        assert_eq!(commit.hash, "3448566d1ec15c7f6eeed33914f1f1eb4b76a7a6");
        assert_eq!(commit.date, "2026-07-08T14:17:57+02:00");
    }

    #[test]
    fn empty_output_errors() {
        let err = parse_git_log_output("").unwrap_err();

        assert!(matches!(err, crate::StatusError::Git(_)));
    }
}
```

Add `mod git;` to `crates/focus_forge_status/src/lib.rs`:

```rust
mod git;
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p focus_forge_status git::`
Expected: written with its implementation alongside its tests (same reasoning as Tasks 2/3/5 — a short, cohesive parsing function). Confirm GREEN in Step 3.

- [ ] **Step 3: Run test to verify it passes**

Run: `cargo test -p focus_forge_status git::`
Expected:
```
test git::tests::parses_hash_and_date_lines ... ok
test git::tests::empty_output_errors ... ok
```

- [ ] **Step 4: Commit**

```bash
git add crates/focus_forge_status/src/git.rs crates/focus_forge_status/src/lib.rs
git commit -S --signoff -m "feat(status): parse git log commit info"
```

---

## Task 7: Wire `run()` — subprocess calls, assembly, and file output

**Files:**
- Modify: `crates/focus_forge_status/src/lib.rs`

**Interfaces:**
- Consumes: `status_md::parse_front_matter`, `cargo_meta::parse_members`, `test_counts::parse_test_summary`, `git::parse_git_log_output`, `model::{Status, CrateStatus}` (all prior tasks).
- Produces: the real `pub fn run(out_path: &Path) -> Result<(), StatusError>` that shells out, assembles a `Status`, and writes it as pretty JSON to `out_path`. This is the crate's only public entry point besides the types already re-exported.

This task is subprocess orchestration glued around already-tested parsers, so per Task Right-Sizing it is verified primarily by one `#[ignore]`d integration test (spec: "Testing" — "One `#[ignore]`d integration-style test may exercise the real `run()`"), not a unit test per branch.

- [ ] **Step 1: Write the (ignored) integration test first**

Replace the placeholder `pub fn run` in `crates/focus_forge_status/src/lib.rs` — first add the test at the bottom of the file, below the existing `pub fn run` stub:

```rust

#[cfg(test)]
mod run_tests {
    use super::run;

    #[test]
    #[ignore = "shells out to real git/cargo against this repo; run manually with `cargo test -p focus_forge_status -- --ignored`"]
    fn run_writes_a_status_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("status.json");

        run(&out_path).unwrap();

        let contents = std::fs::read_to_string(&out_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
        assert!(json["crates"].as_array().unwrap().len() >= 2);
        assert!(json["last_commit"]["hash"].is_string());
    }
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `cargo test -p focus_forge_status -- --ignored run_writes_a_status_json_file`
Expected: FAILS — the current `run` stub returns `Ok(())` without writing any file, so `std::fs::read_to_string(&out_path)` errors with "No such file or directory" (or similar).

- [ ] **Step 3: Write the implementation**

Replace the entire contents of `crates/focus_forge_status/src/lib.rs` with:

```rust
//! Generates a static status.json snapshot of the workspace (crates/labs,
//! test counts, last commit, phase/chapter) by shelling out to git and
//! cargo. This is a repo-maintenance tool, not part of the Focus Forge
//! product surface — see docs/superpowers/specs/2026-07-08-focus-forge-status-design.md.

mod cargo_meta;
mod git;
mod model;
mod status_md;
mod test_counts;

use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use thiserror::Error;

use model::{CommitInfo, CrateStatus, Status};

#[derive(Debug, Error)]
pub enum StatusError {
    #[error("git command failed: {0}")]
    Git(String),
    #[error("cargo metadata failed: {0}")]
    CargoMetadata(String),
    #[error("cargo test failed for {0}: {1}")]
    CargoTest(String, String),
    #[error("could not parse STATUS.md front matter: {0}")]
    StatusMd(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("json error: {0}")]
    Json(String),
}

fn run_git_log() -> Result<CommitInfo, StatusError> {
    let output = Command::new("git")
        .args(["log", "-1", "--format=%H%n%cI"])
        .output()
        .map_err(|e| StatusError::Git(e.to_string()))?;
    if !output.status.success() {
        return Err(StatusError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    git::parse_git_log_output(&stdout)
}

fn run_cargo_metadata() -> Result<Vec<cargo_meta::WorkspaceMember>, StatusError> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .map_err(|e| StatusError::CargoMetadata(e.to_string()))?;
    if !output.status.success() {
        return Err(StatusError::CargoMetadata(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    cargo_meta::parse_members(&stdout)
}

fn run_cargo_test(member: &str) -> Result<(u32, u32), StatusError> {
    let output = Command::new("cargo")
        .args(["test", "-p", member])
        .output()
        .map_err(|e| StatusError::CargoTest(member.to_string(), e.to_string()))?;
    // A crate with failing tests exits nonzero; we still want its counts,
    // so parse stdout regardless of exit status rather than erroring here.
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(test_counts::parse_test_summary(&stdout))
}

fn read_status_md_front_matter() -> Result<status_md::FrontMatter, StatusError> {
    let text = std::fs::read_to_string("STATUS.md").map_err(|e| StatusError::Io(e.to_string()))?;
    status_md::parse_front_matter(&text)
}

fn rfc3339_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Hand-rolled UTC calendar conversion avoids adding a chrono/time
    // dependency for a single timestamp field (see the design spec's
    // "Output Shape" section).
    let days = secs / 86_400;
    let time_of_day = secs % 86_400;
    let (hours, minutes, seconds) = (time_of_day / 3600, (time_of_day % 3600) / 60, time_of_day % 60);

    let mut year = 1970i64;
    let mut remaining_days = days as i64;
    loop {
        let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        let days_in_year = if is_leap { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let month_lengths: [i64; 12] = [
        31,
        if is_leap { 29 } else { 28 },
        31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
    ];
    let mut month = 1;
    for len in month_lengths {
        if remaining_days < len {
            break;
        }
        remaining_days -= len;
        month += 1;
    }
    let day = remaining_days + 1;

    format!(
        "{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z"
    )
}

pub fn run(out_path: &Path) -> Result<(), StatusError> {
    let last_commit = run_git_log()?;
    let front_matter = read_status_md_front_matter()?;
    let members = run_cargo_metadata()?;

    let mut crates = Vec::new();
    for member in &members {
        let (passed, failed) = run_cargo_test(&member.name)?;
        crates.push(CrateStatus {
            name: member.name.clone(),
            kind: member.kind.clone(),
            tests_passed: passed,
            tests_failed: failed,
        });
    }

    let status = Status {
        generated_at: rfc3339_now(),
        last_commit,
        phase: front_matter.phase,
        phase_name: front_matter.phase_name,
        chapter: front_matter.chapter,
        chapter_name: front_matter.chapter_name,
        crates,
    };

    let json = serde_json::to_string_pretty(&status).map_err(|e| StatusError::Json(e.to_string()))?;
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| StatusError::Io(e.to_string()))?;
    }
    std::fs::write(out_path, json).map_err(|e| StatusError::Io(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod run_tests {
    use super::run;

    #[test]
    #[ignore = "shells out to real git/cargo against this repo; run manually with `cargo test -p focus_forge_status -- --ignored`"]
    fn run_writes_a_status_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("status.json");

        run(&out_path).unwrap();

        let contents = std::fs::read_to_string(&out_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
        assert!(json["crates"].as_array().unwrap().len() >= 2);
        assert!(json["last_commit"]["hash"].is_string());
    }
}
```

Note: `run_cargo_test` and `read_status_md_front_matter` both assume the process's current working directory is the repository root (so `cargo test -p <member>` resolves the workspace and `STATUS.md` is found by relative path). This matches how `cargo run -p focus_forge_status` is invoked in every example in this plan and in the design spec's CLI Surface section — no `--cwd` flag is introduced, consistent with YAGNI.

- [ ] **Step 4: Run test to verify it passes**

Run: `cargo test -p focus_forge_status -- --ignored run_writes_a_status_json_file`
Expected: `test run_tests::run_writes_a_status_json_file ... ok`. This actually shells out to `git`/`cargo` and runs every workspace member's test suite, so it takes longer than the other tests (tens of seconds) — that is expected and why it is `#[ignore]`d from the default run.

- [ ] **Step 5: Run the full default test suite to confirm nothing regressed**

Run: `cargo test -p focus_forge_status`
Expected: all non-ignored tests from Tasks 2–6 still pass; `run_writes_a_status_json_file` is skipped (shown as `1 ignored`).

- [ ] **Step 6: Commit**

```bash
git add crates/focus_forge_status/src/lib.rs
git commit -S --signoff -m "feat(status): wire run() to shell out and write status.json"
```

---

## Task 8: End-to-end verification and STATUS.md handoff update

**Files:**
- Modify: `STATUS.md` (Handoff Response Template update — last verified commands, next increment)

**Interfaces:** none (verification + documentation task; no new code).

- [ ] **Step 1: Run the full workspace verification suite**

Run: `cargo fmt --check`
Expected: no diff output (clean).

Run: `cargo check --workspace`
Expected: `Finished` with no errors.

Run: `cargo test --workspace`
Expected: all tests pass, including the new `focus_forge_status` unit tests (the `#[ignore]`d one is skipped by default, which is correct).

Run: `cargo clippy --workspace --all-targets --all-features`
Expected: no warnings.

- [ ] **Step 2: Manually run the real binary against this repo**

Run: `cargo run -p focus_forge_status -- --out target/status-smoke-test.json`
Expected: exits 0. Then inspect the file:

Run (PowerShell): `Get-Content target/status-smoke-test.json`
Expected: valid JSON matching the "Output Shape" section of the design spec — `phase: 2`, `chapter: "ch00"`, a `crates` array containing `focus_forge_core`, `focus_forge_cli`, `focus_forge_status`, and `ch00_setup` with real `tests_passed` counts, and a `last_commit` with today's actual commit hash.

Delete the smoke-test file afterward since `target/` is gitignored and this was just a manual check:

Run: `rm target/status-smoke-test.json` (or `Remove-Item target/status-smoke-test.json` in PowerShell)

- [ ] **Step 3: Update STATUS.md's handoff fields**

In `STATUS.md`, update the `Last verified commit`, `Last verified commands`, and `Next recommended increment` sections to reflect this increment. Replace:

```markdown
Last verified commit: 71b8c77
Last verified commands:
- git status --short --ignored
- git log -5 --oneline
- cargo fmt --check
- cargo check
- cargo clippy --workspace --all-targets --all-features
- cargo test --workspace
- cargo run -p focus_forge_cli -- --file <tmp> project add/list/show, task add/done, note add (manual smoke test)
```

with:

```markdown
Last verified commit: <fill in the actual hash of this task's commit after Step 4>
Last verified commands:
- git status --short --ignored
- git log -5 --oneline
- cargo fmt --check
- cargo check
- cargo clippy --workspace --all-targets --all-features
- cargo test --workspace
- cargo run -p focus_forge_status -- --out target/status-smoke-test.json (manual smoke test, output verified then deleted)
```

And update the `Next recommended increment` list to add, at the top:

```markdown
Next recommended increment:
- Wire ./site to fetch and render site/static/status.json (owned separately from this increment; see docs/superpowers/specs/2026-07-08-focus-forge-status-design.md's Follow-on Work).
- Time the Chapter 0 path (Phase 1 gate target: under ten minutes) and record it.
- Chapters 1 and 2 (basics, ownership) to close out the remaining Phase 2 gate items.
- Consider the workspace export/import slice (explicitly out of scope for the focus_forge_cli slice).
```

Also add one line to the `Current repository state` bullet list, after the `focus_forge_cli` bullet:

```markdown
- focus_forge_status exists per docs/superpowers/specs/2026-07-08-focus-forge-status-design.md:
  generates site/static/status.json from git/cargo (workspace crates, test counts, last commit, phase/chapter from STATUS.md front matter)
```

- [ ] **Step 4: Commit**

```bash
git add STATUS.md
git commit -S --signoff -m "docs: update STATUS.md for focus_forge_status increment"
```

Then run `git log -1 --format=%H` to get this commit's hash, and if you want `STATUS.md`'s `Last verified commit` field to point at itself precisely, that is a cosmetic nicety only — the existing convention in this repo (see `c0f926f` vs `cc4901b` in git history) already tolerates the status doc referencing the commit just before it, so do not create an extra commit solely to update that one hash.

---

## Plan Self-Review Notes

- **Spec coverage:** Goal → Tasks 1–8 (end to end). Context/Non-Goals → enforced via Global Constraints (no `./site` changes, no new deps, no CI). Structure → Tasks 1–7 create exactly the files listed in the spec's Structure section (`main.rs`, `lib.rs`, `git.rs`, `cargo_meta.rs`, `test_counts.rs`, `status_md.rs`, `model.rs`, `tests/fixtures/`). Dependencies → Task 1's `Cargo.toml` uses only already-workspace deps. Data Sources → Tasks 4/5/6 each own one source exactly as spec'd. STATUS.md Front Matter → Task 1 Step 5. Output Shape → Task 2. CLI Surface → Task 1 Step 3 (`--out`, default `site/static/status.json`). Error Handling → Task 1 Step 3's `StatusError` matches the spec's enum verbatim. Testing → each parser task is unit-tested against fixtures; Task 7 has the one `#[ignore]`d real-subprocess test, matching the spec exactly. Follow-on Work → recorded in Task 8's `STATUS.md` update, not implemented (correctly out of scope).
- **Placeholder scan:** no TBD/TODO; every code block is complete and compilable as written.
- **Type consistency:** `StatusError` variants defined once in Task 1/finalized in Task 7 are used identically in `status_md.rs` (Task 3), `cargo_meta.rs` (Task 5), `git.rs` (Task 6). `CommitInfo`/`CrateStatus`/`Status` field names introduced in Task 2 are reused without renaming in Task 7's `run()`. `WorkspaceMember { name, kind }` from Task 5 matches its use in Task 7's `crates.push(CrateStatus { name: member.name.clone(), kind: member.kind.clone(), ... })`.
