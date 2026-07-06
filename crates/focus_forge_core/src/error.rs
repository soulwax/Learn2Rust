//! The one error type the crate exposes. Callers match on `CoreError`
//! instead of juggling `std::io::Error`, `serde_json::Error`, etc.
//! (Compare a single checked-exception type in Java, or a discriminated
//! union of error shapes in TypeScript.)

use thiserror::Error;

/// Every fallible operation in this crate returns this error on failure.
///
/// `#[derive(Error)]` + `#[error("...")]` come from the `thiserror` crate;
/// they generate the `Display` and `std::error::Error` impls for us, so a
/// `CoreError` prints a readable message automatically.
#[derive(Debug, Error, PartialEq)]
pub enum CoreError {
    #[error("name must not be blank")]
    EmptyName,
    #[error("title must not be blank")]
    EmptyTitle,
    #[error("text must not be blank")]
    EmptyText,
    #[error("id must not be blank")]
    BlankId,
    #[error("duplicate project id: {0}")]
    DuplicateId(String),
    #[error("unknown project id: {0}")]
    UnknownProject(String),
    #[error("io error: {0}")]
    Io(String),
    #[error("json error: {0}")]
    Json(String),
}

/// Crate-wide result alias so signatures read `Result<Project>` rather than
/// `std::result::Result<Project, CoreError>`. (Like a project-wide type alias.)
pub type Result<T> = std::result::Result<T, CoreError>;

#[cfg(test)]
mod tests {
    use super::CoreError;

    #[test]
    fn error_messages_are_human_readable() {
        assert_eq!(CoreError::EmptyName.to_string(), "name must not be blank");
        assert_eq!(
            CoreError::DuplicateId("proj-x".to_string()).to_string(),
            "duplicate project id: proj-x"
        );
    }
}
