use ch01_basics::{completion_percent, format_project_summary, project_name_is_valid};

#[test]
fn chapter_1_public_api_formats_a_visible_project_summary() {
    let summary = format_project_summary(
        "Build Momentum",
        "Practice basic Rust with a useful little output.",
        1,
        4,
    );

    assert!(project_name_is_valid("Build Momentum"));
    assert_eq!(completion_percent(1, 4), 25);
    assert!(summary.contains("Project: Build Momentum"));
    assert!(summary.contains("Progress: 1/4 tasks (25%)"));
}
