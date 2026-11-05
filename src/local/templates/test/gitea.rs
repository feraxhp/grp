use grp_core::Platform;
use mynox::Report;

use crate::local::structs::Local;
use super::TempRepo;

#[test]
fn loads_root_or_gitea_single_template_and_ignores_docs() -> Result<(), Report> {
    let temp = TempRepo::new();
    temp.write_file("docs/PULL_REQUEST_TEMPLATE.md", "Docs not used by Gitea");
    temp.write_file(".gitea/PULL_REQUEST_TEMPLATE.md", "---\nname: Gitea Default\nabout: PR\n---\nGitea Body");

    let local = Local(Platform::Gitea);
    let templates = local.load_pr_templates(&temp.repo)?;

    assert_eq!(templates.len(), 1);
    assert_eq!(templates[0].name, "Gitea Default");
    assert!(templates[0].is_default);
    assert_eq!(templates[0].content, "Gitea Body");
    Ok(())
}
