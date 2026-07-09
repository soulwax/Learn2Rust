// Chapter 2's binary shows three data-flow choices:
// borrow when reading, clone when an owned copy is truly needed, and move when a
// function intentionally takes over a value.

use ch02_ownership::{
    archive_project, borrowed_project_summary, cloned_name_cost_bytes,
    duplicate_project_name_for_template, has_tag, project_name_length,
};

fn main() {
    let project_name = String::from("Learn Rust Ownership");
    let description = String::from("Borrow first, move deliberately, clone consciously.");

    // Borrowing with `&project_name` lets the function read our string while this
    // `main` function keeps ownership and can use it again afterward.
    println!("{}", borrowed_project_summary(&project_name, &description));
    println!(
        "Project name has {} visible characters.",
        project_name_length(&project_name)
    );

    // A clone makes a real owned copy. We do it here because the template name
    // is meant to outlive this immediate formatting step.
    let template_name = duplicate_project_name_for_template(&project_name);
    println!(
        "Cloned template name '{template_name}' ({} bytes copied).",
        cloned_name_cost_bytes(&template_name)
    );

    let tags = ["rust", "ownership", "quick-win"];
    println!("Has ownership tag: {}", has_tag(&tags, "ownership"));

    // This move is intentional. After this call, `project_to_archive` is no
    // longer available in `main`; ownership moved into `archive_project`.
    let project_to_archive = String::from("Throwaway Prototype");
    println!("{}", archive_project(project_to_archive));
}
