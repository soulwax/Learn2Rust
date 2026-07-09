//! The status.json output shape. See the "Output Shape" section of
//! docs/superpowers/specs/2026-07-08-focus-forge-status-design.md.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct CommitInfo {
    pub hash: String,
    pub date: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CrateStatus {
    pub name: String,
    pub kind: String,
    pub tests_passed: u32,
    pub tests_failed: u32,
}

#[derive(Debug, Clone, Serialize)]
pub struct Status {
    pub generated_at: String,
    pub last_commit: CommitInfo,
    pub phase: u32,
    pub phase_name: String,
    pub chapter: String,
    pub chapter_name: String,
    pub crates: Vec<CrateStatus>,
}

#[cfg(test)]
mod tests {
    use super::{CommitInfo, CrateStatus, Status};

    fn sample_status() -> Status {
        Status {
            generated_at: "2026-07-08T00:00:00Z".to_string(),
            last_commit: CommitInfo {
                hash: "abc123".to_string(),
                date: "2026-07-08T14:17:57+02:00".to_string(),
            },
            phase: 2,
            phase_name: "First Product Slice".to_string(),
            chapter: "ch00".to_string(),
            chapter_name: "Setup And First Run".to_string(),
            crates: vec![CrateStatus {
                name: "focus_forge_core".to_string(),
                kind: "crate".to_string(),
                tests_passed: 23,
                tests_failed: 0,
            }],
        }
    }

    #[test]
    fn status_serializes_to_documented_shape() {
        let status = sample_status();
        let json = serde_json::to_value(&status).unwrap();

        assert_eq!(json["phase"], 2);
        assert_eq!(json["phase_name"], "First Product Slice");
        assert_eq!(json["chapter"], "ch00");
        assert_eq!(json["last_commit"]["hash"], "abc123");
        assert_eq!(json["crates"][0]["name"], "focus_forge_core");
        assert_eq!(json["crates"][0]["kind"], "crate");
        assert_eq!(json["crates"][0]["tests_passed"], 23);
        assert_eq!(json["crates"][0]["tests_failed"], 0);
    }
}
