//! Parses `cargo metadata --no-deps --format-version 1` JSON output into
//! workspace member names, classifying each as a "crate" (path contains
//! /crates/) or a "lab" (path contains /labs/) per the repo's documented
//! directory layout in IMPLEMENTATION.md.

use serde::Deserialize;

use crate::StatusError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorkspaceMember {
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    packages: Vec<Package>,
}

#[derive(Debug, Deserialize)]
struct Package {
    name: String,
    manifest_path: String,
}

pub fn parse_members(metadata_json: &str) -> Result<Vec<WorkspaceMember>, StatusError> {
    let metadata: Metadata = serde_json::from_str(metadata_json)
        .map_err(|e| StatusError::CargoMetadata(e.to_string()))?;

    Ok(metadata
        .packages
        .into_iter()
        .map(|p| {
            let normalized = p.manifest_path.replace('\\', "/");
            let kind = if normalized.contains("/crates/") {
                "crate"
            } else if normalized.contains("/labs/") {
                "lab"
            } else {
                "other"
            };
            WorkspaceMember {
                name: p.name,
                kind: kind.to_string(),
            }
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::{parse_members, WorkspaceMember};

    #[test]
    fn classifies_crates_and_labs_by_manifest_path() {
        let json = r#"{
            "packages": [
                { "name": "focus_forge_core", "manifest_path": "D:\\repo\\crates\\focus_forge_core\\Cargo.toml" },
                { "name": "ch00_setup", "manifest_path": "D:\\repo\\labs\\ch00_setup\\Cargo.toml" }
            ]
        }"#;

        let members = parse_members(json).unwrap();

        assert_eq!(
            members,
            vec![
                WorkspaceMember {
                    name: "focus_forge_core".to_string(),
                    kind: "crate".to_string(),
                },
                WorkspaceMember {
                    name: "ch00_setup".to_string(),
                    kind: "lab".to_string(),
                },
            ]
        );
    }

    #[test]
    fn malformed_json_errors() {
        let err = parse_members("not json").unwrap_err();

        assert!(matches!(err, crate::StatusError::CargoMetadata(_)));
    }
}
