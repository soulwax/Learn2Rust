// Chapter 1's binary is intentionally tiny: set a few values, call a library
// function, and print the result. The interesting part is how normal Rust code
// is shaped before ownership gets more attention in Chapter 2.

use ch01_basics::{format_project_summary, project_name_is_valid};

fn main() {
    // Immutable bindings use `let`. These are like local variables in C#/Java/TS,
    // except Rust makes immutability the default.
    let project_name = "Learn Rust Basics";
    let description = "Turn small functions into visible Focus Forge progress.";

    // Add `mut` only when a binding really needs to change. This is not the same
    // as "the object is mutable" in many OOP languages; it means this local
    // binding may be assigned a new value.
    let mut completed_tasks = 1;
    completed_tasks += 1;

    let total_tasks = 5;

    println!(
        "{}",
        format_project_summary(project_name, description, completed_tasks, total_tasks)
    );

    if !project_name_is_valid("   ") {
        println!("Blank project names are rejected.");
    }
}
