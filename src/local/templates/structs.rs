

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrTemplate {
    pub name: String,
    pub is_default: bool,
    pub content: String,
}

impl PrTemplate {
    pub fn new(name: impl Into<String>, is_default: bool, content: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_default,
            content: content.into(),
        }
    }
}
