//! One handler per command. This is the only module that touches the
//! filesystem or mutates a `Workspace` — `output.rs` stays pure, `lib.rs`
//! only parses arguments and dispatches here.

use std::path::Path;

use focus_forge_core::{load_workspace, save_workspace, Note, Priority, Project, Task, Workspace};

use crate::CliError;

/// Parses the CLI's plain-string `--priority` value into the domain enum.
/// Lives here (not in `focus_forge_core`) because parsing user-facing text
/// into an error message is CLI presentation concern, not a domain rule.
fn parse_priority(raw: &str) -> Result<Priority, CliError> {
    match raw {
        "low" => Ok(Priority::Low),
        "medium" => Ok(Priority::Medium),
        "high" => Ok(Priority::High),
        other => Err(CliError::InvalidPriority(other.to_string())),
    }
}

/// Adds a project, creating a fresh workspace file first if `path` doesn't
/// exist yet. This is the ONLY command allowed to bootstrap — every other
/// command requires an existing file (see the module-level design note in
/// the CLI design spec: without this, the very first project could never be
/// created).
pub fn project_add(path: &Path, id: &str, name: &str) -> Result<(), CliError> {
    let mut workspace = if path.exists() {
        load_workspace(path)?
    } else {
        Workspace::new("Focus Forge")
    };

    workspace.add_project(Project::new(id, name)?)?;
    save_workspace(&workspace, path)?;
    Ok(())
}

/// Loads an existing workspace, turning a missing file into
/// `CliError::WorkspaceNotFound` instead of the generic `CoreError::Io` that
/// `load_workspace` would raise. Every command except `project add` calls
/// this instead of `load_workspace` directly, so a missing file always gets
/// this friendlier message pointing at `project add`.
fn require_workspace(path: &Path) -> Result<Workspace, CliError> {
    if !path.exists() {
        return Err(CliError::WorkspaceNotFound(path.display().to_string()));
    }
    Ok(load_workspace(path)?)
}

/// Loads the workspace for `project list`.
pub fn project_list(path: &Path) -> Result<Workspace, CliError> {
    require_workspace(path)
}

/// Loads the workspace and returns a clone of the named project for
/// `project show`. Cloning avoids returning a reference into a `Workspace`
/// that's about to be dropped at the end of this function — the same
/// "return a copy, not a dangling reference" tradeoff you'd make returning a
/// detached DTO from a short-lived C#/Java repository call.
pub fn project_show(path: &Path, id: &str) -> Result<Project, CliError> {
    let workspace = require_workspace(path)?;
    workspace
        .project(id)
        .cloned()
        .ok_or_else(|| CliError::Core(focus_forge_core::CoreError::UnknownProject(id.to_string())))
}

/// Adds a task to an existing project: load, validate priority, mutate,
/// save.
pub fn task_add(
    path: &Path,
    project_id: &str,
    task_id: &str,
    title: &str,
    priority: &str,
) -> Result<(), CliError> {
    let priority = parse_priority(priority)?;
    let mut workspace = require_workspace(path)?;
    let task = Task::new(task_id, title, priority)?;
    workspace.add_task(project_id, task)?;
    save_workspace(&workspace, path)?;
    Ok(())
}

/// Marks a task done: load, find project and task by id, mutate, save.
/// `focus_forge_core` doesn't expose a task-lookup helper (out of scope for
/// this slice), so this reaches into `Workspace::projects`/`Project::tasks`
/// directly — both are public fields, so no core changes are needed.
pub fn task_done(path: &Path, project_id: &str, task_id: &str) -> Result<(), CliError> {
    let mut workspace = require_workspace(path)?;
    let project = workspace
        .projects
        .iter_mut()
        .find(|p| p.id == project_id)
        .ok_or_else(|| {
            CliError::Core(focus_forge_core::CoreError::UnknownProject(
                project_id.to_string(),
            ))
        })?;
    let task = project
        .tasks
        .iter_mut()
        .find(|t| t.id == task_id)
        .ok_or_else(|| CliError::UnknownTask(task_id.to_string()))?;
    task.complete();
    save_workspace(&workspace, path)?;
    Ok(())
}

