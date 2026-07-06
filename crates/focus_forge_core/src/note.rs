//! A `Note` is free text attached to a project. `created_at` is a
//! caller-supplied ISO-8601 string (the crate deliberately avoids a time
//! dependency at this stage).

use serde::{Deserialize, Serialize};

use crate::error::{CoreError, Result};

/// A note. Prefer `Note::new`; fields are public for early chapters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub text: String,
    pub created_at: String,
}

impl Note {
    /// Creates a note after validating `id` and `text`. `created_at` is stored
    /// as given (empty is allowed; only blank id/text are rejected).
    pub fn new(id: &str, text: &str, created_at: &str) -> Result<Note> {
        if id.trim().is_empty() {
            return Err(CoreError::BlankId);
        }
        if text.trim().is_empty() {
            return Err(CoreError::EmptyText);
        }

        Ok(Note {
            id: id.trim().to_string(),
            text: text.trim().to_string(),
            created_at: created_at.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Note;
    use crate::error::CoreError;

    #[test]
    fn new_note_keeps_text_and_timestamp() {
        let n = Note::new("n1", "first win", "2026-07-06T09:15:00Z").unwrap();
        assert_eq!(n.text, "first win");
        assert_eq!(n.created_at, "2026-07-06T09:15:00Z");
    }

    #[test]
    fn new_note_rejects_blank_id() {
        assert_eq!(Note::new(" ", "text", ""), Err(CoreError::BlankId));
    }

    #[test]
    fn new_note_rejects_blank_text() {
        assert_eq!(Note::new("n1", "   ", ""), Err(CoreError::EmptyText));
    }
}
