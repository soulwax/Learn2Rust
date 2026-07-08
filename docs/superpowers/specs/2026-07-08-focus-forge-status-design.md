# Design: focus_forge_status (repo status JSON generator)

Date: 2026-07-08
Status: Approved

## Goal

Generate a single static `status.json` file containing real, repo-derived
facts about the course's progress — workspace crates/labs, their test
counts, the last commit, and the current phase/chapter — so the SvelteKit
site under `./site` can display live-looking progress without a running
server. This is the "web server" requirement reinterpreted for the site's
actual deployment model (see Context below).

## Context: Why Not A Live HTTP Server

`./site` (owned by a separate in-progress effort, not this crate) is a
SvelteKit app configured with `@sveltejs/adapter-static`: it prerenders to
plain HTML/CSS/JS for GitHub Pages and has no server at runtime. A live Rust
API server (axum/actix, hosted somewhere persistent) would require a new
deployment model the site doesn't have today, a new dependency class (async
runtime, web framework) far ahead of the curriculum's schedule, and ongoing
hosting — out of proportion to "show the site some status data."

Generating a static JSON file achieves the same user-visible outcome — the
site shows real progress — while fitting the site's existing static-hosting
model exactly. If a later chapter wants a real live API server, this slice
does not preclude it; it only solves today's actual need.

## Non-Goals (YAGNI)

- No HTTP server, no `axum`/`actix`/`tokio` dependency.
- No CI wiring in this slice (`IMPLEMENTATION.md` already establishes "CI
  only after local commands are stable" — this generator is a new local
  command first).
- No changes anywhere under `./site` — that tree belongs to concurrent,
  separate work. This slice only produces a JSON file; wiring the site to
  fetch and render it is a later, separate increment.
- No historical/trend data — one current snapshot per run.
- No network calls beyond local `git`/`cargo` subprocesses.

## Structure

```
crates/focus_forge_status/
  Cargo.toml
  src/
    main.rs       # thin binary entry: parse --out <path>, call run(), print errors
    lib.rs        # StatusError, pub fn run(out_path) -> Result<(), StatusError>
    git.rs        # shells out to `git log`, parses hash + date
    cargo_meta.rs # shells out to `cargo metadata`, parses workspace members
    test_counts.rs# shells out to `cargo test -p <member>`, parses summary line
    status_md.rs  # parses STATUS.md's YAML front matter for phase/chapter
    model.rs      # Status/CrateStatus structs (Serialize) — the JSON shape
  tests/
    fixtures/     # sample STATUS.md, sample cargo-test stdout, for parser tests
```

This is a standalone binary crate, NOT a subcommand of `focus_forge_cli`.
`focus_forge_cli` is learner-facing product code (the Focus Forge app);
`focus_forge_status` is a repo-maintenance tool that inspects the workspace
itself. Keeping them separate avoids the CLI's command surface growing
tooling commands a learner would never use, and avoids `focus_forge_cli`
depending on `git`/`cargo` subprocess plumbing it has no other reason to have.

Dependency direction: `focus_forge_status` depends on nothing else in the
workspace — it inspects the workspace from the outside via subprocess calls
and file reads, so it has no path dependency on `focus_forge_core` or
`focus_forge_cli`.

## Dependencies

Added to root `[workspace.dependencies]`:

- `serde` / `serde_json` — already present; reused to parse `cargo metadata`
  output and serialize the `Status` struct.

No new dependency is required beyond what the workspace already has.
`git`/`cargo` are invoked as subprocesses via `std::process::Command`
(standard library only).

## Data Sources And Gathering Strategy

Per the approved decision: shell out to `git`/`cargo` rather than
reimplementing their parsing logic.

- **Last commit**: `git log -1 --format=%H%n%cI` (hash on line 1, ISO 8601
  commit date on line 2). Parsed by splitting on the newline.
- **Workspace members**: `cargo metadata --no-deps --format-version 1`,
  parsed with `serde_json` into `packages[].name` and `packages[].manifest_path`.
  A member's `kind` (`"crate"` vs `"lab"`) is derived from whether its
  manifest path contains `/crates/` or `/labs/` — both directories are part
  of the documented repo structure, so this is a stable signal, not a guess.
