// This is the BINARY file (`src/main.rs`) — the runnable program.
// When you run `cargo run -p ch00_setup`, execution starts at `fn main` below.
// (Same idea as `static void Main` in C#/Java, or the top-level script that runs
// in a Node entry file.)
//
// This line imports two functions from THIS package's library (`src/lib.rs`).
// `ch00_setup` is the package/crate name (see Cargo.toml); the `::{ ... }` picks
// specific items, like `import { greeting, setup_status } from './lib'` in TS.
use ch00_setup::{greeting, setup_status};

fn main() {
    // `std::env::args()` gives an iterator over the command-line arguments.
    // By convention, argument 0 is the program's own path, so the FIRST real
    // argument the user typed is at index 1 — that's what `.nth(1)` grabs.
    //
    // `.nth(1)` returns an `Option<String>`: `Some(value)` if that argument
    // exists, or `None` if it doesn't. `Option` is Rust's answer to null — instead
    // of a value that might secretly be null (C#/Java) or `undefined` (TS), the
    // "maybe missing" case is part of the type, and the compiler forces you to
    // handle it. You literally cannot forget the empty case.
    let name = std::env::args()
        .nth(1)
        // `.unwrap_or_else(...)` says: "if it's `Some`, use the value inside;
        // if it's `None`, run this closure to produce a fallback instead."
        // The `|| ...` is a closure (a lambda / arrow function with no parameters).
        // So: use the name the user passed, otherwise default to "Rust".
        .unwrap_or_else(|| "Rust".to_string());

    // `println!` prints a line to standard output (it's a macro — hence the `!`).
    // `{}` is a placeholder filled by the argument that follows, like
    // `Console.WriteLine`, `System.out.println`, or `console.log`.
    println!("{}", greeting(&name));

    // We pass `&name` (a borrow) to `greeting` above: we lend it the string to read
    // rather than giving ownership away. `setup_status` takes no arguments and just
    // returns the fixed status line.
    println!("{}", setup_status());
}
