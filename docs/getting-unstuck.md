# Getting Unstuck

This repo is designed for a returning programmer. Getting stuck is part of the loop, not evidence that you are bad at Rust.

Use this page when the project feels noisy.

## First Recovery Loop

Run these from the repository root:

```powershell
git status --short --ignored
cargo fmt --check
cargo check
```

Then ask:

- [ ] Am I in the repository root?
- [ ] Did I edit the file the assignment asked me to edit?
- [ ] Is the first compiler error about the real problem?
- [ ] Did I accidentally leave the deliberate compiler-error exercise broken?
- [ ] Did I change output text without updating the test?

## If Cargo Cannot Find A Package

Check the package name:

```powershell
cargo run -p ch00_setup
```

Then check the workspace:

```powershell
Get-Content Cargo.toml
```

The lab should appear under `[workspace].members`.

## If Tests Fail

Read the failure in this order:

- [ ] Test name.
- [ ] Expected value.
- [ ] Actual value.
- [ ] File and line number.

Most early test failures are just text mismatches. That is useful feedback.

## If The Compiler Error Looks Huge

Do this:

- [ ] Read only the first error.
- [ ] Find the file and line number.
- [ ] Ignore suggestions below later errors until the first one is fixed.
- [ ] Run `cargo check` again after every small fix.

Rust often reports follow-up errors after the first mistake. Fixing the first error may remove the rest.

## If You Feel Rusty

Use a smaller loop:

```powershell
cargo check
```

Make one tiny edit. Run it again.

When that works:

```powershell
cargo test
```

When that works:

```powershell
cargo run -p ch00_setup
```

Momentum returns through repetition.

## If You Need To Ask For Help

Good help request:

```text
I am in Chapter 0. I changed labs/ch00_setup/src/lib.rs.
I ran cargo check.
The first error says: <paste the first error>.
I expected the function to return: <your expectation>.
```

Avoid starting with the whole output. Start with the first error.
