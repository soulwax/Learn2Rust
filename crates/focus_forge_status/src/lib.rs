//! Generates a static status.json snapshot of the workspace (crates/labs,
//! test counts, last commit, phase/chapter) by shelling out to git and
//! cargo. This is a repo-maintenance tool, not part of the Focus Forge
//! product surface — see docs/superpowers/specs/2026-07-08-focus-forge-status-design.md.

mod cargo_meta;
mod git;
mod model;
mod status_md;
mod test_counts;

use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use thiserror::Error;

use model::{CommitInfo, CrateStatus, Status};

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

fn run_git_log() -> Result<CommitInfo, StatusError> {
    let output = Command::new("git")
        .args(["log", "-1", "--format=%H%n%cI"])
        .output()
        .map_err(|e| StatusError::Git(e.to_string()))?;
    if !output.status.success() {
        return Err(StatusError::Git(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    git::parse_git_log_output(&stdout)
}

fn run_cargo_metadata() -> Result<Vec<cargo_meta::WorkspaceMember>, StatusError> {
    let output = Command::new("cargo")
        .args(["metadata", "--no-deps", "--format-version", "1"])
        .output()
        .map_err(|e| StatusError::CargoMetadata(e.to_string()))?;
    if !output.status.success() {
        return Err(StatusError::CargoMetadata(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    cargo_meta::parse_members(&stdout)
}

fn run_cargo_test(member: &str) -> Result<(u32, u32), StatusError> {
    let output = Command::new("cargo")
        .args(["test", "-p", member])
        .output()
        .map_err(|e| StatusError::CargoTest(member.to_string(), e.to_string()))?;
    // A crate with failing tests exits nonzero; we still want its counts,
    // so parse stdout regardless of exit status rather than erroring here.
    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(test_counts::parse_test_summary(&stdout))
}

fn read_status_md_front_matter() -> Result<status_md::FrontMatter, StatusError> {
    let text = std::fs::read_to_string("STATUS.md").map_err(|e| StatusError::Io(e.to_string()))?;
    status_md::parse_front_matter(&text)
}

fn rfc3339_now() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    // Hand-rolled UTC calendar conversion avoids adding a chrono/time
    // dependency for a single timestamp field (see the design spec's
    // "Output Shape" section).
    let days = secs / 86_400;
    let time_of_day = secs % 86_400;
    let (hours, minutes, seconds) = (
        time_of_day / 3600,
        (time_of_day % 3600) / 60,
        time_of_day % 60,
    );

    let mut year = 1970i64;
    let mut remaining_days = days as i64;
    loop {
        let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
        let days_in_year = if is_leap { 366 } else { 365 };
        if remaining_days < days_in_year {
            break;
        }
        remaining_days -= days_in_year;
        year += 1;
    }
    let is_leap = (year % 4 == 0 && year % 100 != 0) || year % 400 == 0;
    let month_lengths: [i64; 12] = [
        31,
        if is_leap { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 1;
    for len in month_lengths {
        if remaining_days < len {
            break;
        }
        remaining_days -= len;
        month += 1;
    }
    let day = remaining_days + 1;

    format!("{year:04}-{month:02}-{day:02}T{hours:02}:{minutes:02}:{seconds:02}Z")
}

pub fn run(out_path: &Path) -> Result<(), StatusError> {
    let last_commit = run_git_log()?;
    let front_matter = read_status_md_front_matter()?;
    let members = run_cargo_metadata()?;

    let mut crates = Vec::new();
    for member in &members {
        let (passed, failed) = run_cargo_test(&member.name)?;
        crates.push(CrateStatus {
            name: member.name.clone(),
            kind: member.kind.clone(),
            tests_passed: passed,
            tests_failed: failed,
        });
    }

    let status = Status {
        generated_at: rfc3339_now(),
        last_commit,
        phase: front_matter.phase,
        phase_name: front_matter.phase_name,
        chapter: front_matter.chapter,
        chapter_name: front_matter.chapter_name,
        crates,
    };

    let json =
        serde_json::to_string_pretty(&status).map_err(|e| StatusError::Json(e.to_string()))?;
    if let Some(parent) = out_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| StatusError::Io(e.to_string()))?;
    }
    std::fs::write(out_path, json).map_err(|e| StatusError::Io(e.to_string()))?;

    Ok(())
}

#[cfg(test)]
mod run_tests {
    use super::run;

    #[test]
    #[ignore = "shells out to real git/cargo against this repo; run manually with `cargo test -p focus_forge_status -- --ignored`"]
    fn run_writes_a_status_json_file() {
        let dir = tempfile::tempdir().unwrap();
        let out_path = dir.path().join("status.json");

        run(&out_path).unwrap();

        let contents = std::fs::read_to_string(&out_path).unwrap();
        let json: serde_json::Value = serde_json::from_str(&contents).unwrap();
        assert!(json["crates"].as_array().unwrap().len() >= 2);
        assert!(json["last_commit"]["hash"].is_string());
    }
}
