use git2::{Error, Repository};

pub(crate) fn get_branch_name(repo: &Repository) -> Result<String, Error> {
    let head = repo.head()?;
    Ok(head.shorthand().unwrap_or("").to_string())
}