# Assignment: ch02 Ownership, Borrowing, And References

## Goal

Make ownership concrete by tracing when Focus Forge project text is borrowed,
cloned, or moved.

## Starting Point

Files to read:

- `chapters/02-ownership.md`
- `labs/ch02_ownership/src/main.rs`
- `labs/ch02_ownership/src/lib.rs`

Files to edit:

- `labs/ch02_ownership/src/main.rs`
- `labs/ch02_ownership/src/lib.rs`
- `labs/ch02_ownership/tests/chapter_checks.rs`

## Quick Win

Run:

```powershell
cargo run -p ch02_ownership
```

You should see a project summary, a tag lookup, a clone-cost line, and an
archived project line.

## Guided Tasks

- [ ] Run `cargo check -p ch02_ownership`.
- [ ] Run `cargo test -p ch02_ownership`.
- [ ] Run `cargo run -p ch02_ownership`.
- [ ] Find every function that borrows `&str`.
- [ ] Find the function that consumes `String`.
- [ ] Find the function that creates an owned copy with `to_owned()`.
- [ ] Add the deliberate moved-value compiler error from the chapter.
- [ ] Read the first compiler error and identify the moved variable.
- [ ] Remove the deliberate error.
- [ ] Add one small borrowed helper and test it.

## Main Build Task

Refactor one place so borrowing avoids unnecessary copying.

Example path:

- Add a helper such as `project_has_word(name: &str, word: &str) -> bool`.
- Call it from `main.rs` with `&project_name`.
- Add a test in `lib.rs` or `tests/chapter_checks.rs`.
- Do not clone the project name just to search it.

The finished code should make the data flow obvious from the function
signatures.

## Acceptance Criteria

- [ ] `cargo fmt --check` passes.
- [ ] `cargo check` passes.
- [ ] `cargo test -p ch02_ownership` passes.
- [ ] `cargo clippy --workspace --all-targets --all-features` passes.
- [ ] `cargo run -p ch02_ownership` prints the ownership demo.
- [ ] You can name one borrowed value, one cloned value, and one moved value.
- [ ] You can explain why a borrow was better than a clone in one place.

## How To Verify

Run:

```powershell
cargo fmt --check
cargo check
cargo test -p ch02_ownership
cargo clippy --workspace --all-targets --all-features
cargo run -p ch02_ownership
```

## Common Errors

- Borrow of moved value: you used a value after passing ownership into a function.
- Expected `String`, found `&str`: decide whether the function should borrow or
  own the text.
- Expected `&str`, found `String`: pass `&name` when the function only needs to
  read the string.
- Unnecessary clone: ask whether the function can borrow instead.

## Recovery Path

If the borrow checker starts feeling loud:

```powershell
cargo check -p ch02_ownership
```

Read only the first error. Look for the words `moved`, `borrowed`, or
`borrow`. Then inspect the function signature involved.

## Stretch Tasks

- [ ] Add `format_tag_list(tags: &[&str]) -> String`.
- [ ] Add a test showing a borrowed `String` is still usable after the call.
- [ ] Clone a project name on purpose, then write one sentence explaining the
      cost and why the clone is acceptable there.
- [ ] Write a short comparison between Rust borrowing and object references in a
      language you already know.

## Reflection

- Which ownership rule felt useful?
- Which ownership rule felt restrictive?
- What did a function signature tell you before reading the function body?
- What would you explain to your past self about `&`?
