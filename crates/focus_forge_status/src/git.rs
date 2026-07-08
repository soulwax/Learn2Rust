//! Parses `git log -1 --format=%H%n%cI` output (commit hash on line 1,
//! ISO 8601 commit date on line 2) into a CommitInfo.

use crate::model::CommitInfo;
use crate::StatusError;

pub fn parse_git_log_output(stdout: &str) -> Result<CommitInfo, StatusError> {
    let mut lines = stdout.lines();
    let hash = lines
        .next()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| StatusError::Git("missing commit hash line".to_string()))?
        .trim()
        .to_string();
    let date = lines
        .next()
        .filter(|s| !s.trim().is_empty())
        .ok_or_else(|| StatusError::Git("missing commit date line".to_string()))?
        .trim()
        .to_string();

    Ok(CommitInfo { hash, date })
}

#[cfg(test)]
mod tests {
    use super::parse_git_log_output;

    #[test]
    fn parses_hash_and_date_lines() {
        let stdout = "3448566d1ec15c7f6eeed33914f1f1eb4b76a7a6\n2026-07-08T14:17:57+02:00\n";

        let commit = parse_git_log_output(stdout).unwrap();

        assert_eq!(commit.hash, "3448566d1ec15c7f6eeed33914f1f1eb4b76a7a6");
        assert_eq!(commit.date, "2026-07-08T14:17:57+02:00");
    }

    #[test]
    fn empty_output_errors() {
        let err = parse_git_log_output("").unwrap_err();

        assert!(matches!(err, crate::StatusError::Git(_)));
    }
}
