use crate::local::git::structs::Action;


impl Action {
    pub fn as_str(&self) -> &str {
        match self {
            Action::Push =>  "push",
            Action::Pull =>  "pull",
            Action::Clone(_) => "clone",
            Action::SetRemote(_, _) => "set remote"
        }
    }
}
