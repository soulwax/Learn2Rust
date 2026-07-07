# Design: focus_forge_cli (Phase 2 CLI slice)

Date: 2026-07-07
Status: Approved

## Goal

Build `crates/focus_forge_cli`, a command-line front end that drives
`focus_forge_core`. It gives the learner fast feedback on the domain crate
before any GUI exists: create projects, add and complete tasks, add notes, and
view them — all persisted to a JSON workspace file. The CLI owns argument
parsing and terminal output only; every domain rule lives in `focus_forge_core`.

This is the second half of Phase 2 ("First Product Slice") from
`IMPLEMENTATION.md`. The core crate already exists and is merged.

## Dependencies

Added to root `[workspace.dependencies]` and referenced with `.workspace = true`:

- `clap` (features = ["derive"]) — argument and subcommand parsing

Crate dependencies:

- `focus_forge_core` — path dependency, `{ path = "../focus_forge_core" }`
- `clap` — `{ workspace = true }`

Dev-dependencies (test-only):

- `assert_cmd` — run the built binary in integration tests
- `predicates` — assert on stdout/stderr/exit code
- `tempfile` — isolated workspace files

`assert_cmd`, `predicates`, and `tempfile` are added to
`[workspace.dependencies]` so versions stay centralized.

## Dependency Ordering Decision

`IMPLEMENTATION.md`'s Dependency Introduction Plan schedules `clap` at
Chapter 8. This crate introduces it at Phase 2, ahead of that slot — the same
build-to-end-state-first tradeoff recorded for serde/thiserror in ADR 0002.

Resolution: amend ADR 0002 (or add a one-line note) so its rationale explicitly
covers `clap` too — the chapter numbers describe when the *learner is taught* a
dependency, not when it first appears in product code. Update the Dependency
Introduction Plan note accordingly. No new ADR is needed; this is the same
decision applied to one more crate.

## Structure and Module Layout

```
crates/focus_forge_cli/
  Cargo.toml
  src/
    main.rs       # thin binary entry: parse args, call run(), map error to exit code
    lib.rs        # Cli/Command clap types, CliError, pub fn run(cli) -> Result<(), CliError>
    commands.rs   # one handler per command; the only place that loads/mutates/saves
    output.rs     # pure formatting: domain refs in, String out (no I/O, no domain logic)
  tests/
    cli.rs        # assert_cmd integration tests against the built binary
```

The crate has BOTH a library target (`lib.rs`) and a binary target (`main.rs`).
`main.rs` is trivial so the real logic in `lib.rs`/`commands.rs`/`output.rs` can
be unit-tested directly, while `tests/cli.rs` exercises the whole binary.

Dependency direction: `focus_forge_cli` → `focus_forge_core`, never the reverse.
`output.rs` must not call domain mutators or do I/O; it only formats.

The crate is student-facing, so all source follows the Teaching Comment Style in
`IMPLEMENTATION.md` (rich `///` and inline `//` notes with C#/Java/TypeScript
bridges — e.g. clap derive attributes ↔ annotations/decorators, `run() -> Result`
↔ a single fallible entry point).

## Command Surface (clap derive)

Global option, available on every command:

- `--file <path>` (default `workspace.json`) — the JSON workspace file.

```
focus-forge project add <id> <name>
focus-forge project list
focus-forge project show <id>
focus-forge task add <project-id> <task-id> <title> [--priority low|medium|high]
focus-forge task done <project-id> <task-id>
focus-forge note add <project-id> <note-id> <text>
```

clap types:

```rust
#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = "workspace.json")]
    file: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Project { #[command(subcommand)] action: ProjectAction },
    Task    { #[command(subcommand)] action: TaskAction },
    Note    { #[command(subcommand)] action: NoteAction },
}
```

