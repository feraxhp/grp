use crate::common::users::structs::User;


pub trait Search {
    fn search(&self, path: &str) -> Option<User>;
}
