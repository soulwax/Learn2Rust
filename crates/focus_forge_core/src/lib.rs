//! `focus_forge_core` owns the Focus Forge domain: the data types, their
//! validation rules, the crate error type, and JSON persistence. It depends
//! on no CLI or GUI code — those crates depend on this one, never the reverse.

mod error;
mod note;
mod project;
mod task;
mod workspace;

pub use error::{CoreError, Result};
pub use note::Note;
pub use project::{Project, ProjectStatus};
pub use task::{Priority, Task};
pub use workspace::{Workspace, CURRENT_VERSION};
