pub fn greeting(name: &str) -> String {
    let name = name.trim();

    if name.is_empty() {
        "Hello, Rust!".to_string()
    } else {
        format!("Hello, {name}!")
    }
}

pub fn setup_status() -> &'static str {
    "Chapter 0 is alive. Cargo, VS Code, and you are talking."
}

#[cfg(test)]
mod tests {
    use super::{greeting, setup_status};

    #[test]
    fn greeting_uses_provided_name() {
        assert_eq!(greeting("Learner"), "Hello, Learner!");
    }

    #[test]
    fn greeting_falls_back_for_blank_names() {
        assert_eq!(greeting("   "), "Hello, Rust!");
    }

    #[test]
    fn setup_status_confirms_the_lab_runs() {
        assert!(setup_status().contains("Chapter 0"));
    }
}
