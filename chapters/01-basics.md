# Chapter 1: Variables, Functions, And Basic Types

Chapter 1 makes Rust feel like ordinary programming again before ownership gets
its own chapter.

You will use simple Focus Forge project data to practice bindings, mutability,
functions, expressions, strings, integers, booleans, formatting, and tests.

## Starting Checkpoint

Before starting, these commands should pass from the repository root:

```powershell
cargo check
cargo test
cargo run -p ch00_setup
```

## What You Will Use

- `let` and `let mut`
- Function arguments and return values
- `&str`, `String`, `u32`, and `bool`
- `if` as an expression
- `format!` and `println!`
- The `labs/ch01_basics` package

## What This Chapter Adds

- A lab that prints a single Focus Forge project summary.
- A validation function for project names.
- A small completion-percentage calculation.
- Tests for formatting and basic validation.

## OOP-To-Rust Bridge

If you come from C#, Java, or TypeScript:

- `let` is closest to a local variable declaration, but Rust makes it immutable
  by default.
- `let mut` means the binding can be reassigned. It does not mean every borrowed
  value behind it is magically mutable.
- Rust functions can live directly in a module; they do not need to be methods
  on a class.
- `String` is owned text. `&str` is borrowed text. Chapter 2 will make that
  distinction sharper.
- `if` can produce a value, so it can replace many tiny ternary expressions.

## Math Warm-Up

Treat a Rust function like a mathematical input-output mapping:

```text
completion_percent(2, 5) = 40
completion_percent(0, 0) = 0
```

For this chapter, keep the math practical:

- Count completed tasks.
- Count total tasks.
- Compute a whole-number percentage.
- Predict the output before running the program.

## Quick Win

Run:

```powershell
cargo run -p ch01_basics
```

Expected shape:

```text
Project: Learn Rust Basics
Description: Turn small functions into visible Focus Forge progress.
Progress: 2/5 tasks (40%)
Blank project names are rejected.
```

## Guided Reading

Open `labs/ch01_basics/src/lib.rs`.

Notice:

- `project_name_is_valid` returns a `bool`.
- `completion_percent` returns a `u32`.
- `format_project_summary` returns a `String`.
- The tests call library functions directly instead of running the binary.

Open `labs/ch01_basics/src/main.rs`.

Notice:

- Most bindings use plain `let`.
- Only `completed_tasks` uses `let mut`.
- The binary prints the string returned by the library.

## Deliberate Compiler Error

Open `labs/ch01_basics/src/main.rs`.

Temporarily change:

```rust
let mut completed_tasks = 1;
```

to:

```rust
let completed_tasks = 1;
```

Then run:

```powershell
cargo check -p ch01_basics
```

Rust should complain when the later line tries to assign a new value to an
immutable binding.

Fix the code by restoring `mut`, then run:

```powershell
cargo check -p ch01_basics
cargo test -p ch01_basics
```

## Main Build Idea

Make the project summary feel personal:

- Change the project name.
- Change the description.
- Change the completed and total task counts.
- Add or update a test so the expected percentage still matches.

Keep this chapter small. The win is understanding the shape of Rust code, not
building the full app yet.

## Verification

Run:

```powershell
cargo fmt --check
cargo check
cargo test -p ch01_basics
cargo clippy --workspace --all-targets --all-features
cargo run -p ch01_basics
```

## Reflection

Answer these briefly:

- Which bindings needed `mut`, and which did not?
- What does `completion_percent` do for each input?
- Where did Rust feel similar to a language you already know?
- Where did the `String` versus `&str` distinction show up?
- What compiler message did you create and fix?
