//! Generates a static status.json snapshot of the workspace (crates/labs,
//! test counts, last commit, phase/chapter) by shelling out to git and
//! cargo. This is a repo-maintenance tool, not part of the Focus Forge
//! product surface — see docs/superpowers/specs/2026-07-08-focus-forge-status-design.md.

use std::path::Path;

use thiserror::Error;

mod model;
mod status_md;
mod test_counts;

#[derive(Debug, Error)]
pub enum StatusError {
    #[error("git command failed: {0}")]
    Git(String),
    #[error("cargo metadata failed: {0}")]
    CargoMetadata(String),
    #[error("cargo test failed for {0}: {1}")]
    CargoTest(String, String),
    #[error("could not parse STATUS.md front matter: {0}")]
    StatusMd(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("json error: {0}")]
    Json(String),
}

pub fn run(_out_path: &Path) -> Result<(), StatusError> {
    Ok(())
}
