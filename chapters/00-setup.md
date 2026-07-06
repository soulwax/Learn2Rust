# Chapter 0: Setup And First Run

Chapter 0 proves that the repository, Rust toolchain, Cargo, and VS Code can all work together.

The goal is not to learn a lot of Rust yet. The goal is to get one small program running, change it, test it, and see the compiler give useful feedback.

## What You Will Use

- `cargo check`
- `cargo test`
- `cargo run`
- `cargo fmt`
- VS Code task runner
- The `labs/ch00_setup` package

## What This Chapter Adds

- A first runnable Rust lab.
- A small library function with tests.
- A command-line program that prints a greeting.
- A safe place to intentionally create and fix a compiler error.

## OOP-To-Rust Bridge

If you come from C#, Java, or TypeScript:

- A Rust package is similar in spirit to a small project or package.
- `src/main.rs` is the program entry point for a binary.
- `src/lib.rs` is reusable library code.
- `cargo test` plays the role of a test runner.
- `cargo check` is a fast compiler pass that catches mistakes without building a final executable.

Rust will feel stricter than many languages at first. In this chapter, that strictness is useful: it gives fast feedback before the project becomes complicated.

## Math Warm-Up

Keep this tiny:

- Count how many tests run.
- Count how many commands succeed.
- Notice the ratio of passing tests to total tests.

For this chapter, success is simple:

```text
passing tests / total tests = 3 / 3 = 100%
```

## Quick Win

From the repository root, run:

```powershell
cargo run -p ch00_setup
```

Expected output:

```text
Hello, Rust!
Chapter 0 is alive. Cargo, VS Code, and you are talking.
```

Now pass your own name:

```powershell
cargo run -p ch00_setup -- YourName
```

Expected shape:

```text
Hello, YourName!
Chapter 0 is alive. Cargo, VS Code, and you are talking.
```

## Verify The Lab

Run:

```powershell
cargo fmt --check
cargo check
cargo test
cargo clippy --workspace --all-targets --all-features
```

All commands should pass.

## Deliberate Compiler Error

Open `labs/ch00_setup/src/lib.rs`.

Temporarily change this:

```rust
"Hello, Rust!".to_string()
```

to this:

```rust
"Hello, Rust!"
```

Then run:

```powershell
cargo check
```

Rust should complain that the function promised to return a `String`, but this branch returns `&str`.

Fix the code by restoring `.to_string()`, then run:

```powershell
cargo check
cargo test
```

## Reflection

Answer these briefly:

- What did `cargo check` catch?
- What did `cargo test` prove?
- What did `cargo run` show that tests alone did not?
- Which part felt familiar from your previous programming background?
- Which part felt Rust-specific?
