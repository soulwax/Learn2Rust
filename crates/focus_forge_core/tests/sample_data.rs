//! Integration test: the committed curriculum sample data must load and
//! validate through the real domain types. This freezes the JSON format as a
//! contract — if the model and the sample ever drift, this test fails.
//!
//! `env!("CARGO_MANIFEST_DIR")` is the crate directory at compile time; we walk
//! up to the repo root to reach `sample_data/`.

use std::path::PathBuf;

use focus_forge_core::{load_workspace, Priority, ProjectStatus};

fn sample_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("sample_data")
        .join("demo_workspace.json")
}

#[test]
fn demo_workspace_loads_and_validates() {
    let ws = load_workspace(&sample_path()).expect("sample data should load and validate");
    assert_eq!(ws.name, "Focus Forge Demo Workspace");
    assert_eq!(ws.projects.len(), 2);

    let learn = ws
        .project("proj-learn-rust")
        .expect("proj-learn-rust present");
    assert_eq!(learn.status, ProjectStatus::Active);
    assert_eq!(learn.tasks.len(), 3);

    let high = learn
        .tasks
        .iter()
        .find(|t| t.id == "task-run-ch00")
        .expect("task-run-ch00 present");
    assert_eq!(high.priority, Priority::High);
    assert!(high.done);
}
