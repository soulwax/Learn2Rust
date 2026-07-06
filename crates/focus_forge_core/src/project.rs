//! A `Project` groups tasks and notes under a name and status. Compare a
//! small entity/aggregate class in an OOP app — but here validation lives in
//! the constructor and illegal states are rejected up front.

use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};
use crate::note::Note;
use crate::task::Task;

/// Lifecycle status of a project.
///
/// `rename_all = "snake_case"` makes the JSON read `"planned"`/`"active"`,
/// matching `sample_data/demo_workspace.json`. A Rust enum is like a C#/Java
/// enum, except variants can carry data (not needed here).
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectStatus {
    /// Not started yet. `#[default]` marks the value `Default` returns.
    #[default]
    Planned,
    Active,
    Paused,
    Done,
}

/// A project and everything attached to it.
///
/// Fields are public for now (early-chapter simplicity). Prefer `Project::new`
/// to build a valid one; direct struct construction bypasses validation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub created_at: String,
    pub updated_at: String,
    pub tags: Vec<String>,
    pub tasks: Vec<Task>,
    pub notes: Vec<Note>,
}

impl Project {
    /// Creates a new, empty project after validating `id` and `name`.
    ///
    /// Returns `Err(CoreError::BlankId)` or `Err(CoreError::EmptyName)` for
    /// blank input. Timestamps start empty: a freshly created in-memory
    /// project has no persisted time yet. The `?`-friendly `Result` return is
    /// Rust's alternative to throwing an exception.
    pub fn new(id: &str, name: &str) -> Result<Project> {
        if id.trim().is_empty() {
            return Err(CoreError::BlankId);
        }
        if name.trim().is_empty() {
            return Err(CoreError::EmptyName);
        }

        Ok(Project {
            id: id.trim().to_string(),
            name: name.trim().to_string(),
            description: String::new(),
            status: ProjectStatus::Planned,
            created_at: String::new(),
            updated_at: String::new(),
            tags: Vec::new(),
            tasks: Vec::new(),
            notes: Vec::new(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Project, ProjectStatus};
    use crate::error::CoreError;

    #[test]
    fn new_project_starts_planned_and_empty() {
        let p = Project::new("proj-x", "Learn Rust").unwrap();
        assert_eq!(p.id, "proj-x");
        assert_eq!(p.name, "Learn Rust");
        assert_eq!(p.status, ProjectStatus::Planned);
        assert!(p.tasks.is_empty());
        assert!(p.notes.is_empty());
    }

    #[test]
    fn new_project_rejects_blank_id() {
        assert_eq!(Project::new("   ", "Name"), Err(CoreError::BlankId));
    }

    #[test]
    fn new_project_rejects_blank_name() {
        assert_eq!(Project::new("id", "  "), Err(CoreError::EmptyName));
    }

    #[test]
    fn status_serializes_snake_case() {
        let json = serde_json::to_string(&ProjectStatus::Active).unwrap();
        assert_eq!(json, "\"active\"");
    }
}
