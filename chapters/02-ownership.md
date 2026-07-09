# Chapter 2: Ownership, Borrowing, And References

Chapter 2 introduces the part of Rust that feels the least like C#, Java, or
TypeScript at first: values have owners, functions can borrow values, and moving
a value is an intentional transfer.

The goal is not to master every ownership rule. The goal is to see ownership as
plain data flow.

## Starting Checkpoint

Before starting, these commands should pass from the repository root:

```powershell
cargo check
cargo test -p ch01_basics
cargo run -p ch01_basics
```

## What You Will Use

- Owned `String`
- Borrowed `&str`
- Borrowed slices such as `&[&str]`
- Moves into functions
- Explicit clones
- Function signatures as data-flow documentation
- The `labs/ch02_ownership` package

## What This Chapter Adds

- A lab that borrows project text without taking ownership.
- A function that intentionally consumes a project name.
- A function that clones only when an owned copy is useful.
- A tag lookup that scans a borrowed slice.
- Tests that prove borrowed values remain usable.

## OOP-To-Rust Bridge

If you come from C#, Java, or TypeScript:

- Passing a `String` by value is not just passing a garbage-collected object
  reference. In Rust, ownership moves unless the type is copied.
- Passing `&name` means "lend this value temporarily." The caller keeps it.
- Cloning is explicit. That is useful because copying data has cost.
- A function signature tells you whether the function reads, takes over, or
  produces new data.
- Borrowing is not "raw pointers in disguise." In safe Rust, a borrow is checked
  by the compiler so temporary access stays valid.

## Math Warm-Up

Think about project tags as a small set:

```text
tags = {"rust", "ownership", "quick-win"}
"ownership" is a member of tags
"blocked" is not a member of tags
```

Now connect copying to size:

```text
"Learn Rust Ownership" has 20 bytes
one clone copies 20 bytes
ten clones copy 200 bytes
```

The numbers are tiny here. The habit matters when the data becomes a whole
workspace instead of one project name.

## Quick Win

Run:

```powershell
cargo run -p ch02_ownership
```

Expected shape:

```text
Learn Rust Ownership: Borrow first, move deliberately, clone consciously.
Project name has 20 visible characters.
Cloned template name 'Learn Rust Ownership' (20 bytes copied).
Has ownership tag: true
Archived project: Throwaway Prototype
```

## Guided Reading

Open `labs/ch02_ownership/src/lib.rs`.

Notice:

- `project_name_length` borrows `&str`.
- `borrowed_project_summary` borrows both inputs and returns a new `String`.
- `archive_project` takes `String`, so it consumes the value passed into it.
- `duplicate_project_name_for_template` uses `to_owned()` to make a conscious
  clone.
- `has_tag` scans `&[&str]`, a borrowed view of a list.

Open `labs/ch02_ownership/src/main.rs`.

Notice:

- `project_name` is borrowed several times and stays usable.
- `template_name` is a cloned owned copy.
- `project_to_archive` is moved into `archive_project`.

## Deliberate Compiler Error

Open `labs/ch02_ownership/src/main.rs`.

After this line:

```rust
println!("{}", archive_project(project_to_archive));
```

temporarily add:

```rust
println!("{project_to_archive}");
```

Then run:

```powershell
cargo check -p ch02_ownership
```

Rust should complain that `project_to_archive` was moved. Read the first error
carefully: it is the compiler showing the exact ownership transfer.

Fix the code by removing that extra `println!`, then run:

```powershell
cargo check -p ch02_ownership
cargo test -p ch02_ownership
```

## Main Build Idea

Add one small borrowed helper.

Example ideas:

- `starts_with_tag(tags: &[&str], prefix: &str) -> bool`
- `project_has_word(name: &str, word: &str) -> bool`
- `format_tag_list(tags: &[&str]) -> String`

Keep the helper focused. It should borrow its inputs unless it truly needs an
owned `String`.

## Verification

Run:

```powershell
cargo fmt --check
cargo check
cargo test -p ch02_ownership
cargo clippy --workspace --all-targets --all-features
cargo run -p ch02_ownership
```

## Reflection

Answer these briefly:

- Which functions borrowed data?
- Which function consumed data?
- Where did a clone make sense?
- Where would cloning be wasteful?
- How is a Rust borrow different from the object references you already know?
