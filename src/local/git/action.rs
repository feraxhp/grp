use std::fmt::Display;

use crate::local::git::structs::Action;


impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let write = match self {
            Action::Push =>  "push",
            Action::Pull =>  "pull",
            Action::Fetch => "fetch",
            Action::Clone => "clone",
            Action::SetRemote(_, _) => "set remote"
        };
        
        write!(f, "{}", write)
    }
}
