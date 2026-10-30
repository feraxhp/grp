use grp_core::structs::Issue;

use super::super::stylizer::Stylizer;
use super::super::FormatTools;
use super::Show;


impl Show for Vec<Issue> {
    type Context = ();
    
    fn to_string_iter(&self, _: Self::Context) -> impl Iterator<Item = String> + '_ {
        let (max_number, max_autor) = self.iter().fold((2, 6), |(number, author), issue| {
            (
                number.max(issue.number.to_string().len() + 1),
                author.max(issue.author.name.to_string().len()),
            )
        });
    
        (!self.is_empty())
            .then(move || {
                let header = format!(
                    "{:<max_number$}  {:<max_autor$}  {}",
                    "ID", "AUTHOR", "TITLE",
                );
    
                let body = self.iter().map(move |issue| {
                    format!(
                        "{:<max_number$}  {:<max_autor$}  {}",
                        format!("#{}", issue.number).as_link(&issue.url),
                        issue.author.to_link(),
                        issue.title,
                    )
                });
                
                std::iter::once(header).chain(body)
            })
            .into_iter()
            .flatten()
    }
}
