//! `focus_forge_core` owns the Focus Forge domain: the data types, their
//! validation rules, the crate error type, and JSON persistence. It depends
//! on no CLI or GUI code — those crates depend on this one, never the reverse.

mod error;
mod note;
mod task;

pub use error::{CoreError, Result};
pub use note::Note;
pub use task::{Priority, Task};