- **Test counts per member**: run `cargo test -p <member>` and scan stdout
  for the line matching `test result: ok. N passed; M failed` (or the
  `FAILED` variant). A workspace member can print more than one such line
  (unit tests, integration tests, doctests) — sum passed/failed across all
  matches for that invocation. No custom test harness or `--format=json`
  (unstable off-nightly) is used.
- **Phase / chapter**: parsed from a small YAML front-matter block added to
  the top of `STATUS.md` (see below). `STATUS.md` remains human-first prose;
  the front matter is the one small structured concession that makes
  phase/chapter machine-readable without scraping prose.

## STATUS.md Front Matter

Add a minimal front-matter block to the top of `STATUS.md`:

```markdown
---
phase: 2
phase_name: First Product Slice
chapter: ch00
chapter_name: Setup And First Run
---

# Project Status
...
```

`focus_forge_status` parses this block (text between the first two `---`
lines) as YAML-ish `key: value` pairs — no `serde_yaml` dependency needed
for four scalar fields; a simple line-splitting parser suffices and is
covered by a unit test against a fixture file. The rest of `STATUS.md` is
untouched prose, exactly as maintained today per the Handoff Response
Template in `IMPLEMENTATION.md`.

This repository's `STATUS.md` gets this front matter added as part of this
slice's implementation (a small, mechanical edit, not a rewrite).

## Output Shape

```json
{
  "generated_at": "2026-07-08T00:00:00Z",
  "last_commit": {
    "hash": "3448566d1ec15c7f6eeed33914f1f1eb4b76a7a6",
    "date": "2026-07-08T14:17:57+02:00"
  },
  "phase": 2,
  "phase_name": "First Product Slice",
  "chapter": "ch00",
  "chapter_name": "Setup And First Run",
  "crates": [
    { "name": "focus_forge_core", "kind": "crate", "tests_passed": 23, "tests_failed": 0 },
    { "name": "focus_forge_cli", "kind": "crate", "tests_passed": 25, "tests_failed": 0 },
    { "name": "ch00_setup", "kind": "lab", "tests_passed": 1, "tests_failed": 0 }
  ]
}
```

`generated_at` is produced with `std::time::SystemTime` formatted as RFC
3339 by hand (no `chrono`/`time` dependency — consistent with the rest of
the workspace's no-time-crate-yet stance; a fixed-format UTC stamp needs no
timezone library).

## CLI Surface

```
focus-forge-status --out <path>
```

- `--out <path>` (default `site/static/status.json`) — where to write the
  JSON file. Defaulting into `site/static/` means a plain `pnpm build` in
  `./site` picks it up automatically (SvelteKit serves `static/` verbatim)
  once a later increment adds the fetch — but this slice does not depend on
  that happening, and works identically if `--out` points anywhere else.
- No other flags. If a future need arises (e.g. `--pretty`), add it then.

## Error Handling

```rust
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
```

`main.rs` follows the same pattern as `focus_forge_cli`: `run() -> Result<(),
StatusError>`, `main` prints `error: {e}` to stderr and exits 1 on failure.

## Testing

- Unit tests for `status_md.rs`: parse a fixture front-matter block, assert
  the four fields extract correctly; assert a missing/malformed block
  produces `StatusError::StatusMd`.
- Unit tests for `test_counts.rs`: given fixture stdout strings containing
  one or more `test result: ...` lines (including a `FAILED` case and a
  multi-line case simulating unit + integration + doctest output), assert
  the summed pass/fail counts.
- Unit tests for `cargo_meta.rs`: given a fixture JSON blob shaped like
  `cargo metadata` output, assert workspace members are extracted with the
  right `kind`.
- Unit tests for `model.rs`: build a `Status` from parts, assert it
  serializes to the documented shape.
- No test shells out to real `git`/`cargo` — all parsing logic is tested
  against fixture strings, so tests are fast and hermetic. One `#[ignore]`d
  integration-style test may exercise the real `run()` against this repo's
  own workspace as a manual sanity check, not part of the default `cargo
  test` run.

## Follow-on Work After This Slice

- A later increment wires `./site` to fetch and render `status.json` (owned
  separately, once the concurrent site work has landed).
- A later increment may wire `focus_forge_status` into CI or a pre-build
  step once local commands are stable (per `IMPLEMENTATION.md`'s existing
  CI ordering rule).
- If a genuine need for live (not build-time) status ever arises, a real API
  server becomes its own design — this slice deliberately does not block
  that path, it just doesn't build it prematurely.
