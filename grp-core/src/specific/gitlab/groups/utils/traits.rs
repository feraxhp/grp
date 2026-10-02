use crate::structs::User;


pub trait Search {
    fn search(&self, path: &str) -> Option<User>;
}
