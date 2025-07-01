
use crate::local::git::options::Methods;


impl Methods {
    pub(super) fn branch_ref(force: &str, branch: &String) -> String {
        format!("{}refs/heads/{branch}:refs/heads/{branch}", force, branch = branch)
    }
    pub(super) fn tag_ref(force: &str, tag: &String) -> String {
        format!("{}refs/tags/{tag}:refs/tags/{tag}", force, tag = tag)
    }
    pub(super) fn tag_ref_formated(force: &str, tag: &String) -> String {
        format!("{}{tag}:{tag}", force, tag = tag)
    }
}
