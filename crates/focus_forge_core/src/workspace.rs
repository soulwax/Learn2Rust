//! The `Workspace` is the aggregate root: the single top-level object that
//! owns every project. Operations that add children enforce invariants
//! (unique project ids, references resolve) so the whole graph stays valid.

use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};
use crate::note::Note;
use crate::project::Project;
use crate::task::Task;

/// On-disk format version. Bump when the JSON shape changes incompatibly.
pub const CURRENT_VERSION: u32 = 1;

/// Top-level container for all Focus Forge data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workspace {
    pub version: u32,
    pub name: String,
    pub projects: Vec<Project>,
}

impl Workspace {
    /// Creates an empty workspace at the current format version.
    pub fn new(name: &str) -> Workspace {
        Workspace {
            version: CURRENT_VERSION,
            name: name.to_string(),
            projects: Vec::new(),
        }
    }

    /// Adds a project, rejecting a duplicate id. `iter().any(...)` is Rust's
    /// LINQ/stream `Any` — it short-circuits on the first match.
    pub fn add_project(&mut self, project: Project) -> Result<()> {
        if self.projects.iter().any(|p| p.id == project.id) {
            return Err(CoreError::DuplicateId(project.id));
        }
        self.projects.push(project);
        Ok(())
    }

    /// Borrows the project with `id`, or `None`. Returning `Option<&Project>`
    /// (a borrow, not a clone) is the idiomatic, allocation-free lookup.
    pub fn project(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }

    /// Adds a task to an existing project, or errors if the project is unknown.
    pub fn add_task(&mut self, project_id: &str, task: Task) -> Result<()> {
        let project = self
            .projects
            .iter_mut()
            .find(|p| p.id == project_id)
            .ok_or_else(|| CoreError::UnknownProject(project_id.to_string()))?;
        project.tasks.push(task);
        Ok(())
    }

    /// Adds a note to an existing project, or errors if the project is unknown.
    pub fn add_note(&mut self, project_id: &str, note: Note) -> Result<()> {
        let project = self
            .projects
            .iter_mut()
            .find(|p| p.id == project_id)
            .ok_or_else(|| CoreError::UnknownProject(project_id.to_string()))?;
        project.notes.push(note);
        Ok(())
    }

    /// Re-checks invariants: non-blank project ids and no duplicates. Called
    /// after loading untrusted JSON from disk.
    pub fn validate(&self) -> Result<()> {
        let mut seen: Vec<&str> = Vec::new();
        for project in &self.projects {
            if project.id.trim().is_empty() {
                return Err(CoreError::BlankId);
            }
            if seen.contains(&project.id.as_str()) {
                return Err(CoreError::DuplicateId(project.id.clone()));
            }
            seen.push(&project.id);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::{Workspace, CURRENT_VERSION};
    use crate::error::CoreError;
    use crate::note::Note;
    use crate::project::Project;
    use crate::task::{Priority, Task};

    fn project(id: &str) -> Project {
        Project::new(id, "A project").unwrap()
    }

    #[test]
    fn new_workspace_has_current_version_and_no_projects() {
        let ws = Workspace::new("Demo");
        assert_eq!(ws.version, CURRENT_VERSION);
        assert!(ws.projects.is_empty());
    }

    #[test]
    fn add_project_then_look_it_up() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(project("p1")).unwrap();
        assert!(ws.project("p1").is_some());
        assert!(ws.project("missing").is_none());
    }

    #[test]
    fn add_duplicate_project_is_rejected() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(project("p1")).unwrap();
        assert_eq!(
            ws.add_project(project("p1")),
            Err(CoreError::DuplicateId("p1".to_string()))
        );
    }

    #[test]
    fn add_task_to_known_project() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(project("p1")).unwrap();
        let t = Task::new("t1", "do it", Priority::Low).unwrap();
        ws.add_task("p1", t).unwrap();
        assert_eq!(ws.project("p1").unwrap().tasks.len(), 1);
    }

    #[test]
    fn add_task_to_unknown_project_errors() {
        let mut ws = Workspace::new("Demo");
        let t = Task::new("t1", "do it", Priority::Low).unwrap();
        assert_eq!(
            ws.add_task("nope", t),
            Err(CoreError::UnknownProject("nope".to_string()))
        );
    }

    #[test]
    fn add_note_to_unknown_project_errors() {
        let mut ws = Workspace::new("Demo");
        let n = Note::new("n1", "hi", "").unwrap();
        assert_eq!(
            ws.add_note("nope", n),
            Err(CoreError::UnknownProject("nope".to_string()))
        );
    }

    #[test]
    fn validate_passes_for_unique_ids() {
        let mut ws = Workspace::new("Demo");
        ws.add_project(project("p1")).unwrap();
        ws.add_project(project("p2")).unwrap();
        assert!(ws.validate().is_ok());
    }
}
