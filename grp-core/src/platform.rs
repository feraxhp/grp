/// Current list of suported repos
pub const SUPPORTED_REPOS: [(&str, &str, &str); 5] = [
    ("0", "gh", "github"),
    ("1", "gt", "gitea"),
    ("2", "gl", "gitlab"),
    ("3", "cb", "codeberg"),
    ("4", "fg", "forgejo"),
];

/// # Platform
/// is the main part of grp, this struct allows 
/// you to interact with any platform, just by 
/// giving a configuration
/// 
/// ~~~
/// use grp_core::Platform;
/// 
/// let platform = Platform::matches("github");
/// 
/// assert!(platform.unwrap() == Platform::Github);
/// ~~~
/// 
#[derive(PartialEq, Clone)]
pub enum Platform {
    Github,
    Gitea,
    Gitlab,
    Codeberg,
    Forgejo,
}

impl Platform {
    /// return the &str name for the repo.
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Github => "github",
            Platform::Gitea => "gitea",
            Platform::Gitlab => "gitlab",
            Platform::Codeberg => "codeberg",
            Platform::Forgejo => "forgejo",
        }
    }
    
    /// Max path depth for a repo
    pub fn max_repo_depth(&self) -> usize {
        match self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Forgejo |
            Platform::Gitea => 1,
            Platform::Gitlab => usize::MAX,
        }
    }
}