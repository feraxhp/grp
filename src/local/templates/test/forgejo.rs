use grp_core::Platform;
use mynox::Report;

use crate::local::structs::Local;
use super::TempRepo;

#[test]
fn stops_at_first_candidate_in_precedence_order_and_strips_frontmatter() -> Result<(), Report> {
    let temp = TempRepo::new();
    temp.write_file(
        ".forgejo/PULL_REQUEST_TEMPLATE.md",
        "---\nname: 'Forgejo PR'\nabout: 'Testing'\nlabels: bug, help needed\n---\nClean body\n",
    );
    temp.write_file(".github/pull_request_template.md", "Should not be loaded");

    let local = Local(Platform::Forgejo);
    let templates = local.load_pr_templates(&temp.repo)?;

    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].name, "Forgejo PR");
    assert!(templates[0].is_default);
    assert_eq!(templates[0].content, "Clean body\n");
    Ok(())
}

#[test]
fn supports_docs_directory_and_preserves_markdown_horizontal_rules() -> Result<(), Report> {
    let temp = TempRepo::new();
    let content = "---\n# Heading\n---\nBody below rules";
    temp.write_file("docs/pull_request_template.md", content);

    let local = Local(Platform::Forgejo);
    let templates = local.load_pr_templates(&temp.repo)?;

    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].name, "Default");
    assert!(templates[0].is_default);
    assert_eq!(templates[0].content, content);
    Ok(())
}

#[test]
fn errors_on_bare_repository() {
    let bare = TempRepo::new_bare();
    let local = Local(Platform::Forgejo);
    assert!(local.load_pr_templates(&bare.repo).is_err());
}
