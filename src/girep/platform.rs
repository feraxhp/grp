pub(crate) const SUPPORTED_REPOS: [(&str, &str, &str); 3] = [
    ("0", "gh", "github"),
    ("1", "gt", "gitea"),
    ("2", "gl", "gitlab"),
];

#[derive(PartialEq, Clone)]
pub(crate) enum Platform {
    Github,
    Gitea,
    Gitlab,
}

impl Platform {
    pub fn name(&self) -> &'static str {
        match self {
            Platform::Github => "github",
            Platform::Gitea => "gitea",
            Platform::Gitlab => "gitlab",
        }
    }
}