use grp_core::structs::User;

use super::Show;

impl Show for Vec<User> {
    type Context = ();
    
    fn to_string_iter(&self, _: Self::Context) -> impl Iterator<Item = String> + '_ {
        let mut sorted: Vec<String> = self.iter().map(|user| {
            match &user.path {
                Some(p) => p.clone(),
                None => user.name.clone(),
            }
        }).collect();
        sorted.sort();
        
        (!sorted.is_empty()).then(|| {
            let header = "ORG_NAME".to_string();
            std::iter::once(header).chain(sorted)
        })
        .into_iter()
        .flatten()
    }
}
