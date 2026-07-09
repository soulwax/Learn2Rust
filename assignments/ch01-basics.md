# Assignment: ch01 Variables, Functions, And Basic Types

## Goal

Practice Rust's basic program shape by changing and testing a tiny Focus Forge
project summary.

## Starting Point

Files to read:

- `chapters/01-basics.md`
- `labs/ch01_basics/src/main.rs`
- `labs/ch01_basics/src/lib.rs`

Files to edit:

- `labs/ch01_basics/src/main.rs`
- `labs/ch01_basics/src/lib.rs`

## Quick Win

Run:

```powershell
cargo run -p ch01_basics
```

You should see a project summary and a line confirming that blank project names
are rejected.

## Guided Tasks

- [ ] Run `cargo check -p ch01_basics`.
- [ ] Run `cargo test -p ch01_basics`.
- [ ] Run `cargo run -p ch01_basics`.
- [ ] Change the hard-coded project name in `main.rs`.
- [ ] Change the description in `main.rs`.
- [ ] Change `completed_tasks` and `total_tasks`.
- [ ] Predict the percentage before running the program.
- [ ] Run the program and compare your prediction with the output.
- [ ] Add one assertion to an existing test in `lib.rs`.
- [ ] Intentionally create and fix the `mut` compiler error from the chapter.

## Main Build Task

Add one small formatting improvement to the project summary.

Example ideas:

- Add a `Status: practicing` line.
- Add an `Estimated remaining tasks: N` line.
- Change the invalid-name fallback text.
- Add a function that formats only the progress line, then call it from
  `format_project_summary`.

Keep the change small enough that `cargo test -p ch01_basics` still tells you
quickly whether it works.

## Acceptance Criteria

- [ ] `cargo fmt --check` passes.
- [ ] `cargo check` passes.
- [ ] `cargo test -p ch01_basics` passes.
- [ ] `cargo clippy --workspace --all-targets --all-features` passes.
- [ ] `cargo run -p ch01_basics` prints your project summary.
- [ ] You can explain when you used `let` and when you used `let mut`.
- [ ] You can describe `completion_percent` as an input-output mapping.

## How To Verify

Run:

```powershell
cargo fmt --check
cargo check
cargo test -p ch01_basics
cargo clippy --workspace --all-targets --all-features
cargo run -p ch01_basics
```

## Common Errors

- Cannot assign twice to immutable variable: add `mut` only to the binding that
  really changes.
- Mismatched types between `String` and `&str`: check whether you are returning
  owned text or borrowed text.
- Test expected output no longer matches: update the test to match the behavior
  you intentionally changed.
- Division by zero: keep the zero-total branch in `completion_percent`.

## Recovery Path

If things get tangled:

```powershell
cargo fmt
cargo check -p ch01_basics
cargo test -p ch01_basics
```

Read the first compiler error only. Fix that one, then run `cargo check` again.

## Stretch Tasks

- [ ] Make `completion_percent(7, 5)` cap at 100 and add a test.
- [ ] Add a second project summary in `main.rs`.
- [ ] Add a `remaining_tasks` function and test it.
- [ ] Write a short comparison between Rust's immutable `let` and `const` or
      `readonly` concepts from a language you know.

## Reflection

- Which function was easiest to understand as input-output?
- Which type felt most familiar?
- Which type felt most Rust-specific?
- What did the compiler prevent you from doing?
