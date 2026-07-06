// This is a LIBRARY file (`src/lib.rs`). It holds reusable logic.
// Think of it like a class library / package that other code imports —
// as opposed to `src/main.rs`, which is the runnable program (the entry point).
//
// A Rust package can have BOTH a library (`lib.rs`) and a binary (`main.rs`).
// The binary here calls into this library. Keeping logic in the library means
// it can be unit-tested directly, which is exactly what the `tests` module below does.

/// Builds a friendly greeting for the given `name`.
///
/// This is a "doc comment" (three slashes, `///`). Rust treats it specially:
/// rust-analyzer shows it when you hover the function in VS Code, and
/// `cargo doc` turns it into HTML documentation. It's like an XML doc comment
/// (`/// <summary>` in C#) or a Javadoc/JSDoc block — but built into the language.
///
/// - `name: &str` — a *borrowed* string. The `&` means "a reference": we can READ
///   the caller's text but we don't own it and won't free it. In C#/Java/TS you
///   pass a `string` and the garbage collector sorts out lifetime; in Rust the
///   `&` says explicitly "just lend it to me."
/// - `-> String` — we return an *owned*, heap-allocated `String` that the caller
///   now owns. So: borrow a view in, hand back a fresh owned value out.
pub fn greeting(name: &str) -> String {
    // `.trim()` returns a NEW `&str` slice with surrounding whitespace removed.
    // It does not modify the caller's string (Rust strings aren't edited in place
    // like this). We shadow the old `name` with the trimmed one — "shadowing" means
    // reusing the same variable name for a new value. Perfectly normal in Rust.
    let name = name.trim();

    // Notice there is NO `return` keyword below. In Rust, the LAST expression in a
    // block is the value the block evaluates to. An `if/else` is itself an
    // expression (like a ternary `cond ? a : b`, but for whole blocks), so whichever
    // branch runs produces the function's return value.
    //
    // The catch: both branches must produce the SAME type — here, a `String`.
    // (Try making one branch return a bare `&str` and `cargo check` will refuse.
    // That is the deliberate compiler-error exercise in chapters/00-setup.md.)
    if name.is_empty() {
        // A bare "..." literal is a `&'static str` (borrowed text baked into the
        // binary), NOT a `String`. `.to_string()` allocates and gives us an owned
        // `String` so this branch's type matches the `-> String` promise above.
        "Hello, Rust!".to_string()
    } else {
        // `format!` builds a `String` from a template, like C#'s `$"Hello, {name}!"`
        // or a JS template literal. The `{name}` reads the local variable directly.
        format!("Hello, {name}!")
    }
}

/// Returns a fixed status line proving the lab ran.
///
/// The return type is `&'static str`, not `String`. `'static` is a *lifetime*:
/// it means this text lives for the entire program, because the literal is stored
/// in the compiled binary itself. So there is nothing to allocate and nothing to
/// free — we can hand back the borrowed slice safely and cheaply.
pub fn setup_status() -> &'static str {
    "Chapter 0 is alive. Cargo, VS Code, and you are talking."
}

// `#[cfg(test)]` means "only compile the code below when running `cargo test`."
// In a normal `cargo build`/`cargo run`, this whole module is skipped, so tests
// add zero weight to the shipped program. This is Rust's built-in test framework —
// no separate NUnit/JUnit/Jest project needed; tests live next to the code.
#[cfg(test)]
mod tests {
    // Tests are in their own module, so they must import the functions they test.
    // `super::` means "the parent module" — i.e. the top of this file.
    use super::{greeting, setup_status};

    // `#[test]` marks a function as a test case. `cargo test` finds and runs every
    // function with this attribute. (Compare `[Test]` in NUnit / `@Test` in JUnit.)
    #[test]
    fn greeting_uses_provided_name() {
        // `assert_eq!` fails the test (and prints both values) if the two sides
        // differ. The `!` means it's a macro, not a function call.
        assert_eq!(greeting("Learner"), "Hello, Learner!");
    }

    #[test]
    fn greeting_falls_back_for_blank_names() {
        // Whitespace-only input trims to empty, so we expect the fallback greeting.
        // This is what proves the `.trim()` + `is_empty()` logic actually works.
        assert_eq!(greeting("   "), "Hello, Rust!");
    }

    #[test]
    fn setup_status_confirms_the_lab_runs() {
        // `assert!` fails unless its expression is `true`. Here we check the status
        // line mentions "Chapter 0" rather than pinning the exact wording, so small
        // message tweaks don't break the test.
        assert!(setup_status().contains("Chapter 0"));
    }
}
