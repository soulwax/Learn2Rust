# Compiler Errors

Rust compiler errors are part of the teaching environment. The goal is not to avoid them. The goal is to learn how to read them calmly.

## The Basic Method

When `cargo check` fails:

- [ ] Read the first error first.
- [ ] Find the file and line number.
- [ ] Read what Rust expected.
- [ ] Read what Rust found.
- [ ] Make the smallest possible fix.
- [ ] Run `cargo check` again.

## Common Chapter 0 Error: `String` vs `&str`

You may see an error after changing:

```rust
"Hello, Rust!".to_string()
```

to:

```rust
"Hello, Rust!"
```

The short version:

- `String` is an owned, growable string.
- `&str` is a borrowed string slice.
- A function that promises `String` must return `String` on every path.

Fix:

```rust
"Hello, Rust!".to_string()
```

or:

```rust
String::from("Hello, Rust!")
```

## Common Chapter 0 Error: Test Text Changed

If a test says two strings are not equal, compare them carefully.

Example:

```text
left:  "Hello, Learner!"
right: "Hello, Rust!"
```

This means the code and the test disagree. Either the code is wrong, or the expected value in the test should be updated.

## Common Chapter 0 Error: Wrong Directory

If Cargo says it cannot find `Cargo.toml`, move to the repository root.

```powershell
cd D:\Workspace\Rust\LearnRust
cargo check
```

## What To Ignore At First

Early on, do not try to understand every part of a long diagnostic.

Focus on:

- [ ] File path.
- [ ] Line number.
- [ ] Expected type.
- [ ] Found type.
- [ ] The first suggested fix, if it is understandable.

The rest will become useful later.
