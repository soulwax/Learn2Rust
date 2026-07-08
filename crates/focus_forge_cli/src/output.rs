//! Pure formatting: domain references in, `String` out. No I/O, no domain
//! mutation — that split is what makes these functions unit-testable without
//! a filesystem, the same way you'd keep a C#/Java `ToString`-style formatter
//! free of side effects so it can be tested in isolation.

use focus_forge_core::{Priority, Project, ProjectStatus, Workspace};

/// Lowercases a `Priority` for display, matching its `snake_case` JSON
/// spelling. Same rationale as `status_label` below.
fn priority_label(priority: &Priority) -> &'static str {
    match priority {
        Priority::Low => "low",
        Priority::Medium => "medium",
        Priority::High => "high",
    }
}

/// Lowercases a `ProjectStatus` for display, matching its `snake_case` JSON
/// spelling. `focus_forge_core` derives `Serialize` but not `Display` for
/// this enum, so the CLI (the only place that prints for humans) owns this
/// mapping — the same split you'd get from a separate presentation-layer
/// formatter in C#/Java rather than overriding `toString()` on the entity.
fn status_label(status: &ProjectStatus) -> &'static str {
    match status {
        ProjectStatus::Planned => "planned",
        ProjectStatus::Active => "active",
        ProjectStatus::Paused => "paused",
        ProjectStatus::Done => "done",
    }
}

/// Formats `project list` output: one line per project, or a friendly hint
/// when the workspace has none yet.
pub fn format_project_list(workspace: &Workspace) -> String {
    if workspace.projects.is_empty() {
        return "No projects yet. Add one with: project add <id> <name>".to_string();
    }

    // `map` + `collect` + `join` is Rust's LINQ `Select(...).ToList()` then
    // `String.Join("\n", ...)` — build one formatted line per project, then
    // stitch them together with newlines.
    workspace
        .projects
        .iter()
        .map(|p| {
            format!(
                "{}  {}  [{}]  ({} tasks, {} notes)",
                p.id,
                p.name,
                status_label(&p.status),
                p.tasks.len(),
                p.notes.len()
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Formats `project show <id>` output: a header line, then tasks and notes.
/// Each section prints `(none)` when empty rather than an empty list, so the
/// output is never ambiguous between "no section" and "nothing to show".
pub fn format_project_show(project: &Project) -> String {
    let header = format!(
        "{}  {}  [{}]",
        project.id,
        project.name,
        status_label(&project.status)
    );

    let tasks = if project.tasks.is_empty() {
        "(none)".to_string()
    } else {
        project
            .tasks
            .iter()
            .map(|t| {
                let mark = if t.done { "x" } else { " " };
                format!("[{mark}]  {}  {}", priority_label(&t.priority), t.title)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    let notes = if project.notes.is_empty() {
        "(none)".to_string()
    } else {
        project
            .notes
            .iter()
            .map(|n| format!("- {}", n.text))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!("{header}\nTasks:\n{tasks}\nNotes:\n{notes}")
}

#[cfg(test)]
mod tests {
    use super::{format_project_list, format_project_show};
    use focus_forge_core::{Note, Priority, Project, Task, Workspace};

    #[test]
    fn empty_workspace_prints_friendly_hint() {
        let ws = Workspace::new("Demo");
        assert_eq!(
            format_project_list(&ws),
            "No projects yet. Add one with: project add <id> <name>"
        );
    }

    #[test]
    fn lists_one_project_per_line_with_status_and_counts() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(Project::new("p1", "Learn Rust").unwrap())
            .unwrap();
        assert_eq!(
            format_project_list(&ws),
            "p1  Learn Rust  [planned]  (0 tasks, 0 notes)"
        );
    }

    #[test]
    fn shows_project_header_tasks_and_notes() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(Project::new("p1", "Learn Rust").unwrap())
            .unwrap();
        let mut task = Task::new("t1", "Read the book", Priority::High).unwrap();
        task.complete();
        ws.add_task("p1", task).unwrap();
        ws.add_task("p1", Task::new("t2", "Write code", Priority::Low).unwrap())
            .unwrap();
        ws.add_note("p1", Note::new("n1", "First win", "").unwrap())
            .unwrap();

        let shown = format_project_show(ws.project("p1").unwrap());

        assert_eq!(
            shown,
            "p1  Learn Rust  [planned]\n\
             Tasks:\n\
             [x]  high  Read the book\n\
             [ ]  low  Write code\n\
             Notes:\n\
             - First win"
        );
    }

    #[test]
    fn shows_project_with_no_tasks_or_notes() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(Project::new("p1", "Learn Rust").unwrap())
            .unwrap();

        let shown = format_project_show(ws.project("p1").unwrap());

        assert_eq!(
            shown,
            "p1  Learn Rust  [planned]\n\
             Tasks:\n\
             (none)\n\
             Notes:\n\
             (none)"
        );
    }
}
