//! JSON persistence. `load_workspace` reads and re-validates; `save_workspace`
//! writes pretty JSON. Foreign errors (`std::io::Error`, `serde_json::Error`)
//! are converted into `CoreError` so callers only ever see our error type.

use std::path::Path;

use crate::error::{CoreError, Result};
use crate::workspace::Workspace;

/// Loads a workspace from `path`, then re-validates it. `map_err` converts the
/// foreign error into a `CoreError` — the `?` then propagates it. This is the
/// Rust equivalent of catching a low-level exception and rethrowing a domain one.
pub fn load_workspace(path: &Path) -> Result<Workspace> {
    let text = std::fs::read_to_string(path).map_err(|e| CoreError::Io(e.to_string()))?;
    let workspace: Workspace =
        serde_json::from_str(&text).map_err(|e| CoreError::Json(e.to_string()))?;
    workspace.validate()?;
    Ok(workspace)
}

/// Saves `workspace` to `path` as pretty-printed JSON.
pub fn save_workspace(workspace: &Workspace, path: &Path) -> Result<()> {
    let text =
        serde_json::to_string_pretty(workspace).map_err(|e| CoreError::Json(e.to_string()))?;
    std::fs::write(path, text).map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{load_workspace, save_workspace};
    use crate::error::CoreError;
    use crate::project::Project;
    use crate::workspace::Workspace;

    #[test]
    fn save_then_load_round_trips() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ws.json");

        let mut ws = Workspace::new("Round Trip");
        ws.add_project(Project::new("p1", "One").unwrap()).unwrap();

        save_workspace(&ws, &path).unwrap();
        let loaded = load_workspace(&path).unwrap();

        assert_eq!(ws, loaded);
    }

    #[test]
    fn load_missing_file_is_io_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("nope.json");
        match load_workspace(&path) {
            Err(CoreError::Io(_)) => {}
            other => panic!("expected Io error, got {other:?}"),
        }
    }

    #[test]
    fn load_malformed_json_is_json_error() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("bad.json");
        std::fs::write(&path, "{ not valid json").unwrap();
        match load_workspace(&path) {
            Err(CoreError::Json(_)) => {}
            other => panic!("expected Json error, got {other:?}"),
        }
    }
}
