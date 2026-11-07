use grp_core::Platform;
use mynox::Report;

use crate::local::structs::Local;
use super::TempRepo;

#[test]
fn loads_single_default_and_multiple_templates() -> Result<(), Report> {
    let temp = TempRepo::new();
    temp.write_file(".github/pull_request_template.md", "# Default PR\nBody");
    temp.write_file(".github/PULL_REQUEST_TEMPLATE/bug_fix.md", "## Bug Fix");
    temp.write_file(".github/PULL_REQUEST_TEMPLATE/feature_request.txt", "Feature text");

    let local = Local(Platform::Github);
    let templates = local.load_pr_templates(&temp.repo)?;

    assert_eq!(templates.len(), 3);
    assert_eq!(templates[0].name, "Default");
    assert!(templates[0].is_default);
    assert_eq!(templates[0].content, "# Default PR\nBody");

    assert_eq!(templates[1].name, "bug fix");
    assert!(!templates[1].is_default);
    assert_eq!(templates[1].content, "## Bug Fix");

    assert_eq!(templates[2].name, "feature request");
    assert!(!templates[2].is_default);
    assert_eq!(templates[2].content, "Feature text");
    Ok(())
}

#[test]
fn preserves_frontmatter_in_github() -> Result<(), Report> {
    let temp = TempRepo::new();
    let md_with_frontmatter = "---\nname: Custom\n---\n# Body";
    temp.write_file("docs/Pull_Request_Template.md", md_with_frontmatter);

    let local = Local(Platform::Github);
    let templates = local.load_pr_templates(&temp.repo)?;

    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].name, "Default");
    assert!(templates[0].is_default);
    assert_eq!(templates[0].content, md_with_frontmatter);
    Ok(())
}
