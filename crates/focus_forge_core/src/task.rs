//! A `Task` is one actionable item inside a project. It carries a priority and
//! a done flag. `Task::complete` shows Rust's `&mut self` — an explicit,
//! compiler-checked mutable borrow, unlike freely mutating a field in C#/Java.

use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};

/// How urgent a task is. Serializes as `"low"`/`"medium"`/`"high"`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Priority {
    Low,
    #[default]
    Medium,
    High,
}

/// A single task. Prefer `Task::new`; fields are public for early chapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub priority: Priority,
    pub done: bool,
}

impl Task {
    /// Creates a task after validating `id` and `title`. Starts not done.
    pub fn new(id: &str, title: &str, priority: Priority) -> Result<Task> {
        if id.trim().is_empty() {
            return Err(CoreError::BlankId);
        }
        if title.trim().is_empty() {
            return Err(CoreError::EmptyTitle);
        }

        Ok(Task {
            id: id.trim().to_string(),
            title: title.trim().to_string(),
            priority,
            done: false,
        })
    }

    /// Marks the task done. `&mut self` = "I need a mutable borrow of this
    /// task"; the borrow checker guarantees no one else is reading it meanwhile.
    pub fn complete(&mut self) {
        self.done = true;
    }
}

#[cfg(test)]
mod tests {
    use super::{Priority, Task};
    use crate::error::CoreError;

    #[test]
    fn new_task_starts_not_done() {
        let t = Task::new("t1", "Write tests", Priority::High).unwrap();
        assert_eq!(t.priority, Priority::High);
        assert!(!t.done);
    }

    #[test]
    fn new_task_rejects_blank_id() {
        assert_eq!(
            Task::new(" ", "title", Priority::Low),
            Err(CoreError::BlankId)
        );
    }

    #[test]
    fn new_task_rejects_blank_title() {
        assert_eq!(
            Task::new("t1", "  ", Priority::Low),
            Err(CoreError::EmptyTitle)
        );
    }

    #[test]
    fn complete_sets_done() {
        let mut t = Task::new("t1", "x", Priority::Medium).unwrap();
        t.complete();
        assert!(t.done);
    }

    #[test]
    fn priority_serializes_snake_case() {
        assert_eq!(
            serde_json::to_string(&Priority::Medium).unwrap(),
            "\"medium\""
        );
    }
}