`--priority` maps to `focus_forge_core::Priority` and defaults to `medium`. IDs
are supplied by the caller (matching core's String-ID model).

## Persistence Flow and Bootstrap

- Mutating commands (`project add`, `task add`, `task done`, `note add`):
  **load → operate → save** to `--file`.
- Read commands (`project list`, `project show`): **load → display**.

Bootstrap rule: `project add` is the ONLY command that bootstraps. If `--file`
does not exist, `project add` creates a fresh `Workspace` (named `"Focus Forge"`),
adds the project, and saves — otherwise the first project could never be created.
Every other command (`project list`, `project show`, `task add`, `task done`,
`note add`) requires an existing file: on a missing file they return
`CliError::WorkspaceNotFound` describing that the workspace file was not found and
suggesting `project add`. (This is unambiguous even for the mutating task/note
commands: their target project cannot exist in a file that does not exist.)

Timestamps: the CLI passes empty strings for `created_at` on notes and for
project timestamps, consistent with `focus_forge_core`'s current no-`chrono`
stance. This is a documented limitation to revisit when a time dependency is
introduced (a later chapter). It does not affect validation, which only checks
non-blank ids/text.

## Output

- `project list`: one line per project — `id  name  [status]  (N tasks, M notes)`.
  Empty workspace prints a friendly `No projects yet. Add one with: project add <id> <name>`.
- `project show <id>`: a header line for the project, then its tasks
  (`[x]`/`[ ]  <priority>  <title>`) and notes (`- <text>`). Unknown id errors.
- All formatting lives in `output.rs` as functions returning `String`;
  `commands.rs` and `main.rs` do the printing. This keeps formatting unit-testable.

## Error Handling

```rust
#[derive(Debug, Error)]
enum CliError {
    #[error(transparent)]
    Core(#[from] CoreError),
    #[error("workspace file not found: {0} (create a project first with: project add <id> <name>)")]
    WorkspaceNotFound(String),
}
```

`run(cli) -> Result<(), CliError>` returns errors upward. `main.rs`:

```rust
fn main() {
    let cli = Cli::parse();
    if let Err(e) = focus_forge_cli::run(cli) {
        eprintln!("error: {e}");
        std::process::exit(1);
    }
}
```

clap handles bad/missing arguments and `--help` itself (exit code 2). Domain
errors (`CoreError::UnknownProject`, `DuplicateId`, validation) surface as
`error: <message>` on stderr with exit code 1.

## Testing

- Unit tests in `output.rs`: format a known in-memory `Workspace` and assert the
  exact strings (list line shape, `[x]`/`[ ]` markers, empty-workspace message).
- Unit tests in `commands.rs`: run a handler against a `tempfile` path, reload the
  file, assert the mutation persisted (e.g. `project add` then the workspace on
  disk contains the project; `task done` sets `done`).
- Integration tests in `tests/cli.rs` (`assert_cmd` + `predicates` + `tempfile`):
  - `project add` then `project list` shows the project.
  - Bootstrap: `project add` against a nonexistent `--file` creates it.
  - `task add` + `task done` + `project show` shows `[x]`.
  - `note add` then `project show` shows the note.
  - Unknown project id exits non-zero with an error message on stderr.
  - A missing-file read command (`project list`) errors clearly.

No test depends on network access or a fixed system clock; all use temp files.

## Out of Scope (YAGNI)

- `workspace export` / `import` — a later slice.
- `task list` as a standalone command (tasks show under `project show`).
- Editing or deleting projects/tasks/notes.
- Search / filtering (waits on `search.rs` in core).
- Real timestamp generation (`chrono`) — deferred.
- Colored/table output — plain text for now.

## Follow-on Work After This Slice

- Add `crates/focus_forge_cli` to root `Cargo.toml` members.
- Amend ADR 0002 + the Dependency Introduction Plan for `clap` (see Dependency
  Ordering Decision).
- Update `STATUS.md`: Phase 2 gate ("CLI can print a project summary") is then
  met; next increment points toward Phase 3 (persistence chapters) or the
  export/import slice.
- Consider a chapters/assignments entry once the CLI is stable (curriculum work,
  separate from this crate).
