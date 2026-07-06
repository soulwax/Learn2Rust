# Assignment: ch00 Setup And First Run

## Goal

Prove that the local Rust learning environment works and that you can run, test, modify, break, and fix a tiny Rust program.

## Starting Point

Files to read:

- `README.md`
- `chapters/00-setup.md`
- `labs/ch00_setup/src/main.rs`
- `labs/ch00_setup/src/lib.rs`

Files to edit:

- `labs/ch00_setup/src/lib.rs`
- Optionally `labs/ch00_setup/src/main.rs`

## Quick Win

Run:

```powershell
cargo run -p ch00_setup -- YourName
```

You should see a greeting with your name.

## Guided Tasks

- [ ] Run `cargo check`.
- [ ] Run `cargo test`.
- [ ] Run `cargo run -p ch00_setup`.
- [ ] Run `cargo run -p ch00_setup -- YourName`.
- [ ] Change the fallback greeting in `greeting`.
- [ ] Update the matching test if needed.
- [ ] Run `cargo test` again.
- [ ] Intentionally create the compiler error described in `chapters/00-setup.md`.
- [ ] Read the first compiler error.
- [ ] Fix the compiler error.
- [ ] Run the verification commands again.

## Main Build Task

Change the lab so the default run feels like your own first checkpoint.

Example ideas:

- Change the fallback name.
- Change the setup status message.
- Add a tiny extra line of output in `main`.
- Add or update a test for your changed behavior.

Keep the program small. The point is to complete the loop, not to build an app yet.

## Acceptance Criteria

- [ ] `cargo fmt --check` passes.
- [ ] `cargo check` passes.
- [ ] `cargo test` passes.
- [ ] `cargo clippy --workspace --all-targets --all-features` passes.
- [ ] `cargo run -p ch00_setup -- YourName` prints a greeting.
- [ ] You can explain one compiler error you caused and fixed.

## How To Verify

Run:

```powershell
cargo fmt --check
cargo check
cargo test
cargo clippy --workspace --all-targets --all-features
cargo run -p ch00_setup -- YourName
```

## Common Errors

- `cargo` is not recognized: Rust is not installed or the terminal needs to be restarted after installing Rust.
- Package not found: make sure you are running commands from the repository root.
- Type mismatch between `String` and `&str`: restore `.to_string()` or return a `String` consistently.
- Tests fail after changing text: update the expected string in the test.

## Stretch Tasks

- [ ] Add a second command-line argument for a mood or status word.
- [ ] Add a test for trimming whitespace.
- [ ] Add a test for the exact setup status message.
- [ ] Write a one-paragraph comparison between `cargo test` and test runners you know from C#, Java, or TypeScript.

## Reflection

- What command will you run first when you come back tomorrow?
- What did the compiler explain well?
- What compiler message still felt strange?
- What made this feel like programming again?
