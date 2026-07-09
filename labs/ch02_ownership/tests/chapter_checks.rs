use ch02_ownership::{
    archive_project, borrowed_project_summary, duplicate_project_name_for_template, has_tag,
    project_name_length,
};

#[test]
fn chapter_2_public_api_demonstrates_borrow_clone_and_move() {
    let name = String::from(" Ownership Lab ");
    let summary = borrowed_project_summary(&name, "Watch data flow.");
    let copied = duplicate_project_name_for_template(&name);
    let archived = archive_project(String::from("Finished Spike"));
    let tags = ["rust", "borrow"];

    assert_eq!(summary, "Ownership Lab: Watch data flow.");
    assert_eq!(project_name_length(&name), 13);
    assert_eq!(copied, "Ownership Lab");
    assert_eq!(archived, "Archived project: Finished Spike");
    assert!(has_tag(&tags, "borrow"));
}
