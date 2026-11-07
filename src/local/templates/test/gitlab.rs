use grp_core::Platform;
use mynox::Report;

use crate::local::structs::Local;
use super::TempRepo;

#[test]
fn prioritizes_default_md_first_and_marks_is_default() -> Result<(), Report> {
    let temp = TempRepo::new();
    temp.write_file(".gitlab/merge_request_templates/alpha.md", "Alpha MR");
    temp.write_file(".gitlab/merge_request_templates/Default.md", "Default MR\n/draft");
    temp.write_file(".gitlab/merge_request_templates/zeta.md", "Zeta MR");

    let local = Local(Platform::Gitlab);
    let templates = local.load_pr_templates(&temp.repo)?;

    assert_eq!(templates.len(), 3);
    assert_eq!(templates[0].name, "Default");
    assert!(templates[0].is_default);
    assert_eq!(templates[0].content, "Default MR\n/draft");

    assert_eq!(templates[1].name, "alpha");
    assert!(!templates[1].is_default);
    assert_eq!(templates[1].content, "Alpha MR");

    assert_eq!(templates[2].name, "zeta");
    assert!(!templates[2].is_default);
    assert_eq!(templates[2].content, "Zeta MR");
    Ok(())
}
