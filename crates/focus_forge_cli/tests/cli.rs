//! Integration tests: run the built `focus-forge` binary via `assert_cmd`
//! and assert on stdout/stderr/exit code with `predicates`. Unlike the unit
//! tests in `commands.rs`/`output.rs`, these exercise the whole process —
//! argument parsing, dispatch, and printing together.

use assert_cmd::Command;
use predicates::prelude::*;

fn cmd() -> Command {
    Command::cargo_bin("focus-forge").unwrap()
}

#[test]
fn project_add_then_list_shows_the_project() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("workspace.json");

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "add", "p1", "Learn Rust"])
        .assert()
        .success();

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "list"])
        .assert()
        .success()
        .stdout(predicate::str::contains("p1").and(predicate::str::contains("Learn Rust")));
}

#[test]
fn project_add_bootstraps_a_nonexistent_file() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("does-not-exist-yet.json");
    assert!(!file.exists());

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "add", "p1", "Learn Rust"])
        .assert()
        .success();

    assert!(file.exists());
}

#[test]
fn task_add_then_done_then_show_marks_it_complete() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("workspace.json");
    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "add", "p1", "Learn Rust"])
        .assert()
        .success();
    cmd()
        .arg("--file")
        .arg(&file)
        .args(["task", "add", "p1", "t1", "Read the book"])
        .assert()
        .success();

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["task", "done", "p1", "t1"])
        .assert()
        .success();

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "show", "p1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("[x]"));
}

#[test]
fn note_add_then_show_displays_the_note() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("workspace.json");
    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "add", "p1", "Learn Rust"])
        .assert()
        .success();

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["note", "add", "p1", "n1", "First win"])
        .assert()
        .success();

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "show", "p1"])
        .assert()
        .success()
        .stdout(predicate::str::contains("First win"));
}

#[test]
fn unknown_project_id_exits_nonzero_with_stderr_message() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("workspace.json");
    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "add", "p1", "Learn Rust"])
        .assert()
        .success();

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "show", "nope"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("unknown project id"));
}

#[test]
fn missing_file_read_command_errors_clearly() {
    let dir = tempfile::tempdir().unwrap();
    let file = dir.path().join("workspace.json");

    cmd()
        .arg("--file")
        .arg(&file)
        .args(["project", "list"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("workspace file not found"));
}
