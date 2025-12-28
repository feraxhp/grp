pub(crate) const SUPPORTED_REPOS: [(&str, &str, &str); 5] = [
    ("0", "gh", "github"),
    ("1", "gt", "gitea"),
    ("2", "gl", "gitlab"),
    ("3", "cb", "codeberg"),
    ("4", "fg", "forgejo"),
];

#[derive(PartialEq, Clone)]
pub enum Platform {
    Github,
    Gitea,
    Gitlab,
    Codeberg,
    Forgejo,
}

impl Platform {
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Github => "github",
            Platform::Gitea => "gitea",
            Platform::Gitlab => "gitlab",
            Platform::Codeberg => "codeberg",
            Platform::Forgejo => "forgejo",
        }
    }
    
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