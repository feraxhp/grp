use std::iter::once;

use grp_core::structs::Issue;

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
    title: String,
}

impl Show for Vec<Issue> {
    type Context = ();
    
    fn to_string_iter(&self, _: Self::Context) -> impl Iterator<Item = String> + '_ {
        if self.is_empty() { return None.into_iter().flatten(); }
        
        let mut max_id   = 0;
        let mut max_name = 0;
        
        let second: Vec<_> = self.into_iter().map(|issue| {
            let id = format!("#{}", issue.number);
            let len = id.len();
            let id = Cell { text: id.as_link(&issue.url), length: len };
            let author = Cell { text: issue.author.to_link(), length: issue.author.name.len() };
            
            max_id = max_id.max(id.length);
            max_name = max_name.max(author.length);
            
            Row { id, author, title: issue.title.to_owned() }
        }).collect();
        
        let header = format!("{:<max_id$}  {:<max_name$}  TITLE", "ID", "AUTHOR");
        
        let body = second.into_iter().map(move |r| {
             format!("{}  {}  {}",
                 r.id.text.padding(max_id - r.id.length),
                 r.author.text.padding(max_name - r.author.length),
                 r.title,
             )
        });
        
        Some(once(header).chain(body)).into_iter().flatten()
    }
}
