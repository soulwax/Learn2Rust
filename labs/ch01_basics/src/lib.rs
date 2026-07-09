// Chapter 1 is about the ordinary bricks of Rust programs: bindings,
// mutability, functions, expressions, simple numbers, booleans, and strings.
//
// This file stays deliberately small. Focus Forge will eventually have proper
// project structs, persistence, and CLI commands; here we use plain function
// arguments so the basics stay visible.

/// Returns `true` when a project name contains at least one visible character.
///
/// The input is `&str`, a borrowed string slice. That means callers can pass a
/// string literal or a borrowed `String` without giving this function ownership.
/// In C#, Java, or TypeScript you might just pass `string`; Rust makes the
/// borrowing part explicit with `&`.
pub fn project_name_is_valid(name: &str) -> bool {
    // `.trim()` gives us a borrowed view without leading or trailing whitespace.
    // `.is_empty()` returns a `bool`, Rust's true/false type.
    !name.trim().is_empty()
}

/// Calculates a whole-number completion percentage.
///
/// `u32` is an unsigned 32-bit integer. It is a good fit here because task
/// counts cannot be negative, and the exact size is visible in the type.
pub fn completion_percent(completed_tasks: u32, total_tasks: u32) -> u32 {
    // In Rust, `if` is an expression: this whole block produces the value that
    // becomes the function result. Here we use an early `return` for the special
    // zero-total case so the normal calculation stays easy to read.
    if total_tasks == 0 {
        return 0;
    }

    completed_tasks * 100 / total_tasks
}

/// Formats a small Focus Forge project summary.
///
/// This returns an owned `String` because `format!` creates new text. The caller
/// owns that returned text and can print it, store it, or test it.
pub fn format_project_summary(
    name: &str,
    description: &str,
    completed_tasks: u32,
    total_tasks: u32,
) -> String {
    let name = name.trim();
    let description = description.trim();

    // `let` creates an immutable binding by default. We are not changing either
    // fallback value after choosing it, so immutable bindings tell the truth.
    let display_name = if project_name_is_valid(name) {
        name
    } else {
        "(invalid project name)"
    };

    let display_description = if description.is_empty() {
        "No description yet."
    } else {
        description
    };

    let percent = completion_percent(completed_tasks, total_tasks);

    // `format!` works like `println!`, but returns a `String` instead of printing
    // immediately. Keeping formatting in a function makes it easy to test.
    format!(
        "Project: {display_name}\nDescription: {display_description}\nProgress: {completed_tasks}/{total_tasks} tasks ({percent}%)"
    )
}

#[cfg(test)]
mod tests {
    use super::{completion_percent, format_project_summary, project_name_is_valid};

    #[test]
    fn project_name_must_have_visible_text() {
        assert!(project_name_is_valid("Learn Rust Basics"));
        assert!(!project_name_is_valid("   "));
    }

    #[test]
    fn completion_percent_uses_integer_math() {
        assert_eq!(completion_percent(2, 5), 40);
    }

    #[test]
    fn completion_percent_handles_zero_total() {
        assert_eq!(completion_percent(0, 0), 0);
    }

    #[test]
    fn project_summary_includes_name_description_and_progress() {
        let summary = format_project_summary(
            " Learn Rust Basics ",
            "Build confidence with small functions.",
            2,
            5,
        );

        assert!(summary.contains("Project: Learn Rust Basics"));
        assert!(summary.contains("Description: Build confidence"));
        assert!(summary.contains("Progress: 2/5 tasks (40%)"));
    }
}
