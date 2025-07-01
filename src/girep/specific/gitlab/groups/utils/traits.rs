use crate::girep::common::users::structs::User;


pub trait Search {
    fn search(&self, path: &str) -> Option<User>;
}
