pub(crate) const SUPPORTED_REPOS: [(&str, &str, &str); 4] = [
    ("0", "gh", "github"),
    ("1", "gt", "gitea"),
    ("2", "gl", "gitlab"),
    ("3", "cb", "codeberg"),
];

#[derive(PartialEq, Clone)]
pub enum Platform {
    Github,
    Gitea,
    Gitlab,
    Codeberg,
}

impl Platform {
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Github => "github",
            Platform::Gitea => "gitea",
            Platform::Gitlab => "gitlab",
            Platform::Codeberg => "codeberg"
        }
    }
    
    pub fn max_repo_depth(&self) -> usize {
        match self {
            Platform::Github |
            Platform::Codeberg |
            Platform::Gitea => 1,
            Platform::Gitlab => usize::MAX,
        }
    }
}