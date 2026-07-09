// Chapter 2 gives names to the data-flow rules Chapter 1 only hinted at:
// ownership, borrowing, moving, and cloning.
//
// The examples stay intentionally small. Focus Forge will get proper structs in
// Chapter 3; here, plain `String`, `&str`, and slices make the ownership flow
// impossible to hide.

/// Borrows a project name and returns the number of visible characters.
///
/// The caller keeps ownership of the original text. This is the Rust habit to
/// build early: if a function only needs to inspect text, borrow `&str` instead
/// of taking a `String`.
pub fn project_name_length(name: &str) -> usize {
    name.trim().chars().count()
}

/// Borrows project text and returns a new owned summary.
///
/// Both inputs are borrowed, so the caller can keep using its original strings
/// after this function returns. The returned `String` is new text made by
/// `format!`, so returning ownership is exactly right.
pub fn borrowed_project_summary(name: &str, description: &str) -> String {
    let clean_name = name.trim();
    let clean_description = description.trim();

    format!("{clean_name}: {clean_description}")
}

/// Consumes an owned project name and turns it into archived text.
///
/// Taking `String` by value means ownership moves into this function. The caller
/// cannot use that original `String` afterward unless it cloned before calling.
/// That is not scary; it is a clear API signal that this function takes over.
pub fn archive_project(project_name: String) -> String {
    format!("Archived project: {project_name}")
}

/// Clones a borrowed name only when a genuinely owned copy is needed.
///
/// `to_owned()` allocates a new `String`. This is useful at boundaries where data
/// must outlive the borrowed input, but it should be a conscious choice rather
/// than the default way out of every ownership question.
pub fn duplicate_project_name_for_template(project_name: &str) -> String {
    project_name.trim().to_owned()
}

/// Checks whether a borrowed list of tags contains a target tag.
///
/// `&[&str]` is a borrowed slice: a temporary view into a sequence owned
/// somewhere else. We can scan it without copying the whole list.
pub fn has_tag(tags: &[&str], target: &str) -> bool {
    tags.iter().any(|tag| tag.trim() == target)
}

/// Estimates the number of bytes copied when cloning a project name.
///
/// This tiny calculation connects ownership to cost: cloning ten bytes is cheap,
/// cloning a large workspace repeatedly is not.
pub fn cloned_name_cost_bytes(project_name: &str) -> usize {
    project_name.len()
}

#[cfg(test)]
mod tests {
    use super::{
        archive_project, borrowed_project_summary, cloned_name_cost_bytes,
        duplicate_project_name_for_template, has_tag, project_name_length,
    };

    #[test]
    fn borrowed_functions_leave_original_string_usable() {
        let name = String::from(" Learn Ownership ");
        let description = String::from("Practice moving and borrowing.");

        let summary = borrowed_project_summary(&name, &description);

        assert_eq!(summary, "Learn Ownership: Practice moving and borrowing.");
        assert_eq!(project_name_length(&name), 15);
        assert!(name.contains("Ownership"));
    }

    #[test]
    fn consuming_function_returns_archived_text() {
        let archived = archive_project(String::from("Old Experiment"));

        assert_eq!(archived, "Archived project: Old Experiment");
    }

    #[test]
    fn clone_is_explicit_when_owned_copy_is_needed() {
        let original = String::from("Template Project");
        let copied = duplicate_project_name_for_template(&original);

        assert_eq!(copied, original);
        assert_eq!(cloned_name_cost_bytes(&original), 16);
    }

    #[test]
    fn borrowed_tag_slice_can_be_scanned_without_copying() {
        let tags = ["rust", " ownership ", "quick-win"];

        assert!(has_tag(&tags, "ownership"));
        assert!(!has_tag(&tags, "blocked"));
    }
}
