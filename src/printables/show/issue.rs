use std::iter::once;

use grp_core::structs::Issue;
use color_print::cformat;
use grp_core::Config;

use super::super::stylizer::Stylizer;
use super::super::FormatTools;
use super::Show;


struct Cell {
    text: String,
    length: usize,
}

struct Row {
    id: Cell,
    author: Cell,
    reference: Cell,
    title: String,
}

impl Show for Vec<Issue> {
    type Context = Config;
    
    fn to_string_iter(&self, config: Self::Context) -> impl Iterator<Item = String> + '_ {
        if self.is_empty() { return None.into_iter().flatten(); }
        
        let mut max_id   = 2;
        let mut max_name = 6;
        let mut max_ref = 9;
        
        let second: Vec<_> = self.into_iter().map(|issue| {
            let id = format!("#{}", issue.number);
            let len = id.len();
            let id = Cell { text: id.as_link(&issue.url), length: len };
            
            let author = match issue.author.name.as_str() {
                u if u == &config.user => cformat!("<green>{}</>", issue.author.to_link()),
                _ => issue.author.to_link()
            };
            let author = Cell { text: author, length: issue.author.name.len() };
            
            let reference = format!("{}:{}", config.pconf, issue.reference);
            let len = reference.len();
            let reference = Cell { text: reference, length: len };
            
            max_id = max_id.max(id.length);
            max_name = max_name.max(author.length);
            max_ref = max_ref.max(reference.length);
            
            Row { id, author, title: issue.title.to_owned(), reference }
        }).collect();
        
        let header = format!("{:<max_id$}  {:<max_name$}  {:<max_ref$}  TITLE", "ID", "AUTHOR", "REFERENCE");
        
        let body = second.into_iter().map(move |r| {
             format!("{}  {}  {}  {}",
                 r.id.text.padding(max_id - r.id.length),
                 r.author.text.padding(max_name - r.author.length),
                 r.reference.text.padding(max_ref - r.reference.length),
                 r.title,
             )
        });
        
        Some(once(header).chain(body)).into_iter().flatten()
    }
}