/// Adds a note to an existing project: load, mutate, save. `created_at` is
/// stored as an empty string — `focus_forge_core` deliberately has no time
/// dependency yet (see the core crate's module docs); a later chapter that
/// introduces `chrono`/`time` will fill this in for real.
pub fn note_add(path: &Path, project_id: &str, note_id: &str, text: &str) -> Result<(), CliError> {
    let mut workspace = require_workspace(path)?;
    let note = Note::new(note_id, text, "")?;
    workspace.add_note(project_id, note)?;
    save_workspace(&workspace, path)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::project_add;

    #[test]
    fn project_add_bootstraps_missing_workspace_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");

        project_add(&path, "p1", "Learn Rust").unwrap();

        let loaded = focus_forge_core::load_workspace(&path).unwrap();
        assert_eq!(loaded.name, "Focus Forge");
        assert!(loaded.project("p1").is_some());
    }

    #[test]
    fn project_add_appends_to_existing_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "First").unwrap();

        project_add(&path, "p2", "Second").unwrap();

        let loaded = focus_forge_core::load_workspace(&path).unwrap();
        assert!(loaded.project("p1").is_some());
        assert!(loaded.project("p2").is_some());
    }

    #[test]
    fn project_add_rejects_duplicate_id() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "First").unwrap();

        let err = project_add(&path, "p1", "Second").unwrap_err();

        assert!(matches!(
            err,
            crate::CliError::Core(focus_forge_core::CoreError::DuplicateId(_))
        ));
    }

    #[test]
    fn project_list_on_missing_file_errors_workspace_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");

        let err = super::project_list(&path).unwrap_err();

        assert!(matches!(err, crate::CliError::WorkspaceNotFound(_)));
    }

    #[test]
    fn project_list_returns_loaded_workspace() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        let ws = super::project_list(&path).unwrap();

        assert!(ws.project("p1").is_some());
    }

    #[test]
    fn project_show_returns_the_named_project() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        let project = super::project_show(&path, "p1").unwrap();

        assert_eq!(project.id, "p1");
    }

    #[test]
    fn project_show_unknown_id_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        let err = super::project_show(&path, "nope").unwrap_err();

        assert!(matches!(
            err,
            crate::CliError::Core(focus_forge_core::CoreError::UnknownProject(_))
        ));
    }

    #[test]
    fn project_show_on_missing_file_errors_workspace_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");

        let err = super::project_show(&path, "p1").unwrap_err();

        assert!(matches!(err, crate::CliError::WorkspaceNotFound(_)));
    }

    #[test]
    fn task_add_persists_task_on_project() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        super::task_add(&path, "p1", "t1", "Read the book", "high").unwrap();

        let project = super::project_show(&path, "p1").unwrap();
        assert_eq!(project.tasks.len(), 1);
        assert_eq!(project.tasks[0].priority, focus_forge_core::Priority::High);
    }

    #[test]
    fn task_add_unknown_project_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        let err = super::task_add(&path, "nope", "t1", "title", "medium").unwrap_err();

        assert!(matches!(
            err,
            crate::CliError::Core(focus_forge_core::CoreError::UnknownProject(_))
        ));
    }

    #[test]
    fn task_add_invalid_priority_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        let err = super::task_add(&path, "p1", "t1", "title", "urgent").unwrap_err();

        assert!(matches!(err, crate::CliError::InvalidPriority(_)));
    }

    #[test]
    fn task_done_marks_task_complete() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();
        super::task_add(&path, "p1", "t1", "Read the book", "medium").unwrap();

        super::task_done(&path, "p1", "t1").unwrap();

        let project = super::project_show(&path, "p1").unwrap();
        assert!(project.tasks[0].done);
    }

    #[test]
    fn task_done_unknown_project_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        let err = super::task_done(&path, "nope", "t1").unwrap_err();

        assert!(matches!(
            err,
            crate::CliError::Core(focus_forge_core::CoreError::UnknownProject(_))
        ));
    }

    #[test]
    fn note_add_persists_note_on_project() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        super::note_add(&path, "p1", "n1", "First win").unwrap();

        let project = super::project_show(&path, "p1").unwrap();
        assert_eq!(project.notes.len(), 1);
        assert_eq!(project.notes[0].text, "First win");
    }

    #[test]
    fn note_add_unknown_project_errors() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("workspace.json");
        project_add(&path, "p1", "Learn Rust").unwrap();

        let err = super::note_add(&path, "nope", "n1", "text").unwrap_err();

        assert!(matches!(
            err,
            crate::CliError::Core(focus_forge_core::CoreError::UnknownProject(_))
        ));
    }
}
