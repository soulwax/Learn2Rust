//! Parses the small YAML-ish front-matter block at the top of STATUS.md
//! into phase/chapter fields. Not a general YAML parser — four scalar
//! `key: value` lines between two `---` delimiters is all this needs to
//! handle, so a full serde_yaml dependency would be more than this job
//! requires.

use crate::StatusError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrontMatter {
    pub phase: u32,
    pub phase_name: String,
    pub chapter: String,
    pub chapter_name: String,
}

pub fn parse_front_matter(text: &str) -> Result<FrontMatter, StatusError> {
    let mut lines = text.lines();

    let first = lines.next().unwrap_or("");
    if first.trim() != "---" {
        return Err(StatusError::StatusMd(
            "file does not start with a '---' front matter delimiter".to_string(),
        ));
    }

    let mut phase: Option<u32> = None;
    let mut phase_name: Option<String> = None;
    let mut chapter: Option<String> = None;
    let mut chapter_name: Option<String> = None;

    for line in lines.by_ref() {
        if line.trim() == "---" {
            break;
        }
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        let key = key.trim();
        let value = value.trim().to_string();
        match key {
            "phase" => {
                phase = Some(value.parse::<u32>().map_err(|_| {
                    StatusError::StatusMd(format!("phase is not a number: {value}"))
                })?)
            }
            "phase_name" => phase_name = Some(value),
            "chapter" => chapter = Some(value),
            "chapter_name" => chapter_name = Some(value),
            _ => {}
        }
    }

    Ok(FrontMatter {
        phase: phase.ok_or_else(|| StatusError::StatusMd("missing 'phase' field".to_string()))?,
        phase_name: phase_name
            .ok_or_else(|| StatusError::StatusMd("missing 'phase_name' field".to_string()))?,
        chapter: chapter
            .ok_or_else(|| StatusError::StatusMd("missing 'chapter' field".to_string()))?,
        chapter_name: chapter_name
            .ok_or_else(|| StatusError::StatusMd("missing 'chapter_name' field".to_string()))?,
    })
}

#[cfg(test)]
mod tests {
    use super::{parse_front_matter, FrontMatter};

    #[test]
    fn parses_well_formed_front_matter() {
        let text = std::fs::read_to_string("tests/fixtures/status_with_front_matter.md").unwrap();

        let front_matter = parse_front_matter(&text).unwrap();

        assert_eq!(
            front_matter,
            FrontMatter {
                phase: 2,
                phase_name: "First Product Slice".to_string(),
                chapter: "ch00".to_string(),
                chapter_name: "Setup And First Run".to_string(),
            }
        );
    }

    #[test]
    fn missing_front_matter_errors() {
        let text =
            std::fs::read_to_string("tests/fixtures/status_missing_front_matter.md").unwrap();

        let err = parse_front_matter(&text).unwrap_err();

        assert!(matches!(err, crate::StatusError::StatusMd(_)));
    }
}
